use gpui::{
    App, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{
    components::{ChangeHandler, pointer_surface::pointer_surface},
    theme,
};

const TRACK_HEIGHT: f32 = 6.0;
const THUMB_DIAMETER: f32 = 16.0;
const HIT_AREA_HEIGHT: f32 = 24.0;

#[derive(IntoElement)]
pub struct Slider {
    id: SharedString,
    label: SharedString,
    value_label: SharedString,
    value: f32,
    range: (f32, f32),
    step: f32,
    on_change: ChangeHandler<f32>,
}

impl Slider {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        value_label: impl Into<SharedString>,
        value: f32,
        range: (f32, f32),
        step: f32,
        on_change: impl Fn(f32, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value_label: value_label.into(),
            value,
            range,
            step,
            on_change: Box::new(on_change),
        }
    }
}

impl RenderOnce for Slider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (minimum, maximum) = self.range;
        let fraction = (self.value - minimum) / (maximum - minimum);
        let step = self.step;
        let on_change = self.on_change;
        let surface = pointer_surface(self.id, window, cx, move |position, size, cx| {
            let pointer_fraction = (position.x / size.width).clamp(0.0, 1.0);
            let raw_value = minimum + pointer_fraction * (maximum - minimum);
            on_change((raw_value / step).round() * step, cx);
        });

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div().flex().justify_between().child(self.label).child(
                    div()
                        .text_color(theme::muted_text())
                        .child(self.value_label),
                ),
            )
            .child(
                surface
                    .h(px(HIT_AREA_HEIGHT))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .w_full()
                            .h(px(TRACK_HEIGHT))
                            .rounded_full()
                            .bg(theme::control_background())
                            .child(
                                div()
                                    .h_full()
                                    .w(gpui::relative(fraction))
                                    .rounded_full()
                                    .bg(theme::accent()),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .left(gpui::relative(fraction))
                            .top(px((HIT_AREA_HEIGHT - THUMB_DIAMETER) / 2.0))
                            .ml(px(-THUMB_DIAMETER * fraction))
                            .size(px(THUMB_DIAMETER))
                            .rounded_full()
                            .bg(theme::text()),
                    ),
            )
    }
}
