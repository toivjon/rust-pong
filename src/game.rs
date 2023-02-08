use std::collections::HashMap;

use windows::{w, Foundation::Numerics::Vector2};

use crate::graphics::{Geometry, Rectangle, Text};

pub struct Game {
    pub entities: HashMap<String, Entity>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            entities: HashMap::from([
                (
                    String::from("ball"),
                    Entity {
                        pos: Vector2 { X: 0.5, Y: 0.5 },
                        geo: Box::new(Rectangle {
                            w: 0.025,
                            h: 0.0325,
                        }),
                    },
                ),
                (
                    String::from("left-paddle"),
                    Entity {
                        pos: Vector2 {
                            X: 0.05,
                            Y: 0.5 - 0.075,
                        },
                        geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
                    },
                ),
                (
                    String::from("right-paddle"),
                    Entity {
                        pos: Vector2 {
                            X: 1.0 - 0.025 - 0.05,
                            Y: 0.5 - (0.15 / 2.0),
                        },
                        geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
                    },
                ),
                (
                    String::from("top-border"),
                    Entity {
                        pos: Vector2 { X: 0.0, Y: 0.0 },
                        geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
                    },
                ),
                (
                    String::from("bottom-border"),
                    Entity {
                        pos: Vector2 {
                            X: 0.0,
                            Y: 1.0 - 0.03,
                        },
                        geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
                    },
                ),
                (
                    String::from("left-score"),
                    Entity {
                        pos: Vector2 { X: 0.35, Y: 0.15 },
                        geo: Box::new(Text {
                            text: unsafe { w!("0").as_wide().to_vec() },
                        }),
                    },
                ),
                (
                    String::from("right-score"),
                    Entity {
                        pos: Vector2 { X: 0.65, Y: 0.15 },
                        geo: Box::new(Text {
                            text: unsafe { w!("0").as_wide().to_vec() },
                        }),
                    },
                ),
            ]),
        }
    }
}

/// An object representing a single item in the game world.
pub struct Entity {
    pub pos: Vector2,
    pub geo: Box<dyn Geometry>,
}
