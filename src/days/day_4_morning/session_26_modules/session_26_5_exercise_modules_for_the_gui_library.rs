use super::gui_library::widgets::{Button, Label, Widget, Window};

/// # 26.5 Exercise: Modules for the GUI Library
///
/// In this exercise, you will reorganize the GUI Library exercise from
/// the “Methods and Traits” segment of the course into a collection of
/// modules. It is typical to put each type or set of closely-related
/// types into its own module, so each widget type should get its own
/// module.
///

pub fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
