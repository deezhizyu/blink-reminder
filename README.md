<p align="center">
  <img src="assets/eye.svg" width="96" alt="Blink Reminder eye icon">
</p>

<h1 align="center">Blink Reminder</h1>

<p align="center">
  A tiny Windows tray app that reminds you to blink slowly and fully.
</p>

<p align="center">
  <a href="https://github.com/deezhizyu/blink-reminder/releases/latest"><img src="https://img.shields.io/github/v/release/deezhizyu/blink-reminder" alt="Latest release"></a>
  <a href="https://github.com/deezhizyu/blink-reminder/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/deezhizyu/blink-reminder/ci.yml?branch=main&label=CI" alt="CI status"></a>
  <img src="https://img.shields.io/badge/platform-Windows-0078d4" alt="Platform: Windows">
  <img src="https://img.shields.io/badge/made%20with-Rust-dea584" alt="Made with Rust">
</p>

When you stare at a screen, you blink less, and your eyes get dry and tired. Blink Reminder shows a quiet overlay every few minutes with one message: **"Blink slowly and fully — rest your eyes"**. It does not take focus, and it does not interrupt what you are doing.

## Features

- **Stays out of the way.** The overlay never takes focus, and mouse clicks pass through it, so you can keep typing and clicking.
- **Gentle by default.** A small black overlay with an animated blinking eye, at 25% opacity, that fades in and out over 200 ms.
- **Follows you.** The reminder appears on the display that has your mouse cursor.
- **Your position.** Drag the overlay anywhere on a preview of your screen, or type the exact position in percent.
- **Lives in the tray.** Open **Settings** or **Quit** from the tray icon. There is no main window.
- **Starts with Windows.** You can turn this on with one toggle.
- **Live settings.** Changes apply right away and are saved automatically.
- **Test reminder.** Shows the overlay right away, so you can preview your settings. Your timer keeps running.

## Install

1. Download `blink-reminder-windows-x64.exe` from the [latest release](https://github.com/deezhizyu/blink-reminder/releases/latest).
2. Run it. The eye icon appears in the system tray.
3. Click the tray icon and choose **Settings** to adjust it.

Each release also has a `.sha256` file, so you can check the download.

## Settings

| Setting | Default | Range |
|---|---|---|
| Remind every | 20 min | 1–120 min |
| Show for | 5 s | 1–30 s |
| Opacity | 25% | 10–100% |
| Animations | On | On / Off |
| Start with Windows | Off | On / Off |
| Position on screen | Center | Anywhere |

Settings are stored in `%APPDATA%\blink-reminder\settings.toml`.

## Build from source

You need [Rust](https://rustup.rs) and Windows.

```bash
cargo build --release
```

The app is written to `target\release\blink-reminder.exe`.

## Thanks to

Blink Reminder is built on these crates:

- [gpui](https://crates.io/crates/gpui): the UI framework from [Zed](https://zed.dev), used for windows, rendering, and animation
- [tray-icon](https://crates.io/crates/tray-icon): the tray icon and menu
- [windows](https://crates.io/crates/windows): Win32 APIs from Microsoft
- [raw-window-handle](https://crates.io/crates/raw-window-handle): access to native window handles
- [serde](https://crates.io/crates/serde) and [toml](https://crates.io/crates/toml): the settings file
- [anyhow](https://crates.io/crates/anyhow): error handling
- [futures](https://crates.io/crates/futures): async channels
- [winresource](https://crates.io/crates/winresource): the executable icon

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
