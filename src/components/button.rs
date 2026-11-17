use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, MouseButton, ParentElement as _,
    RenderOnce, SharedString, Styled as _, Window, div,
};

use crate::theme;

#[derive(IntoElement)]
pub struct Button {
    id: SharedString,
    label: SharedString,
    on_click: Box<dyn Fn(&mut App)>,
}

impl Button {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_click = self.on_click;

        div()
            .id(ElementId::from(self.id))
            .flex()
            .justify_center()
            .items_center()
            .h_9()
            .rounded_md()
            .cursor_pointer()
            .bg(theme::control_background())
            .hover(|style| style.bg(theme::control_background_hover()))
            .on_mouse_down(MouseButton::Left, move |_, _, cx| on_click(cx))
            .child(self.label)
    }
}
