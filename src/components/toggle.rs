use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, MouseButton, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{components::ChangeHandler, theme};

const SWITCH_WIDTH: f32 = 40.0;
const SWITCH_HEIGHT: f32 = 22.0;
const KNOB_MARGIN: f32 = 3.0;

#[derive(IntoElement)]
pub struct Toggle {
    id: SharedString,
    label: SharedString,
    enabled: bool,
    on_change: ChangeHandler<bool>,
}

impl Toggle {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        enabled: bool,
        on_change: impl Fn(bool, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            enabled,
            on_change: Box::new(on_change),
        }
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let enabled = self.enabled;
        let on_change = self.on_change;
        let knob_diameter = SWITCH_HEIGHT - 2.0 * KNOB_MARGIN;
        let knob_left = if enabled {
            SWITCH_WIDTH - knob_diameter - KNOB_MARGIN
        } else {
            KNOB_MARGIN
        };

        div()
            .id(ElementId::from(self.id))
            .flex()
            .justify_between()
            .items_center()
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, move |_, _, cx| on_change(!enabled, cx))
            .child(self.label)
            .child(
                div()
                    .relative()
                    .w(px(SWITCH_WIDTH))
                    .h(px(SWITCH_HEIGHT))
                    .rounded_full()
                    .bg(if enabled {
                        theme::accent()
                    } else {
                        theme::control_background()
                    })
                    .child(
                        div()
                            .absolute()
                            .top(px(KNOB_MARGIN))
                            .left(px(knob_left))
                            .size(px(knob_diameter))
                            .rounded_full()
                            .bg(theme::text()),
                    ),
            )
    }
}
