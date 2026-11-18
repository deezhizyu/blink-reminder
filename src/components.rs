pub mod button;
pub mod number_input;
pub mod position_picker;
pub mod slider;
pub mod toggle;

mod pointer_surface;

use gpui::App;

pub type ChangeHandler<Value> = Box<dyn Fn(Value, &mut App)>;
