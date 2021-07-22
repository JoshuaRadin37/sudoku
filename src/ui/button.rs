//! For using buttons

use graphics::types::Rectangle;

/// Contains information needed by the button
pub struct Button {
    /// The position and size of the button on the screen
    pub rect: Rectangle,
    /// What action to take when the button is pressed
    pub on_click: Box<dyn Fn()>,
}

impl Button {
    /// Creates a new button instance
    pub fn new(rect: Rectangle) -> Self {
        Self {
            rect,
            on_click: Box::new(|| {}),
        }
    }

}
