use anyhow::Result;
use futures::{StreamExt as _, channel::mpsc};
use gpui::{App, Global};
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};

use crate::settings_window;

const EXECUTABLE_ICON_ORDINAL: u16 = 1;
const TOOLTIP: &str = "Blink Reminder";

struct Tray {
    _icon: TrayIcon,
}

impl Global for Tray {}

pub fn install(cx: &mut App) -> Result<()> {
    let settings_item = MenuItem::new("Settings", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let menu = Menu::new();
    menu.append_items(&[&settings_item, &quit_item])?;
    let icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip(TOOLTIP)
        .with_icon(Icon::from_resource(EXECUTABLE_ICON_ORDINAL, None)?)
        .build()?;

    let (sender, mut receiver) = mpsc::unbounded();
    MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
        let _ = sender.unbounded_send(event.id);
    }));
    let settings_item_id = settings_item.id().clone();
    let quit_item_id = quit_item.id().clone();
    cx.spawn(async move |cx| {
        while let Some(clicked_item_id) = receiver.next().await {
            let _ = cx.update(|cx| {
                if clicked_item_id == settings_item_id {
                    settings_window::open(cx);
                } else if clicked_item_id == quit_item_id {
                    cx.quit();
                }
            });
        }
    })
    .detach();

    cx.set_global(Tray { _icon: icon });
    Ok(())
}
