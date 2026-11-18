use std::time::Duration;

use gpui::{App, Global, Task};

use crate::{
    overlay,
    settings::{self, Settings},
};

struct Scheduler {
    _reminder_loop: Task<()>,
}

impl Global for Scheduler {}

pub fn start(cx: &mut App) {
    restart(cx);
    settings::observe_field(
        cx,
        |settings| settings.interval_minutes,
        |_, cx| restart(cx),
    );
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
