/// A simple rectangle with a position and size.
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// A simple text with a position, text and format.
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub text: Vec<u16>,
}
