use std::rc::Rc;

use gpui::{
    App, Bounds, Div, ElementId, InteractiveElement as _, MouseButton, ParentElement as _, Pixels,
    Point, SharedString, Size, Stateful, Styled as _, Window, canvas, div,
};

pub fn pointer_surface(
    id: impl Into<SharedString>,
    window: &mut Window,
    cx: &mut App,
    on_pointer: impl Fn(Point<Pixels>, Size<Pixels>, &mut App) + 'static,
) -> Stateful<Div> {
    let id: SharedString = id.into();
    let tracked_bounds =
        window.use_keyed_state(ElementId::from((id.clone(), 0usize)), cx, |_, _| {
            Bounds::<Pixels>::default()
        });
    let bounds_tracker = canvas(|bounds, _, _| bounds, {
        let tracked_bounds = tracked_bounds.clone();
        move |_, bounds, _, cx| {
            tracked_bounds.update(cx, |stored, _| *stored = bounds);
        }
    })
    .absolute()
    .size_full();
    let report_pointer = Rc::new(move |position: Point<Pixels>, cx: &mut App| {
        let bounds = *tracked_bounds.read(cx);
        on_pointer(position - bounds.origin, bounds.size, cx);
    });

    div()
        .id(ElementId::from(id))
        .relative()
        .child(bounds_tracker)
        .on_mouse_down(MouseButton::Left, {
            let report_pointer = report_pointer.clone();
            move |event, _, cx| report_pointer(event.position, cx)
        })
        .on_mouse_move(move |event, _, cx| {
            if event.dragging() {
                report_pointer(event.position, cx);
            }
        })
}
