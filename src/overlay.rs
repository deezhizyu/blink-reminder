use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use gpui::{
    Animation, AnimationExt as _, App, Bounds, Context, IntoElement, Render, Transformation,
    Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions, black, div,
    prelude::*, rgb, size, svg,
};

use crate::{
    animation::{BLINK_PERIOD, FADE_DURATION, blink_scale, overlay_opacity},
    diagnostics::report_error,
    placement::{OVERLAY_SIZE, overlay_origin},
    platform,
    settings::Settings,
};

const MESSAGE: &str = "Blink slowly and fully — rest your eyes";
const EYE_ICON_PATH: &str = "eye.svg";

struct Overlay {
    opened_at: Instant,
    display_duration: Duration,
    fade_duration: Duration,
    maximum_opacity: f32,
    animations_enabled: bool,
}

impl Render for Overlay {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if !self.fade_duration.is_zero() {
            window.request_animation_frame();
        }
        let opacity = overlay_opacity(
            self.opened_at.elapsed(),
            self.display_duration,
            self.fade_duration,
            self.maximum_opacity,
        );
        let eye = svg()
            .path(EYE_ICON_PATH)
            .size_10()
            .text_color(rgb(0xffffff));
        let eye = if self.animations_enabled {
            eye.with_animation(
                "blink",
                Animation::new(BLINK_PERIOD).repeat(),
                |eye, progress| {
                    eye.with_transformation(Transformation::scale(size(1.0, blink_scale(progress))))
                },
            )
            .into_any_element()
        } else {
            eye.into_any_element()
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .gap_4()
            .px_6()
            .rounded_2xl()
            .bg(black())
            .opacity(opacity)
            .text_color(rgb(0xffffff))
            .text_lg()
            .child(eye)
            .child(MESSAGE)
    }
}

pub fn show(cx: &mut App) {
    if let Err(error) = open_window(cx) {
        report_error("failed to show the reminder", &error);
    }
}

fn open_window(cx: &mut App) -> Result<()> {
    let settings = cx.global::<Settings>().clone();
    let display_duration = Duration::from_secs_f32(settings.display_seconds);
    let fade_duration = if settings.animations_enabled {
        FADE_DURATION
    } else {
        Duration::ZERO
    };
    let display = platform::cursor_display_index()
        .and_then(|index| cx.displays().into_iter().nth(index))
        .or_else(|| cx.primary_display())
        .context("no display available")?;
    let display_bounds = display.bounds();
    let origin = display_bounds.origin
        + overlay_origin(
            display_bounds.size,
            OVERLAY_SIZE,
            (settings.position_x, settings.position_y),
        );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::new(origin, OVERLAY_SIZE))),
        display_id: Some(display.id()),
        titlebar: None,
        window_background: WindowBackgroundAppearance::Transparent,
        kind: WindowKind::PopUp,
        focus: false,
        show: true,
        is_movable: false,
        is_resizable: false,
        is_minimizable: false,
        ..Default::default()
    };
    let window = cx.open_window(options, |_, cx| {
        cx.new(|_| Overlay {
            opened_at: Instant::now(),
            display_duration,
            fade_duration,
            maximum_opacity: settings.opacity,
            animations_enabled: settings.animations_enabled,
        })
    })?;
    window.update(cx, |_, window, cx| -> Result<()> {
        platform::make_click_through(window)?;
        window
            .spawn(cx, async move |cx| {
                cx.background_executor().timer(display_duration).await;
                let _ = cx.update(|window, _| window.remove_window());
            })
            .detach();
        Ok(())
    })?
}
