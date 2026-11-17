use std::{f32::consts::PI, time::Duration};

pub const FADE_DURATION: Duration = Duration::from_millis(200);
pub const BLINK_PERIOD: Duration = Duration::from_millis(1600);
const BLINK_SHARE_OF_PERIOD: f32 = 0.15;
const BLINK_CLOSED_SCALE: f32 = 0.1;

pub fn overlay_opacity(
    elapsed: Duration,
    display_duration: Duration,
    fade_duration: Duration,
    maximum_opacity: f32,
) -> f32 {
    if elapsed >= display_duration {
        return 0.0;
    }
    let fade_progress = |distance: Duration| {
        if fade_duration.is_zero() {
            1.0
        } else {
            (distance.as_secs_f32() / fade_duration.as_secs_f32()).min(1.0)
        }
    };
    let fade_in = fade_progress(elapsed);
    let fade_out = fade_progress(display_duration - elapsed);
    maximum_opacity * fade_in.min(fade_out)
}

pub fn blink_scale(progress: f32) -> f32 {
    if progress >= BLINK_SHARE_OF_PERIOD {
        return 1.0;
    }
    let closing = (progress / BLINK_SHARE_OF_PERIOD * PI).sin();
    1.0 - (1.0 - BLINK_CLOSED_SCALE) * closing
}

#[cfg(test)]
mod tests {
    use super::*;

    const DISPLAY: Duration = Duration::from_secs(5);
    const FADE: Duration = Duration::from_millis(200);

    fn opacity_at(elapsed: Duration) -> f32 {
        overlay_opacity(elapsed, DISPLAY, FADE, 0.25)
    }

    #[test]
    fn opacity_starts_and_ends_at_zero() {
        assert_eq!(opacity_at(Duration::ZERO), 0.0);
        assert_eq!(opacity_at(DISPLAY), 0.0);
    }

    #[test]
    fn opacity_is_halfway_in_the_middle_of_each_fade() {
        assert_eq!(opacity_at(Duration::from_millis(100)), 0.125);
        assert_eq!(opacity_at(DISPLAY - Duration::from_millis(100)), 0.125);
    }

    #[test]
    fn opacity_holds_at_maximum_between_fades() {
        assert_eq!(opacity_at(Duration::from_secs(2)), 0.25);
    }

    #[test]
    fn opacity_without_fade_jumps_to_maximum() {
        let opacity = overlay_opacity(Duration::ZERO, DISPLAY, Duration::ZERO, 0.25);

        assert_eq!(opacity, 0.25);
    }

    #[test]
    fn blink_is_open_outside_the_blink_window_and_closed_inside_it() {
        assert_eq!(blink_scale(0.5), 1.0);
        assert!((blink_scale(BLINK_SHARE_OF_PERIOD / 2.0) - BLINK_CLOSED_SCALE).abs() < 1e-6);
    }
}
