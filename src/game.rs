use windows::Foundation::Numerics::Vector2;

use crate::graphics::{Geometry, Rectangle};

/// An object representing a single item in the game world.
pub struct Entity {
    pub pos: Vector2,
    pub geo: Box<dyn Geometry>,
}

pub fn create_entities() -> Vec<Entity> {
    vec![Entity {
        pos: Vector2 { X: 0.5, Y: 0.5 },
        geo: Box::new(Rectangle {
            w: 0.025,
            h: 0.0325,
        }),
    }]
}
