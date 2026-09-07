use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription,
    WeakEntity, Window,
};
use platform_title_bar::PlatformTitleBar;
use ui::{
    ButtonStyle, Color, IconButton, IconName, IconSize, Label, LabelSize, Tooltip, h_flex,
    prelude::*,
};
use workspace::Workspace;

pub fn init(cx: &mut App) {
    PlatformTitleBar::init(cx);
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        let title_bar = cx.new(|cx| TitleBar::new(workspace, cx));
        workspace.set_titlebar_item(title_bar.into(), window, cx);
    })
    .detach();
}

struct TitleBar {
    platform: Entity<PlatformTitleBar>,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl TitleBar {
    fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let workspace_handle = workspace.weak_handle();
        let mut subscriptions = vec![cx.observe(workspace.project(), |_, _, cx| cx.notify())];
        if let Some(workspace) = workspace_handle.upgrade() {
            subscriptions.push(cx.observe(&workspace, |_, _, cx| cx.notify()));
        }
        Self {
            platform: cx.new(|cx| PlatformTitleBar::new("seshat-title-bar", cx)),
            workspace: workspace_handle,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (title, directory) = self
            .workspace
            .upgrade()
            .map(|workspace| {
                let workspace = workspace.read(cx);
                if let Some(item) = workspace.active_item(cx) {
                    let absolute = item.project_path(cx).and_then(|path| {
                        workspace
                            .project()
                            .read(cx)
                            .worktree_for_id(path.worktree_id, cx)
                            .map(|tree| tree.read(cx).absolutize(&path.path))
                    });
                    if let Some(absolute) = absolute {
                        let name = absolute
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        let directory = absolute
                            .parent()
                            .map(|parent| parent.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        return (
                            format!("{}{}", name, if item.is_dirty(cx) { " •" } else { "" }),
                            directory,
                        );
                    }
                    return (
                        format!(
                            "{}{}",
                            item.tab_content_text(0, cx),
                            if item.is_dirty(cx) { " •" } else { "" }
                        ),
                        String::new(),
                    );
                }
                ("Seshat".into(), String::new())
            })
            .unwrap_or_else(|| ("Seshat".into(), String::new()));
        let controls = h_flex()
            .gap_2()
            .h_full()
            .child(
                IconButton::new("toggle-files", IconName::FileTree)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("显示侧边栏"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::ToggleLeftDock), cx)
                    }),
            )
            .child(
                IconButton::new("open-file", IconName::File)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("打开文件"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::Open::default()), cx)
                    }),
            )
            .child(
                IconButton::new("open-folder", IconName::FolderOpen)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(Tooltip::text("打开文件夹"))
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::AddFolderToProject), cx)
                    }),
            )
            .child(Label::new(title).size(LabelSize::Small))
            .child(
                Label::new(directory)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any_element();
        self.platform
            .update(cx, |platform, _| platform.set_children([controls]));
        self.platform.clone()
    }
}
