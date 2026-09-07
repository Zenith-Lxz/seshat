use gpui::{App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, Render, Window};
use language::language_settings::all_language_settings;
use project::project_settings::ProjectSettings;
use settings::{
    Settings, SettingsStore, SoftWrap, ThemeAppearanceMode, ThemeName, ThemeSelection,
    update_settings_file,
};
use theme_settings::ThemeSettings;
use ui::{ButtonStyle, Divider, Label, LabelSize, prelude::*};
use vim_mode_setting::VimModeSetting;
use workspace::{ModalView, with_active_or_new_workspace};

pub fn init(cx: &mut App) {
    cx.on_action(|_: &zed_actions::OpenSettings, cx| {
        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |_, cx| Preferences::new(cx));
        });
    });
}

struct Preferences {
    focus: FocusHandle,
}

impl Preferences {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<SettingsStore>(|_, cx| cx.notify())
            .detach();
        Self {
            focus: cx.focus_handle(),
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

fn row(
    label: &'static str,
    description: &'static str,
    control: impl IntoElement,
) -> impl IntoElement {
    h_flex()
        .justify_between()
        .gap_6()
        .w_full()
        .child(
            v_flex().gap_1().child(Label::new(label)).child(
                Label::new(description)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            ),
        )
        .child(control)
}

fn toggle(
    id: &'static str,
    enabled: bool,
    change: impl Fn(bool, &mut App) + 'static,
) -> impl IntoElement {
    Button::new(id, if enabled { "On" } else { "Off" })
        .style(if enabled {
            ButtonStyle::Filled
        } else {
            ButtonStyle::Outlined
        })
        .width(rems(3.5))
        .on_click(move |_, _, cx| change(!enabled, cx))
}

impl Render for Preferences {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = ThemeSettings::get_global(cx);
        let font_size = f32::from(theme.buffer_font_size(cx));
        let mode = match &theme.theme {
            theme_settings::ThemeSelection::Dynamic { mode, .. } => *mode,
            theme_settings::ThemeSelection::Static(_) => {
                if cx.theme().appearance().is_light() {
                    ThemeAppearanceMode::Light
                } else {
                    ThemeAppearanceMode::Dark
                }
            }
        };
        let vim_enabled = VimModeSetting::get_global(cx).0;
        let wrap = all_language_settings(None, cx).defaults.soft_wrap != SoftWrap::None;
        let git_enabled = ProjectSettings::get_global(cx).git.enabled.status;
        let themes = h_flex().gap_1().children(
            [
                ("theme-system", "System", ThemeAppearanceMode::System),
                ("theme-light", "Light", ThemeAppearanceMode::Light),
                ("theme-dark", "Dark", ThemeAppearanceMode::Dark),
            ]
            .into_iter()
            .map(move |(id, label, choice)| {
                Button::new(id, label)
                    .style(if mode == choice {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Subtle
                    })
                    .on_click(move |_, _, cx| {
                        update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                            settings.theme.theme = Some(ThemeSelection::Dynamic {
                                mode: choice,
                                light: ThemeName("Seshat Light".into()),
                                dark: ThemeName("Seshat Dark".into()),
                            });
                        });
                    })
            }),
        );
        let font_control = h_flex()
            .gap_2()
            .child(
                Button::new("font-smaller", "−")
                    .style(ButtonStyle::Subtle)
                    .disabled(font_size <= 10.)
                    .on_click(move |_, _, cx| {
                        update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                            settings.theme.buffer_font_size =
                                Some((font_size - 1.).max(10.).into());
                        });
                    }),
            )
            .child(Label::new(format!("{font_size:.0} px")).size(LabelSize::Small))
            .child(
                Button::new("font-larger", "+")
                    .style(ButtonStyle::Subtle)
                    .disabled(font_size >= 32.)
                    .on_click(move |_, _, cx| {
                        update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                            settings.theme.buffer_font_size =
                                Some((font_size + 1.).min(32.).into());
                        });
                    }),
            );
        v_flex()
            .id("seshat-preferences")
            .key_context("SeshatPreferences")
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::dismiss))
            .on_action(
                cx.listener(|_, _: &workspace::CloseActiveItem, _, cx| cx.emit(DismissEvent)),
            )
            .w(rems(35.))
            .p_6()
            .gap_5()
            .rounded_lg()
            .bg(cx.theme().colors().editor_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Settings").size(LabelSize::Large))
                    .child(
                        Button::new("close-settings", "Done")
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    ),
            )
            .child(row(
                "Appearance",
                "Follow your Mac or choose a theme.",
                themes,
            ))
            .child(row(
                "Text size",
                "Size of text in the editor.",
                font_control,
            ))
            .child(Divider::horizontal())
            .child(row(
                "Vim",
                "Normal, insert and visual modes.",
                toggle("vim-mode", vim_enabled, |value, cx| {
                    update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                        settings.vim_mode = Some(value)
                    });
                }),
            ))
            .child(row(
                "Wrap long lines",
                "Keep long lines inside the editor.",
                toggle("line-wrap", wrap, |value, cx| {
                    update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                        settings.project.all_languages.defaults.soft_wrap = Some(if value {
                            SoftWrap::EditorWidth
                        } else {
                            SoftWrap::None
                        })
                    });
                }),
            ))
            .child(row(
                "Git",
                "Changes, history and manual commits.",
                toggle("git-integration", git_enabled, |value, cx| {
                    update_settings_file(<dyn fs::Fs>::global(cx), cx, move |settings, _| {
                        settings
                            .git
                            .get_or_insert_default()
                            .enabled
                            .get_or_insert_default()
                            .disable_git = Some(!value);
                    });
                }),
            ))
            .child(Divider::horizontal())
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("keyboard-shortcuts", "Keyboard Shortcuts")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, window, cx| {
                                cx.emit(DismissEvent);
                                window.dispatch_action(Box::new(zed_actions::OpenKeymap), cx);
                            })),
                    )
                    .child(
                        Button::new("settings-file", "Settings File")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, window, cx| {
                                cx.emit(DismissEvent);
                                window.dispatch_action(Box::new(zed_actions::OpenSettingsFile), cx);
                            })),
                    ),
            )
    }
}
impl ModalView for Preferences {}
impl EventEmitter<DismissEvent> for Preferences {}
impl Focusable for Preferences {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
