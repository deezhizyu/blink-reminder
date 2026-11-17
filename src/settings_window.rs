use gpui::{
    App, Bounds, Context, IntoElement, Render, Size, TitlebarOptions, Window, WindowBounds,
    WindowHandle, WindowOptions, div, prelude::*, px, size,
};

use crate::{
    components::{position_picker::PositionPicker, slider::Slider, toggle::Toggle},
    diagnostics::report_error,
    settings::{self, DISPLAY_SECONDS_RANGE, INTERVAL_MINUTES_RANGE, OPACITY_RANGE, Settings},
    theme,
};

const WINDOW_SIZE: Size<gpui::Pixels> = size(px(340.0), px(560.0));
const FALLBACK_DISPLAY_SIZE: Size<gpui::Pixels> = size(px(1920.0), px(1080.0));

struct SettingsWindow;

impl Render for SettingsWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current = cx.global::<Settings>().clone();
        let display_size = cx
            .primary_display()
            .map_or(FALLBACK_DISPLAY_SIZE, |display| display.bounds().size);

        div()
            .size_full()
            .bg(theme::window_background())
            .text_color(theme::text())
            .p_6()
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
                        display_size,
                        (current.position_x, current.position_y),
                        |(position_x, position_y), cx| {
                            settings::update(cx, |settings| {
                                settings.position_x = position_x;
                                settings.position_y = position_y;
                            })
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
            WINDOW_SIZE,
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
    cx.open_window(options, |_, cx| cx.new(|_| SettingsWindow))
}
