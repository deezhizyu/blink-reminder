use gpui::{Pixels, Point, Size, point, px, size};

pub const OVERLAY_SIZE: Size<Pixels> = size(px(440.0), px(80.0));

pub fn overlay_origin(
    container: Size<Pixels>,
    overlay: Size<Pixels>,
    center: (f32, f32),
) -> Point<Pixels> {
    point(
        axis_origin(container.width, overlay.width, center.0),
        axis_origin(container.height, overlay.height, center.1),
    )
}

pub fn center_from_pointer(
    pointer: Point<Pixels>,
    container: Size<Pixels>,
    overlay: Size<Pixels>,
) -> (f32, f32) {
    let pointer_center = (pointer.x / container.width, pointer.y / container.height);
    let origin = overlay_origin(container, overlay, pointer_center);
    (
        (origin.x + overlay.width / 2.0) / container.width,
        (origin.y + overlay.height / 2.0) / container.height,
    )
}

fn axis_origin(container: Pixels, overlay: Pixels, center: f32) -> Pixels {
    (container * center - overlay / 2.0)
        .min(container - overlay)
        .max(px(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTAINER: Size<Pixels> = size(px(1000.0), px(500.0));
    const OVERLAY: Size<Pixels> = size(px(100.0), px(50.0));

    #[test]
    fn center_position_centers_the_overlay() {
        let origin = overlay_origin(CONTAINER, OVERLAY, (0.5, 0.5));

        assert_eq!(origin, point(px(450.0), px(225.0)));
    }

    #[test]
    fn corner_positions_keep_the_overlay_inside() {
        assert_eq!(
            overlay_origin(CONTAINER, OVERLAY, (0.0, 0.0)),
            point(px(0.0), px(0.0))
        );
        assert_eq!(
            overlay_origin(CONTAINER, OVERLAY, (1.0, 1.0)),
            point(px(900.0), px(450.0))
        );
    }

    #[test]
    fn pointer_center_is_clamped_to_where_the_overlay_fits() {
        let center = center_from_pointer(point(px(1000.0), px(0.0)), CONTAINER, OVERLAY);

        assert_eq!(center, (0.95, 0.05));
    }
}
