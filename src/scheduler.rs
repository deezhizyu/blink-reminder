use std::time::Duration;

use gpui::{App, Global, Task};

use crate::{overlay, settings::Settings};

struct Scheduler {
    _reminder_loop: Task<()>,
}

impl Global for Scheduler {}

pub fn start(cx: &mut App) {
    restart(cx);
    let mut interval_minutes = cx.global::<Settings>().interval_minutes;
    cx.observe_global::<Settings>(move |cx| {
        let changed_interval_minutes = cx.global::<Settings>().interval_minutes;
        if changed_interval_minutes != interval_minutes {
            interval_minutes = changed_interval_minutes;
            restart(cx);
        }
    })
    .detach();
}

fn restart(cx: &mut App) {
    let interval = Duration::from_secs_f32(cx.global::<Settings>().interval_minutes * 60.0);
    let reminder_loop = cx.spawn(async move |cx| {
        loop {
            cx.background_executor().timer(interval).await;
            if cx.update(overlay::show).is_err() {
                break;
            }
        }
    });
    cx.set_global(Scheduler {
        _reminder_loop: reminder_loop,
    });
}
