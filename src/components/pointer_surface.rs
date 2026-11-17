use std::rc::Rc;

use gpui::{
    App, Bounds, Div, ElementId, InteractiveElement as _, MouseButton, MouseMoveEvent,
    MouseUpEvent, ParentElement as _, Pixels, Point, SharedString, Size, Stateful, Styled as _,
    Window, canvas, div,
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
    let dragging = window.use_keyed_state(ElementId::from((id.clone(), 1usize)), cx, |_, _| false);
    let report_pointer = Rc::new({
        let tracked_bounds = tracked_bounds.clone();
        move |position: Point<Pixels>, cx: &mut App| {
            let bounds = *tracked_bounds.read(cx);
            on_pointer(position - bounds.origin, bounds.size, cx);
        }
    });
    let drag_listeners = canvas(|bounds, _, _| bounds, {
        let dragging = dragging.clone();
        let report_pointer = report_pointer.clone();
        move |_, bounds, window, cx| {
            tracked_bounds.update(cx, |stored, _| *stored = bounds);
            window.on_mouse_event({
                let dragging = dragging.clone();
                move |event: &MouseMoveEvent, phase, _, cx| {
                    if phase.capture() || !*dragging.read(cx) {
                        return;
                    }
                    if event.dragging() {
                        report_pointer(event.position, cx);
                    } else {
                        dragging.update(cx, |dragging, _| *dragging = false);
                    }
                }
            });
            window.on_mouse_event(move |_: &MouseUpEvent, phase, _, cx| {
                if phase.bubble() {
                    dragging.update(cx, |dragging, _| *dragging = false);
                }
            });
        }
    })
    .absolute()
    .size_full();

    div()
        .id(ElementId::from(id))
        .relative()
        .child(drag_listeners)
        .on_mouse_down(MouseButton::Left, move |event, _, cx| {
            dragging.update(cx, |dragging, _| *dragging = true);
            report_pointer(event.position, cx);
        })
}
