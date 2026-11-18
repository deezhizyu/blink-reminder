use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, KeyDownEvent, MouseButton,
    ParentElement as _, RenderOnce, SharedString, Styled as _, Window, div, px,
};

use crate::{components::ChangeHandler, theme};

const INPUT_WIDTH: f32 = 72.0;
const INPUT_HEIGHT: f32 = 28.0;
const MAXIMUM_TEXT_LENGTH: usize = 6;

#[derive(IntoElement)]
pub struct NumberInput {
    id: SharedString,
    label: SharedString,
    value: f32,
    on_change: ChangeHandler<f32>,
}

impl NumberInput {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        value: f32,
        on_change: impl Fn(f32, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value,
            on_change: Box::new(on_change),
        }
    }
}

impl RenderOnce for NumberInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = ElementId::from(self.id.clone());
        let focus_handle = window
            .use_keyed_state(ElementId::from((self.id.clone(), 0usize)), cx, |_, cx| {
                cx.focus_handle()
            })
            .read(cx)
            .clone();
        let typed_text =
            window.use_keyed_state(ElementId::from((self.id, 1usize)), cx, |_, _| String::new());
        let focused = focus_handle.is_focused(window);
        let shown_text = if focused {
            typed_text.read(cx).clone()
        } else {
            format!("{:.0}", self.value)
        };
        let current_text = format!("{:.0}", self.value);
        let on_change = self.on_change;

        div()
            .flex()
            .justify_between()
            .items_center()
            .child(self.label)
            .child(
                div()
                    .id(id)
                    .track_focus(&focus_handle)
                    .w(px(INPUT_WIDTH))
                    .h(px(INPUT_HEIGHT))
                    .px_2()
                    .flex()
                    .items_center()
                    .justify_end()
                    .rounded_md()
                    .border_1()
                    .border_color(if focused {
                        theme::accent()
                    } else {
                        theme::control_background()
                    })
                    .bg(theme::control_background())
                    .cursor_text()
                    .on_mouse_down(MouseButton::Left, {
                        let focus_handle = focus_handle.clone();
                        let typed_text = typed_text.clone();
                        move |_, window, cx| {
                            typed_text.update(cx, |text, _| text.clone_from(&current_text));
                            focus_handle.focus(window);
                            window.refresh();
                        }
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let keystroke = &event.keystroke;
                        if keystroke.key == "enter" || keystroke.key == "escape" {
                            window.blur();
                            return;
                        }
                        let edited = edit_text(
                            typed_text.read(cx),
                            keystroke.key.as_str(),
                            keystroke.key_char.as_deref(),
                        );
                        if let Some(edited) = edited {
                            if let Ok(value) = edited.parse::<f32>() {
                                on_change(value, cx);
                            }
                            typed_text.update(cx, |text, _| *text = edited);
                            window.refresh();
                        }
                    })
                    .child(shown_text),
            )
    }
}

fn edit_text(text: &str, key: &str, key_char: Option<&str>) -> Option<String> {
    let mut edited = text.to_owned();
    if key == "backspace" {
        edited.pop()?;
        return Some(edited);
    }
    let typed = key_char?;
    let is_number_character = typed
        .chars()
        .all(|character| character.is_ascii_digit() || character == '.');
    if typed.is_empty() || !is_number_character || edited.len() + typed.len() > MAXIMUM_TEXT_LENGTH
    {
        return None;
    }
    edited.push_str(typed);
    Some(edited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_and_dots_are_appended() {
        assert_eq!(edit_text("1", "2", Some("2")), Some("12".to_owned()));
        assert_eq!(edit_text("1", ".", Some(".")), Some("1.".to_owned()));
    }

    #[test]
    fn backspace_removes_the_last_character() {
        assert_eq!(edit_text("12", "backspace", None), Some("1".to_owned()));
        assert_eq!(edit_text("", "backspace", None), None);
    }

    #[test]
    fn other_keys_and_overlong_text_are_ignored() {
        assert_eq!(edit_text("1", "a", Some("a")), None);
        assert_eq!(edit_text("1", "left", None), None);
        assert_eq!(edit_text("123456", "7", Some("7")), None);
    }
}
