use std::{fs, io, path::PathBuf, time::Duration};

use gpui::{App, Global, Task};
use serde::{Deserialize, Serialize};

use crate::diagnostics::report_error;

const APPLICATION_DIRECTORY_NAME: &str = "blink-reminder";
const SETTINGS_FILE_NAME: &str = "settings.toml";
const SAVE_DEBOUNCE: Duration = Duration::from_millis(400);

pub const INTERVAL_MINUTES_RANGE: (f32, f32) = (1.0, 120.0);
pub const DISPLAY_SECONDS_RANGE: (f32, f32) = (1.0, 30.0);
pub const OPACITY_RANGE: (f32, f32) = (0.1, 1.0);
pub const POSITION_RANGE: (f32, f32) = (0.0, 1.0);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub interval_minutes: f32,
    pub display_seconds: f32,
    pub opacity: f32,
    pub animations_enabled: bool,
    pub position_x: f32,
    pub position_y: f32,
}

impl Global for Settings {}

struct PendingSave {
    _task: Task<()>,
}

impl Global for PendingSave {}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_minutes: 20.0,
            display_seconds: 5.0,
            opacity: 0.25,
            animations_enabled: true,
            position_x: 0.5,
            position_y: 0.5,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        settings_file_path()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|contents| toml::from_str::<Self>(&contents).ok())
            .unwrap_or_default()
            .clamped()
    }

    pub fn save(&self) -> io::Result<()> {
        let path = settings_file_path()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "APPDATA is not set"))?;
        if let Some(directory) = path.parent() {
            fs::create_dir_all(directory)?;
        }
        let contents = toml::to_string_pretty(self).map_err(io::Error::other)?;
        fs::write(path, contents)
    }

    pub fn clamped(self) -> Self {
        Self {
            interval_minutes: clamp_to_range(self.interval_minutes, INTERVAL_MINUTES_RANGE),
            display_seconds: clamp_to_range(self.display_seconds, DISPLAY_SECONDS_RANGE),
            opacity: clamp_to_range(self.opacity, OPACITY_RANGE),
            position_x: clamp_to_range(self.position_x, POSITION_RANGE),
            position_y: clamp_to_range(self.position_y, POSITION_RANGE),
            ..self
        }
    }
}

pub fn update(cx: &mut App, change: impl FnOnce(&mut Settings)) {
    let mut updated = cx.global::<Settings>().clone();
    change(&mut updated);
    let updated = updated.clamped();
    if updated == *cx.global::<Settings>() {
        return;
    }
    cx.set_global(updated);
    schedule_save(cx);
}

pub fn save_on_quit(cx: &mut App) {
    cx.on_app_quit(|cx| {
        let settings = cx.global::<Settings>().clone();
        async move { save_and_report(&settings) }
    })
    .detach();
}

fn schedule_save(cx: &mut App) {
    let task = cx.spawn(async move |cx| {
        cx.background_executor().timer(SAVE_DEBOUNCE).await;
        if let Ok(settings) = cx.read_global(|settings: &Settings, _| settings.clone()) {
            cx.background_executor()
                .spawn(async move { save_and_report(&settings) })
                .await;
        }
    });
    cx.set_global(PendingSave { _task: task });
}

fn save_and_report(settings: &Settings) {
    if let Err(error) = settings.save() {
        report_error("failed to save settings", &error);
    }
}

fn clamp_to_range(value: f32, (minimum, maximum): (f32, f32)) -> f32 {
    if value.is_nan() {
        minimum
    } else {
        value.clamp(minimum, maximum)
    }
}

fn settings_file_path() -> Option<PathBuf> {
    let application_data = std::env::var_os("APPDATA")?;
    Some(
        PathBuf::from(application_data)
            .join(APPLICATION_DIRECTORY_NAME)
            .join(SETTINGS_FILE_NAME),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamped_limits_every_value_to_its_range() {
        let settings = Settings {
            interval_minutes: 500.0,
            display_seconds: 0.0,
            opacity: 7.0,
            position_x: -1.0,
            position_y: f32::NAN,
            ..Settings::default()
        }
        .clamped();

        assert_eq!(settings.interval_minutes, 120.0);
        assert_eq!(settings.display_seconds, 1.0);
        assert_eq!(settings.opacity, 1.0);
        assert_eq!(settings.position_x, 0.0);
        assert_eq!(settings.position_y, 0.0);
    }

    #[test]
    fn toml_round_trip_keeps_values() {
        let settings = Settings {
            interval_minutes: 7.0,
            animations_enabled: false,
            ..Settings::default()
        };

        let contents = toml::to_string_pretty(&settings).unwrap();

        assert_eq!(toml::from_str::<Settings>(&contents).unwrap(), settings);
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let settings: Settings = toml::from_str("opacity = 0.5").unwrap();

        assert_eq!(settings.opacity, 0.5);
        assert_eq!(
            settings.interval_minutes,
            Settings::default().interval_minutes
        );
    }
}
