/// A simple rectangle with a position and size.
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    /// Check whether the rectangle collides with the given rectangle.
    pub fn collides(&self, rect: &Rectangle) -> bool {
        self.x < (rect.x + rect.w)
            && self.y < (rect.y + rect.h)
            && (self.x + self.w) > rect.x
            && (self.y + self.h) > rect.y
    }
}

/// The pre-defined size for a text object.
pub enum TextSize {
    SMALL,
    MEDIUM,
    BIG,
}

/// A simple text with a position, text and format.
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: Vec<u16>,
    pub size: TextSize,
}

impl Text {
    /// Set the to_string result of the given value as the textual content.
    pub fn set_text<T: ToString>(&mut self, val: T) {
        self.text = val.to_string().encode_utf16().collect()
    }
}
