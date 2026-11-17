#![cfg_attr(not(test), windows_subsystem = "windows")]

mod animation;
mod components;
mod diagnostics;
mod overlay;
mod placement;
mod platform;
mod scheduler;
mod settings;
mod settings_window;
mod theme;
mod tray;

use std::borrow::Cow;

use anyhow::Result;
use gpui::{
    App, AppContext as _, Application, AssetSource, Empty, SharedString, WindowKind, WindowOptions,
};

use crate::{diagnostics::report_error, settings::Settings};

struct EmbeddedAssets;

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            "eye.svg" => Some(Cow::Borrowed(include_bytes!("../assets/eye.svg"))),
            _ => None,
        })
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

fn main() {
    Application::new()
        .with_assets(EmbeddedAssets)
        .run(|cx: &mut App| {
            cx.set_global(Settings::load());
            keep_application_alive(cx);
            if let Err(error) = tray::install(cx) {
                report_error("failed to create the tray icon", &error);
                cx.quit();
                return;
            }
            scheduler::start(cx);
        });
}

// gpui on Windows quits when its last window closes, so an invisible window keeps the tray app running.
fn keep_application_alive(cx: &mut App) {
    let options = WindowOptions {
        kind: WindowKind::PopUp,
        show: false,
        focus: false,
        ..Default::default()
    };
    if let Err(error) = cx.open_window(options, |_, cx| cx.new(|_| Empty)) {
        report_error("failed to create the keep-alive window", &error);
    }
}
