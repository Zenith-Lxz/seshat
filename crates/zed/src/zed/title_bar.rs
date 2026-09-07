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
        let title = self
            .workspace
            .upgrade()
            .and_then(|workspace| {
                workspace
                    .read(cx)
                    .project()
                    .read(cx)
                    .visible_worktrees(cx)
                    .next()
                    .map(|worktree| worktree.read(cx).root_name().to_string())
            })
            .unwrap_or_else(|| "Seshat".into());
        let controls = h_flex()
            .gap_2()
            .h_full()
            .child(
                IconButton::new("open-file", IconName::FolderOpen)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .tooltip(|_, cx| {
                        Tooltip::for_action("Open File or Folder", &workspace::Open::default(), cx)
                    })
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::Open::default()), cx)
                    }),
            )
            .child(Label::new(title).size(LabelSize::Small).color(Color::Muted))
            .into_any_element();
        let sidebar = IconButton::new("toggle-files", IconName::FileTree)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(|_, cx| {
                Tooltip::for_action("Files", &zed_actions::project_panel::ToggleFocus, cx)
            })
            .on_click(|_, window, cx| {
                window.dispatch_action(Box::new(zed_actions::project_panel::ToggleFocus), cx)
            })
            .into_any_element();
        self.platform
            .update(cx, |platform, _| platform.set_children([controls, sidebar]));
        self.platform.clone()
    }
}
