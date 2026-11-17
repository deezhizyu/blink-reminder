# AGENTS.md

## Critical rule: vet every dependency

Before adding or upgrading any dependency (crate, build dependency, or tool), check it for supply-chain attacks and malware, and confirm it can be trusted at all. Do this before it goes into `Cargo.toml`. Do not skip it for "small" or "well-known" crates.

Checklist:

- The crates.io page links to the official repository, and that repository links back to the crate.
- The owners and publishers are the expected organization or maintainers. Look for recent ownership changes.
- The crate name is not a typosquat of a popular crate.
- Download count, age, and release history look healthy. Be suspicious of a brand-new crate or a sudden release after a long silence.
- No open RustSec advisory (`cargo audit`). Run `cargo deny check` too if it is configured.
- `build.rs` files and procedural macros do not touch the network, run unexpected processes, or read files outside the build.
- The published crate contents match the repository (`cargo vendor` or the crates.io source view when in doubt).
- The dependency tree it brings in is acceptable. Prefer features that keep it small (`default-features = false`).

Prefer, in order: the standard library, a crate already in the tree, a new vetted crate. Pin versions through `Cargo.lock`, which is committed. Record any new dependency and the reason for it in the dependency table below.

## GitHub Actions

The same rule applies to third-party actions. Pin each one to a full commit SHA with the version in a trailing comment, and vet it before use. `actions/checkout` is official. Vetted third-party actions:

| Action | Pinned | Vetting |
|---|---|---|
| `Swatinem/rust-cache` | `d1527c8f67ef9af357290475d4af4b0fab8f12e0` (v2.9.3) | Single long-time maintainer, 100+ commits, runs on `node24` from a committed `dist/`. Source and `dist/` inspected for network calls and process execution: only the GitHub cache service and opt-in cache backends, and `sudo purge` on macOS. `dist/` was not rebuilt and diffed against `src/`. |

## Project

Blink Reminder is a Windows tray app written in Rust. It reminds the user to blink more often and fully, once per configurable interval (default 20 minutes).

- The app lives in the system tray. The tray menu has **Settings** and **Quit**.
- The reminder is a small overlay on top of all windows. It never takes focus, never disturbs the window under it, and passes mouse clicks through.
- Overlay look: black rounded background, an animated blinking-eye icon on the left, and the text "Blink slowly and fully — rest your eyes" on the right. The whole overlay is shown at 25% opacity by default.
- The overlay fades in and out smoothly over 200 ms.
- The overlay appears on the display that has the mouse cursor.
- The Settings window is dark-themed and simple. Settings apply live. Changes are saved to disk after a short pause (debounced) and on quit. A **Test reminder** button shows the overlay without touching the timers.

Settings:

| Setting | Default | Range |
|---|---|---|
| Reminder interval (minutes) | 20 | 1–120 |
| Display time (seconds) | 5 | 1–30 |
| Overlay opacity | 0.25 | 0.1–1.0 |
| Animations enabled | on | on/off |
| Overlay position (normalized center on the screen) | 0.5, 0.5 | 0–1 |

The position is edited in a light-gray box that has the aspect ratio of the screen, with a draggable overlay rectangle inside it.

Platform: Windows only. UI framework: [gpui](https://crates.io/crates/gpui). Tray: `tray-icon`. Settings file: TOML in `%APPDATA%\blink-reminder`.

See `PLAN.md` for the design and the build order.

## Dependencies

| Crate | Purpose |
|---|---|
| `gpui` | Windows, rendering, animation, timers |
| `tray-icon` | Tray icon and menu |
| `serde`, `toml` | Settings file |
| `anyhow` | Error handling (`Result` with context) |
| `futures` | Channel that carries tray menu events into the gpui executor |
| `windows` | Win32 calls: click-through styles, display under the cursor |
| `raw-window-handle` | Window handle from the gpui window |
| `winresource` (build) | Embeds the executable icon |

## Layout

Standard Cargo binary crate, edition 2024. Modules use the file-per-module style (`src/components.rs` plus `src/components/*.rs`, no `mod.rs`). Unit tests live inline in `#[cfg(test)] mod tests`. Assets are in `assets/`.

Module list and responsibilities are in `PLAN.md`.

## Commands

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo audit
```

The app targets Windows only. In a Linux container, install `gcc-mingw-w64-x86-64`, add the target (`rustup target add x86_64-pc-windows-gnu`), and check with `cargo clippy --target x86_64-pc-windows-gnu --all-targets -- -D warnings`. Tests cannot run there (the test binary needs Windows system DLLs that Wine lacks), so `cargo test` runs in the GitHub Actions workflow on `windows-latest`.

## Workflow

- Follow `CODE_STYLE.md` for all code and commit messages.
- Search the codebase for an existing implementation before writing a new feature or helper.
- Keep `PLAN.md` and this file correct when the design or the dependencies change.
- Release: bump `version` in `Cargo.toml`, then push a tag `vX.Y.Z` that matches it. `.github/workflows/release.yml` builds and publishes the GitHub release.
- Develop on the branch given for the session. Do not push to other branches, and do not open a pull request unless asked.
