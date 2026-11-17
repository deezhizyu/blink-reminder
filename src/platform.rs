use std::ffi::c_void;

use anyhow::{Result, anyhow};
use gpui::Window;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::{
    Foundation::{COLORREF, HWND, LPARAM, POINT, RECT, TRUE},
    Graphics::Gdi::{
        EnumDisplayMonitors, HDC, HMONITOR, MONITOR_DEFAULTTONEAREST, MonitorFromPoint,
    },
    UI::WindowsAndMessaging::{
        GWL_EXSTYLE, GetCursorPos, GetWindowLongPtrW, HWND_TOPMOST, LWA_ALPHA, SWP_NOACTIVATE,
        SWP_NOMOVE, SWP_NOSIZE, SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos,
        WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TRANSPARENT,
    },
};

const FULLY_OPAQUE_ALPHA: u8 = 255;

pub fn make_click_through(window: &Window) -> Result<()> {
    let window_handle = window_handle(window)?;
    let added_styles = WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW;
    unsafe {
        let current_styles = GetWindowLongPtrW(window_handle, GWL_EXSTYLE);
        SetWindowLongPtrW(
            window_handle,
            GWL_EXSTYLE,
            current_styles | added_styles.0 as isize,
        );
        SetLayeredWindowAttributes(window_handle, COLORREF(0), FULLY_OPAQUE_ALPHA, LWA_ALPHA)?;
        SetWindowPos(
            window_handle,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
        )?;
    }
    Ok(())
}

pub fn cursor_display_index() -> Option<usize> {
    let mut cursor = POINT::default();
    unsafe { GetCursorPos(&mut cursor).ok()? };
    let cursor_monitor = unsafe { MonitorFromPoint(cursor, MONITOR_DEFAULTTONEAREST) };
    monitors()
        .into_iter()
        .position(|monitor| monitor == cursor_monitor)
}

// gpui numbers displays by the order of EnumDisplayMonitors, so the index matches its display list.
fn monitors() -> Vec<HMONITOR> {
    let mut monitors = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(collect_monitor),
            LPARAM(&mut monitors as *mut Vec<HMONITOR> as isize),
        );
    }
    monitors
}

unsafe extern "system" fn collect_monitor(
    monitor: HMONITOR,
    _device_context: HDC,
    _bounds: *mut RECT,
    monitors: LPARAM,
) -> windows::core::BOOL {
    let monitors = unsafe { &mut *(monitors.0 as *mut Vec<HMONITOR>) };
    monitors.push(monitor);
    TRUE
}

fn window_handle(window: &Window) -> Result<HWND> {
    let handle = HasWindowHandle::window_handle(window)
        .map_err(|error| anyhow!("no window handle: {error}"))?;
    match handle.as_raw() {
        RawWindowHandle::Win32(handle) => Ok(HWND(handle.hwnd.get() as *mut c_void)),
        other => Err(anyhow!("expected a Win32 window handle, got {other:?}")),
    }
}
