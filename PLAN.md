# Plan

## Context
The repo `deezhizyu/blink-reminder` is empty. The user wants a Windows tray app in Rust. At a set interval it shows a small overlay that reminds them to blink. The overlay never takes focus and passes clicks through. Answers from the user:
- **Platform:** Windows only.
- **UI framework:** gpui (crates.io `gpui` 0.2.2, from the official zed-industries repo, 513k downloads).
- **Overlay:** click-through.
- **Monitor:** the overlay shows on the display that has the cursor.

## Order of work (from the user)
1. `AGENTS.md`: the main instructions file. It covers the project overview, stack, layout, build/test commands, and the git branch rule. It has a **critical rule**: vet every dependency before adding it. The vetting covers crates.io owners, the repository matching the official source, downloads and age, recent ownership changes, RustSec advisories (`cargo audit`), typosquats, and build scripts / proc-macros doing network or file access. Prefer crates already in the tree and the std library.
2. `CODE_STYLE.md`: no duplication (search before you write, extract shared logic), idiomatic Rust naming with no short names, no comments unless the code is truly quirky, simple code first, compute a value once and reuse it, simple *and* optimized, no code debt from day one. Write commit messages as Conventional Commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, ...).
3. `CLAUDE.md`: the two lines `@AGENTS.md` and `@CODE_STYLE.md`.
4. `PLAN.md`: this plan, copied into the repo.
5. Build the app.

## Dependencies (each one vetted per the AGENTS.md rule before it goes into `Cargo.toml`)
| Crate | Why |
|---|---|
| `gpui` | Windows, rendering, animation, timers |
| `tray-icon` (tauri-apps, 45M downloads) | Tray icon and menu. gpui has no tray. It works on gpui's Win32 message loop. |
| `serde` + `toml` | The settings file |
| `windows` 0.61 (Microsoft) | Click-through window styles and finding the cursor's monitor. 0.61 is the version gpui uses, so there is no duplicate. |
| `raw-window-handle` | Gets the HWND from the gpui `Window`. Same version as gpui. |
| `anyhow` | Error handling |
| `futures` | Channel from the tray menu handler into the gpui executor |
| `winresource` (build-dep) | Embeds `icon.ico` as the exe icon. The tray uses it too, through `Icon::from_resource`. |

The config path comes from the std `APPDATA` env var, so no `dirs` crate is needed.

## Layout
This is a standard Cargo binary crate on **edition 2024**, the latest Rust edition. It uses the current module style, unchanged since 2018 (`src/components.rs` declares `src/components/*.rs`, with no `mod.rs`), unit tests inline under `#[cfg(test)] mod tests`, `build.rs` at the root, and `assets/` for embedded files.
```
Cargo.toml, build.rs (winresource)
assets/eye.svg, assets/icon.ico
src/main.rs            #![windows_subsystem = "windows"], App::run, wiring
src/settings.rs        Settings struct + defaults + clamp + load/save (TOML in %APPDATA%\blink-reminder), impl gpui::Global
src/scheduler.rs       Reminder loop task; restarts when the Settings global changes (cx.observe_global)
src/overlay.rs         Overlay view + open/close; pure fn `overlay_opacity(elapsed, show_duration, fade, max_opacity)` + tests
src/settings_window.rs Dark settings view; opens the window or focuses the open one
src/components.rs                  pub mod slider; pub mod toggle; pub mod position_picker;
src/components/slider.rs           One reusable slider (interval, duration, opacity)
src/components/toggle.rs           Animations on/off
src/components/position_picker.rs  Light-gray screen-ratio box with a draggable overlay rect
src/tray.rs            Tray icon, menu "Settings" / "Quit"; MenuEvent handler -> channel -> gpui task
src/platform.rs        Win32 only: make HWND click-through (WS_EX_LAYERED|WS_EX_TRANSPARENT|WS_EX_NOACTIVATE|WS_EX_TOOLWINDOW), display under the cursor (GetCursorPos/MonitorFromPoint mapped to the gpui display)
```

## Key behavior
- **Settings** (all saved right away on change, no Save button): `interval_minutes` (default 20, range 1–120), `show_seconds` (default 5, range 1–30), `opacity` (default 0.25, range 0.1–1.0), `animations_enabled` (default true), `position` as a normalized overlay center (default 0.5, 0.5). Loading clamps values, and a missing or broken file falls back to the defaults.
- **Overlay window:** `WindowKind::PopUp`, `focus: false`, `titlebar: None`, `is_movable: false`, transparent background. Size is fixed at about 440×80 logical px. Bounds come from the cursor display's bounds and the normalized position, clamped inside the screen. After the window opens, `platform` applies the click-through styles.
- **Overlay view:** a black rounded div with the whole thing at the current opacity. On the left is `eye.svg`, which blinks with a repeating `with_animation` that scales it on Y. On the right is the text **"Blink slowly and fully — rest your eyes"**. Opacity follows the envelope: a 200 ms fade in, a hold at `opacity`, then a 200 ms fade out. The fade is 0 when animations are off. The view calls `request_animation_frame` only while a fade is running.
- **Scheduler:** the loop is wait `interval`, show the overlay, wait `show_seconds` plus the fades, then close it. Any settings change restarts the countdown.
- **Settings window:** dark (#1e1e1e-style background, light text), a single column with the position picker on top, then the three sliders and the toggle. The picker box uses the aspect ratio of the cursor display. Dragging the rect updates `position`.
- **Tray:** "Settings" opens or focuses the settings window. "Quit" calls `cx.quit()`. The app stays alive with no windows open.

## Skipped (not asked; can add later)
Launch at login, single-instance guard, "show now" preview button, macOS/Linux support.

## Risks to check first
- Read gpui 0.2.2's Windows window code. Confirm that `PopUp` + `focus: false` does not activate the window and stays topmost, that the app does not quit when its last window closes, and that `Window: HasWindowHandle` holds.
- `WS_EX_LAYERED` may clash with gpui's DirectComposition transparency. If it does, fall back to `WS_EX_TRANSPARENT` with a `WM_NCHITTEST` override, or a region-based approach.

## Verification
- Here (Linux container): `rustup target add x86_64-pc-windows-gnu` and `cargo check --target x86_64-pc-windows-gnu`. Unit tests cover pure logic: the opacity envelope, settings clamp and TOML round-trip, and position-to-bounds math.
- Add a small GitHub Actions workflow on `windows-latest` that runs `cargo clippy -- -D warnings` and `cargo test`. It is the real Windows build check.
- Manual on Windows (user): the tray icon shows with both menu items. Set the interval to 1 min and confirm that the overlay fades in and out, does not steal focus while you type, passes clicks through, and shows on the monitor with the cursor. Each setting should apply live and persist after a restart.
- Commit and push to `claude/clever-sagan-s74l3p`.

## Implementation notes
- gpui on Windows quits the app when its last window closes. `main.rs` opens one invisible window (`show: false`) so the tray app keeps running after the overlay or settings window closes.
- The overlay opens with `show: true`, `focus: false`, `kind: PopUp`. `platform::make_click_through` then adds `WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW` and moves the window to `HWND_TOPMOST` without activating it.
- `platform::cursor_display_index` returns the index of the monitor under the cursor. gpui numbers its displays in `EnumDisplayMonitors` order, so the same index selects the gpui display.
- Pure logic lives in `animation.rs`, `placement.rs` and `settings.rs`, with unit tests. Those tests were run against a stub of gpui's geometry types in a Linux container. The full test suite runs in CI on Windows.
- Not verified on real Windows yet: no focus steal, true click-through, and topmost behaviour of the overlay with gpui's DirectComposition window.
