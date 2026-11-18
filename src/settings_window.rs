use gpui::{
    App, Bounds, Context, IntoElement, Pixels, Render, Size, TitlebarOptions, Window, WindowBounds,
    WindowHandle, WindowOptions, div, prelude::*, px, size,
};

use crate::{
    components::{
        button::Button, number_input::NumberInput, position_picker::PositionPicker, slider::Slider,
        toggle::Toggle,
    },
    diagnostics::report_error,
    overlay,
    settings::{self, DISPLAY_SECONDS_RANGE, INTERVAL_MINUTES_RANGE, OPACITY_RANGE, Settings},
    theme,
};

const WINDOW_WIDTH: f32 = 340.0;
const WINDOW_PADDING: f32 = 24.0;
const CONTENT_WIDTH: f32 = WINDOW_WIDTH - 2.0 * WINDOW_PADDING;
const INITIAL_WINDOW_HEIGHT: f32 = 690.0;
const FALLBACK_DISPLAY_SIZE: Size<Pixels> = size(px(1920.0), px(1080.0));

struct SettingsWindow;

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = cx.global::<Settings>().clone();
        let display_size = cx
            .primary_display()
            .map_or(FALLBACK_DISPLAY_SIZE, |display| display.bounds().size);

        let content = div()
            .w_full()
            .p(px(WINDOW_PADDING))
            .flex()
            .flex_col()
            .gap_5()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child("Position on screen")
                    .child(PositionPicker::new(
                        px(CONTENT_WIDTH),
                        display_size,
                        (current.position_x, current.position_y),
                        |(position_x, position_y), cx| {
                            settings::update(cx, |settings| {
                                settings.position_x = position_x;
                                settings.position_y = position_y;
                            })
                        },
                    ))
                    .child(NumberInput::new(
                        "position-x",
                        "Horizontal (%)",
                        current.position_x * 100.0,
                        |percent, cx| {
                            settings::update(cx, |settings| settings.position_x = percent / 100.0)
                        },
                    ))
                    .child(NumberInput::new(
                        "position-y",
                        "Vertical (%)",
                        current.position_y * 100.0,
                        |percent, cx| {
                            settings::update(cx, |settings| settings.position_y = percent / 100.0)
                        },
                    )),
            )
            .child(Slider::new(
                "interval",
                "Remind every",
                format!("{:.0} min", current.interval_minutes),
                current.interval_minutes,
                INTERVAL_MINUTES_RANGE,
                1.0,
                |value, cx| settings::update(cx, |settings| settings.interval_minutes = value),
            ))
            .child(Slider::new(
                "display-seconds",
                "Show for",
                format!("{:.0} s", current.display_seconds),
                current.display_seconds,
                DISPLAY_SECONDS_RANGE,
                1.0,
                |value, cx| settings::update(cx, |settings| settings.display_seconds = value),
            ))
            .child(Slider::new(
                "opacity",
                "Opacity",
                format!("{:.0}%", current.opacity * 100.0),
                current.opacity,
                OPACITY_RANGE,
                0.05,
                |value, cx| settings::update(cx, |settings| settings.opacity = value),
            ))
            .child(Toggle::new(
                "animations",
                "Animations",
                current.animations_enabled,
                |enabled, cx| {
                    settings::update(cx, |settings| settings.animations_enabled = enabled)
                },
            ))
            .child(Toggle::new(
                "launch-at-login",
                "Start with Windows",
                current.launch_at_login,
                |enabled, cx| settings::update(cx, |settings| settings.launch_at_login = enabled),
            ))
            .child(Button::new("test-reminder", "Test reminder", overlay::show));

        div()
            .size_full()
            .bg(theme::window_background())
            .text_color(theme::text())
            .on_children_prepainted(fit_window_to_content)
            .child(content)
    }
}

// The content height depends on the display aspect ratio and the font, so the window is sized after layout.
fn fit_window_to_content(children: Vec<Bounds<Pixels>>, window: &mut Window, _cx: &mut App) {
    let Some(content) = children.first() else {
        return;
    };
    let viewport = window.viewport_size();
    if (content.size.height - viewport.height).abs() >= px(1.0) {
        window.resize(size(viewport.width, content.size.height));
    }
}

pub fn open(cx: &mut App) {
    let existing = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<SettingsWindow>());
    if let Some(existing) = existing
        && existing
            .update(cx, |_, window, _| window.activate_window())
            .is_ok()
    {
        return;
    }
    if let Err(error) = open_new(cx) {
        report_error("failed to open settings", &error);
    }
}

fn open_new(cx: &mut App) -> anyhow::Result<WindowHandle<SettingsWindow>> {
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(WINDOW_WIDTH), px(INITIAL_WINDOW_HEIGHT)),
            cx,
        ))),
        titlebar: Some(TitlebarOptions {
            title: Some("Blink Reminder Settings".into()),
            ..Default::default()
        }),
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    cx.open_window(options, |_, cx| {
        cx.new(|cx| {
            cx.observe_global::<Settings>(|_, cx| cx.notify()).detach();
            SettingsWindow
        })
    })
}
