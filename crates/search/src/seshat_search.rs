use crate::SearchOptions;
use editor::{Editor, SelectionEffects};
use futures::StreamExt;
use gpui::{
    App, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity, actions, uniform_list,
};
use language::Buffer;
use project::{Project, search::SearchResult};
use std::ops::Range;
use text::{Anchor, Point, ToPoint};
use ui::{HighlightedLabel, Toggleable, Tooltip, prelude::*};
use util::{ResultExt, paths::PathMatcher};
use workspace::{
    Panel, Workspace,
    dock::{DockPosition, PanelEvent},
};

actions!(seshat_search, [CurrentFile, AllFiles]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    CurrentFile,
    AllFiles,
    OpenFiles,
    OpenLogs,
}

impl Scope {
    fn label(self) -> &'static str {
        match self {
            Self::CurrentFile => "当前文件",
            Self::AllFiles => "全部文件",
            Self::OpenFiles => "已打开文件",
            Self::OpenLogs => "已打开日志",
        }
    }
}

fn is_log_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase();
    name.ends_with(".log")
        || name.rsplit_once(".log.").is_some_and(|(_, suffix)| {
            suffix.starts_with(|character: char| character.is_ascii_digit())
        })
}

struct MatchGroup {
    buffer: Entity<Buffer>,
    ranges: Vec<Range<Anchor>>,
}
#[derive(Clone, Copy)]
enum ResultRow {
    File(usize),
    Match(usize, usize),
}

pub struct SeshatSearchPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus: FocusHandle,
    query_editor: Entity<Editor>,
    current_buffer: Option<Entity<Buffer>>,
    scope: Scope,
    options: SearchOptions,
    groups: Vec<MatchGroup>,
    rows: Vec<ResultRow>,
    pending: Option<Task<()>>,
    searching: bool,
    searched: bool,
    generation: u64,
    limit_reached: bool,
    error: Option<String>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &CurrentFile, window, cx| {
            SeshatSearchPanel::show(workspace, Scope::CurrentFile, window, cx);
        });
        workspace.register_action(|workspace, _: &AllFiles, window, cx| {
            SeshatSearchPanel::show(workspace, Scope::AllFiles, window, cx);
        });
    })
    .detach();
}

impl SeshatSearchPanel {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("搜索", window, cx);
            editor
        });
        cx.subscribe(
            workspace.project(),
            |panel, _, event: &project::Event, cx| {
                if let project::Event::WorktreeRemoved(worktree_id) = event {
                    panel.pending.take();
                    panel.generation += 1;
                    panel.searching = false;
                    panel.groups.retain(|group| {
                        group
                            .buffer
                            .read(cx)
                            .file()
                            .is_none_or(|file| file.worktree_id(cx) != *worktree_id)
                    });
                    panel.rows = panel
                        .groups
                        .iter()
                        .enumerate()
                        .flat_map(|(index, group)| {
                            std::iter::once(ResultRow::File(index)).chain(
                                (0..group.ranges.len())
                                    .map(move |match_index| ResultRow::Match(index, match_index)),
                            )
                        })
                        .collect();
                    cx.notify();
                }
            },
        )
        .detach();
        Self {
            workspace: workspace.weak_handle(),
            project: workspace.project().clone(),
            focus: cx.focus_handle(),
            query_editor,
            current_buffer: None,
            scope: Scope::CurrentFile,
            options: SearchOptions::NONE,
            groups: Vec::new(),
            rows: Vec::new(),
            pending: None,
            searching: false,
            searched: false,
            generation: 0,
            limit_reached: false,
            error: None,
        }
    }

    pub fn show(
        workspace: &mut Workspace,
        scope: Scope,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let current_buffer = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton());
        let panel = workspace.panel::<Self>(cx).unwrap_or_else(|| {
            let panel = cx.new(|cx| Self::new(workspace, window, cx));
            workspace.add_panel(panel.clone(), window, cx);
            panel
        });
        panel.update(cx, |panel, cx| {
            panel.current_buffer = current_buffer;
            panel.scope = scope;
            panel.query_editor.update(cx, |editor, cx| {
                editor.select_all(&editor::actions::SelectAll, window, cx)
            });
            panel.query_editor.focus_handle(cx).focus(window, cx);
            cx.notify();
        });
        workspace.reveal_panel::<Self>(window, cx);
    }

    fn dismiss(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        let workspace = self.workspace.clone();
        window.defer(cx, move |window, cx| {
            workspace
                .update(cx, |workspace, cx| {
                    workspace.close_panel::<Self>(window, cx);
                    if let Some(item) = workspace.active_item(cx) {
                        item.item_focus_handle(cx).focus(window, cx);
                    }
                })
                .log_err();
        });
    }

    fn buffers_for_scope(&self, cx: &App) -> Option<Vec<Entity<Buffer>>> {
        if self.scope == Scope::AllFiles {
            return None;
        }
        if self.scope == Scope::CurrentFile {
            return Some(self.current_buffer.iter().cloned().collect());
        }
        let mut buffers = Vec::new();
        if let Some(workspace) = self.workspace.upgrade() {
            for item in workspace.read(cx).items(cx) {
                if let Some(buffer) = item
                    .act_as::<Editor>(cx)
                    .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton())
                    && !buffers.contains(&buffer)
                    && (self.scope != Scope::OpenLogs
                        || buffer
                            .read(cx)
                            .file()
                            .is_some_and(|file| is_log_name(file.file_name(cx))))
                {
                    buffers.push(buffer);
                }
            }
        }
        Some(buffers)
    }

    fn start_search(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        self.current_buffer = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).active_item(cx))
            .and_then(|item| item.act_as::<Editor>(cx))
            .and_then(|editor| editor.read(cx).buffer().read(cx).as_singleton());
        self.pending.take();
        self.generation += 1;
        let generation = self.generation;
        self.groups.clear();
        self.rows.clear();
        self.error = None;
        self.limit_reached = false;
        self.searching = false;
        self.searched = true;
        let query = self.query_editor.read(cx).text(cx);
        if query.is_empty() {
            self.searched = false;
            cx.notify();
            return;
        }
        let query = match self.options.build_query(
            query,
            PathMatcher::default(),
            PathMatcher::default(),
            false,
            self.buffers_for_scope(cx),
        ) {
            Ok(query) => query,
            Err(error) => {
                self.error = Some(error.to_string());
                cx.notify();
                return;
            }
        };
        let results = self
            .project
            .update(cx, |project, cx| project.search(query, cx));
        self.searching = true;
        self.pending = Some(cx.spawn(async move |panel, cx| {
            let mut chunks = std::pin::pin!(results.rx.clone().ready_chunks(64));
            while let Some(results_chunk) = chunks.next().await {
                if panel
                    .update(cx, |panel, cx| {
                        if panel.generation != generation {
                            return;
                        }
                        for result in results_chunk {
                            match result {
                                SearchResult::Buffer { buffer, ranges } => {
                                    if ranges.is_empty() {
                                        continue;
                                    }
                                    let index = panel.groups.len();
                                    panel.rows.push(ResultRow::File(index));
                                    panel.rows.extend(
                                        (0..ranges.len())
                                            .map(|range| ResultRow::Match(index, range)),
                                    );
                                    panel.groups.push(MatchGroup { buffer, ranges });
                                }
                                SearchResult::LimitReached => panel.limit_reached = true,
                                _ => {}
                            }
                        }
                        cx.notify();
                    })
                    .is_err()
                {
                    return;
                }
            }
            panel
                .update(cx, |panel, cx| {
                    if panel.generation == generation {
                        panel.searching = false;
                        cx.notify();
                    }
                })
                .log_err();
            drop(results);
        }));
        cx.notify();
    }

    fn open_match(
        &mut self,
        group: usize,
        match_index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(group) = self.groups.get(group) else {
            return;
        };
        let Some(range) = group.ranges.get(match_index).cloned() else {
            return;
        };
        let buffer = group.buffer.clone();
        self.workspace
            .update(cx, |workspace, cx| {
                let editor = workspace.open_project_item::<Editor>(
                    workspace.active_pane().clone(),
                    buffer.clone(),
                    true,
                    true,
                    true,
                    false,
                    window,
                    cx,
                );
                editor.update(cx, |editor, cx| {
                    editor.clear_content_view(window, cx);
                    let snapshot = buffer.read(cx).snapshot();
                    let range = range.start.to_point(&snapshot)..range.end.to_point(&snapshot);
                    editor.change_selections(
                        SelectionEffects::default(),
                        window,
                        cx,
                        |selections| selections.select_ranges([range]),
                    );
                });
            })
            .log_err();
    }

    fn render_rows(&self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<AnyElement> {
        range
            .filter_map(|index| {
                let row = *self.rows.get(index)?;
                let (group_index, match_index) = match row {
                    ResultRow::File(group) => (group, None),
                    ResultRow::Match(group, index) => (group, Some(index)),
                };
                let group = self.groups.get(group_index)?;
                let snapshot = group.buffer.read(cx).snapshot();
                let name = group
                    .buffer
                    .read(cx)
                    .file()
                    .map(|file| file.full_path(cx).to_string_lossy().into_owned())
                    .unwrap_or_else(|| "未命名".into());
                Some(if let Some(match_index) = match_index {
                    let source_range = group.ranges.get(match_index)?;
                    let start = source_range.start.to_point(&snapshot);
                    let line = snapshot
                        .text_for_range(
                            Point::new(start.row, 0)
                                ..Point::new(start.row, snapshot.line_len(start.row)),
                        )
                        .collect::<String>();
                    let end = source_range.end.to_point(&snapshot);
                    let highlight_end = if end.row == start.row {
                        end.column as usize
                    } else {
                        line.len()
                    };
                    let highlight = line
                        .char_indices()
                        .filter_map(|(offset, _)| {
                            (offset >= start.column as usize && offset < highlight_end)
                                .then_some(offset)
                        })
                        .collect();
                    h_flex()
                        .id(("search-match", index))
                        .w_full()
                        .h_7()
                        .gap_3()
                        .px_4()
                        .cursor_pointer()
                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                        .child(
                            Label::new((start.row + 1).to_string())
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .child(
                            HighlightedLabel::new(line, highlight)
                                .single_line()
                                .size(LabelSize::Small),
                        )
                        .on_click(cx.listener(move |panel, _, window, cx| {
                            panel.open_match(group_index, match_index, window, cx)
                        }))
                        .into_any_element()
                } else {
                    h_flex()
                        .h_7()
                        .px_3()
                        .gap_2()
                        .bg(cx.theme().colors().surface_background)
                        .child(Icon::new(IconName::File).size(IconSize::Small))
                        .child(Label::new(name).single_line().size(LabelSize::Small))
                        .child(
                            Label::new(format!("{} 处", group.ranges.len()))
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .into_any_element()
                })
            })
            .collect()
    }
}

impl Render for SeshatSearchPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count: usize = self.groups.iter().map(|group| group.ranges.len()).sum();
        v_flex()
            .id("seshat-search")
            .size_full()
            .track_focus(&self.focus)
            .key_context("SeshatSearch")
            .bg(cx.theme().colors().editor_background)
            .on_action(cx.listener(Self::start_search))
            .on_action(cx.listener(Self::dismiss))
            .child(
                v_flex()
                    .p_2()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                h_flex()
                                    .flex_1()
                                    .min_w_0()
                                    .p_1()
                                    .rounded_sm()
                                    .bg(cx.theme().colors().element_background)
                                    .child(
                                        Icon::new(IconName::MagnifyingGlass).size(IconSize::Small),
                                    )
                                    .child(self.query_editor.clone())
                                    .child(
                                        IconButton::new("clear-search", IconName::Close)
                                            .icon_size(IconSize::Small)
                                            .tooltip(Tooltip::text("清空搜索"))
                                            .on_click(cx.listener(|panel, _, window, cx| {
                                                panel.query_editor.update(cx, |editor, cx| {
                                                    editor.set_text("", window, cx);
                                                });
                                                panel.start_search(&menu::Confirm, window, cx);
                                                panel
                                                    .query_editor
                                                    .focus_handle(cx)
                                                    .focus(window, cx);
                                            })),
                                    ),
                            )
                            .child(
                                Button::new(
                                    "run-search",
                                    if self.searching { "停止" } else { "搜索" },
                                )
                                .on_click(cx.listener(
                                    |panel, _, window, cx| {
                                        if panel.searching {
                                            panel.pending.take();
                                            panel.generation += 1;
                                            panel.searching = false;
                                            cx.notify();
                                        } else {
                                            panel.start_search(&menu::Confirm, window, cx);
                                        }
                                    },
                                )),
                            )
                            .child(
                                Label::new(format!("{count} 处  {} 个文件", self.groups.len()))
                                    .color(Color::Muted)
                                    .size(LabelSize::Small),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .children(
                                [
                                    Scope::CurrentFile,
                                    Scope::AllFiles,
                                    Scope::OpenFiles,
                                    Scope::OpenLogs,
                                ]
                                .into_iter()
                                .map(|scope| {
                                    Button::new(scope.label(), scope.label())
                                        .toggle_state(self.scope == scope)
                                        .on_click(cx.listener(move |panel, _, window, cx| {
                                            panel.scope = scope;
                                            panel.start_search(&menu::Confirm, window, cx);
                                        }))
                                }),
                            )
                            .children(
                                [
                                    ("Aa", SearchOptions::CASE_SENSITIVE),
                                    ("全词", SearchOptions::WHOLE_WORD),
                                    (".*", SearchOptions::REGEX),
                                ]
                                .into_iter()
                                .map(|(label, option)| {
                                    Button::new(label, label)
                                        .toggle_state(self.options.contains(option))
                                        .on_click(cx.listener(move |panel, _, window, cx| {
                                            panel.options.toggle(option);
                                            panel.start_search(&menu::Confirm, window, cx);
                                        }))
                                }),
                            ),
                    ),
            )
            .when_some(self.error.clone(), |view, error| {
                view.child(Label::new(error).color(Color::Error))
            })
            .when(self.limit_reached, |view| {
                view.child(Label::new("结果达到上限，请缩小搜索范围。").color(Color::Warning))
            })
            .child(if self.rows.is_empty() {
                v_flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .child(Icon::new(IconName::MagnifyingGlass).color(Color::Muted))
                    .child(
                        Label::new(if self.searching {
                            "搜索中…"
                        } else if self.searched {
                            "没有匹配结果"
                        } else {
                            "等待搜索"
                        })
                        .color(Color::Muted),
                    )
                    .child(
                        Label::new("按 Return 搜索，结果会按文件保留在这里。")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .into_any_element()
            } else {
                uniform_list(
                    "search-results",
                    self.rows.len(),
                    cx.processor(|panel, range, _, cx| panel.render_rows(range, cx)),
                )
                .flex_1()
                .size_full()
                .into_any_element()
            })
    }
}
impl Focusable for SeshatSearchPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
impl EventEmitter<PanelEvent> for SeshatSearchPanel {}
impl Panel for SeshatSearchPanel {
    fn persistent_name() -> &'static str {
        "Seshat Search"
    }
    fn panel_key() -> &'static str {
        "SeshatSearch"
    }
    fn position(&self, _: &Window, _: &App) -> DockPosition {
        DockPosition::Bottom
    }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        position == DockPosition::Bottom
    }
    fn set_position(&mut self, _: DockPosition, _: &mut Window, _: &mut Context<Self>) {}
    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(240.)
    }
    fn icon(&self, _: &Window, _: &App) -> Option<IconName> {
        Some(IconName::MagnifyingGlass)
    }
    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("搜索")
    }
    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(CurrentFile)
    }
    fn activation_priority(&self) -> u32 {
        1
    }
    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.query_editor.focus_handle(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, VisualTestContext};
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use workspace::MultiWorkspace;

    #[test]
    fn test_seshat_search_log_names_include_rotated_logs() {
        for name in ["app.log", "APP.LOG", "archive.LOG.1", "app.log.2026-09-07"] {
            assert!(is_log_name(name));
        }
        for name in ["catalog", "app.log.rs", "app.log.md", "app.txt"] {
            assert!(!is_log_name(name));
        }
    }

    #[gpui::test]
    async fn test_seshat_search_scopes_use_open_items_and_unsaved_text(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/fixtures",
            json!({
                "project": {"open.txt": "needle source", "closed.txt": "needle closed"},
                "loose.log": "needle log"
            }),
        )
        .await;
        let project = Project::test(
            fs,
            ["/fixtures/project".as_ref(), "/fixtures/loose.log".as_ref()],
            cx,
        )
        .await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |multi, _| multi.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        for absolute in ["/fixtures/loose.log", "/fixtures/project/open.txt"] {
            workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.open_abs_path(absolute.into(), Default::default(), window, cx)
                })
                .await
                .unwrap();
        }
        let editor = workspace.read_with(cx, |workspace, cx| {
            workspace.active_item_as::<Editor>(cx).unwrap()
        });
        editor.update_in(cx, |editor, window, cx| {
            editor.set_text("needle unsaved needle", window, cx)
        });
        workspace.update_in(cx, |workspace, window, cx| {
            SeshatSearchPanel::show(workspace, Scope::CurrentFile, window, cx)
        });
        let panel = workspace.read_with(cx, |workspace, cx| {
            workspace.panel::<SeshatSearchPanel>(cx).unwrap()
        });
        panel.update_in(cx, |panel, window, cx| {
            panel
                .query_editor
                .update(cx, |editor, cx| editor.set_text("needle", window, cx));
            panel.start_search(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();
        panel.read_with(cx, |panel, _| {
            assert_eq!(panel.groups.len(), 1);
            assert_eq!(panel.groups[0].ranges.len(), 2);
        });
        for (scope, files, matches) in [
            (Scope::AllFiles, 3, 4),
            (Scope::OpenFiles, 2, 3),
            (Scope::OpenLogs, 1, 1),
        ] {
            panel.update_in(cx, |panel, window, cx| {
                panel.scope = scope;
                panel.start_search(&menu::Confirm, window, cx);
            });
            cx.run_until_parked();
            panel.read_with(cx, |panel, _| {
                assert_eq!(panel.groups.len(), files, "scope: {scope:?}");
                assert_eq!(
                    panel
                        .groups
                        .iter()
                        .map(|group| group.ranges.len())
                        .sum::<usize>(),
                    matches
                );
            });
        }
        // A bad query must replace the old result state, and a later valid query must recover.
        panel.update_in(cx, |panel, window, cx| {
            panel.options = SearchOptions::REGEX;
            panel
                .query_editor
                .update(cx, |editor, cx| editor.set_text("[", window, cx));
            panel.start_search(&menu::Confirm, window, cx);
        });
        panel.read_with(cx, |panel, _| {
            assert!(panel.error.is_some());
            assert!(panel.rows.is_empty());
        });
        panel.update_in(cx, |panel, window, cx| {
            panel
                .query_editor
                .update(cx, |editor, cx| editor.set_text("needle", window, cx));
            panel.start_search(&menu::Confirm, window, cx);
            panel
                .query_editor
                .update(cx, |editor, cx| editor.set_text("missing", window, cx));
            panel.start_search(&menu::Confirm, window, cx);
        });
        cx.run_until_parked();
        panel.read_with(cx, |panel, _| {
            assert!(panel.error.is_none());
            assert!(panel.groups.is_empty());
            assert!(!panel.searching);
        });
        cx.dispatch_action(menu::Cancel);
        cx.run_until_parked();
        workspace.read_with(cx, |workspace, cx| {
            assert!(!workspace.bottom_dock().read(cx).is_open());
        });
        cx.update(|window, cx| assert!(editor.focus_handle(cx).is_focused(window)));
    }
}
