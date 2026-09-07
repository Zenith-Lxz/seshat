mod preview;

use editor::{Editor, EditorSettings};
use gpui::{Context, Entity, EventEmitter, Render, WeakEntity, Window};
use search::BufferSearchBar;
use settings::{Settings, SettingsStore};
use ui::{ButtonStyle, IconButton, IconSize, Tooltip, prelude::*};
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, item::ItemHandle,
};

pub struct QuickActionBar {
    active_item: Option<Box<dyn ItemHandle>>,
    buffer_search_bar: Entity<BufferSearchBar>,
    workspace: WeakEntity<Workspace>,
    show: bool,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: Entity<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_global::<SettingsStore>(|this, cx| {
            this.show = EditorSettings::get_global(cx).toolbar.quick_actions;
            cx.emit(ToolbarItemEvent::ChangeLocation(this.location()));
            cx.notify();
        })
        .detach();
        Self {
            active_item: None,
            buffer_search_bar,
            workspace: workspace.weak_handle(),
            show: EditorSettings::get_global(cx).toolbar.quick_actions,
        }
    }

    fn location(&self) -> ToolbarItemLocation {
        if self.show
            && self
                .active_item
                .as_ref()
                .and_then(|item| item.downcast::<Editor>())
                .is_some()
        {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for QuickActionBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_bar = self.buffer_search_bar.clone();
        h_flex()
            .gap_1()
            .children(self.render_preview_button(cx))
            .child(
                IconButton::new("find-in-file", search::SEARCH_ICON)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(!search_bar.read(cx).is_dismissed())
                    .tooltip(|_, cx| {
                        Tooltip::for_action("搜索当前文件", &search::seshat_search::CurrentFile, cx)
                    })
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(Box::new(search::seshat_search::CurrentFile), cx);
                    }),
            )
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}
impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.active_item = active_pane_item.map(ItemHandle::boxed_clone);
        self.location()
    }
}
