use gpui::{
    App, IntoElement, ParentElement as _, Pixels, RenderOnce, Size, Styled as _, Window, black,
    div, size,
};

use crate::{
    components::{ChangeHandler, pointer_surface::pointer_surface},
    placement::{OVERLAY_SIZE, center_from_pointer, overlay_origin},
    theme,
};

const OVERLAY_PREVIEW_OPACITY: f32 = 0.8;

#[derive(IntoElement)]
pub struct PositionPicker {
    width: Pixels,
    display_size: Size<Pixels>,
    center: (f32, f32),
    on_change: ChangeHandler<(f32, f32)>,
}

impl PositionPicker {
    pub fn new(
        width: Pixels,
        display_size: Size<Pixels>,
        center: (f32, f32),
        on_change: impl Fn((f32, f32), &mut App) + 'static,
    ) -> Self {
        Self {
            width,
            display_size,
            center,
            on_change: Box::new(on_change),
        }
    }
}

impl RenderOnce for PositionPicker {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let picker_size = Size {
            width: self.width,
            height: self.width * (self.display_size.height / self.display_size.width),
        };
        let preview_scale = picker_size.width / self.display_size.width;
        let preview_size = size(
            OVERLAY_SIZE.width * preview_scale,
            OVERLAY_SIZE.height * preview_scale,
        );
        let preview_origin = overlay_origin(picker_size, preview_size, self.center);
        let on_change = self.on_change;
        let surface = pointer_surface("position-picker", window, cx, move |position, size, cx| {
            on_change(center_from_pointer(position, size, preview_size), cx);
        });

        surface
            .w(picker_size.width)
            .h(picker_size.height)
            .rounded_md()
            .bg(theme::picker_background())
            .cursor_move()
            .child(
                div()
                    .absolute()
                    .left(preview_origin.x)
                    .top(preview_origin.y)
                    .w(preview_size.width)
                    .h(preview_size.height)
                    .rounded_sm()
                    .bg(black())
                    .opacity(OVERLAY_PREVIEW_OPACITY),
            )
    }
}
