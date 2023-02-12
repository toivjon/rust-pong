use std::collections::HashMap;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::{w, Foundation::Numerics::Vector2};

use crate::graphics::{Geometry, Rectangle, Text};

/// A constant for the paddle movement velocity.
const PADDLE_VELOCITY: f32 = 0.001;

#[derive(Hash, PartialEq, Eq)]
pub enum EntityID {
    Ball,
    LeftPaddle,
    RightPaddle,
    TopWall,
    BottomWall,
    LeftScore,
    RightScore,
}

pub struct Game {
    pub entities: HashMap<EntityID, Entity>,
    left_player: Player,
    right_player: Player,
}

struct Player {
    y_movement: f32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            entities: HashMap::from([
                (
                    EntityID::Ball,
                    Entity {
                        pos: Vector2 { X: 0.5, Y: 0.5 },
                        geo: Box::new(Rectangle {
                            w: 0.025,
                            h: 0.0325,
                        }),
                    },
                ),
                (
                    EntityID::LeftPaddle,
                    Entity {
                        pos: Vector2 {
                            X: 0.05,
                            Y: 0.5 - 0.075,
                        },
                        geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
                    },
                ),
                (
                    EntityID::RightPaddle,
                    Entity {
                        pos: Vector2 {
                            X: 1.0 - 0.025 - 0.05,
                            Y: 0.5 - (0.15 / 2.0),
                        },
                        geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
                    },
                ),
                (
                    EntityID::TopWall,
                    Entity {
                        pos: Vector2 { X: 0.0, Y: 0.0 },
                        geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
                    },
                ),
                (
                    EntityID::BottomWall,
                    Entity {
                        pos: Vector2 {
                            X: 0.0,
                            Y: 1.0 - 0.03,
                        },
                        geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
                    },
                ),
                (
                    EntityID::LeftScore,
                    Entity {
                        pos: Vector2 { X: 0.35, Y: 0.15 },
                        geo: Box::new(Text {
                            text: unsafe { w!("0").as_wide().to_vec() },
                        }),
                    },
                ),
                (
                    EntityID::RightScore,
                    Entity {
                        pos: Vector2 { X: 0.65, Y: 0.15 },
                        geo: Box::new(Text {
                            text: unsafe { w!("0").as_wide().to_vec() },
                        }),
                    },
                ),
            ]),
            left_player: Player { y_movement: 0.0 },
            right_player: Player { y_movement: 0.0 },
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let millis = dt.as_millis();

        // TODO There's gotta be a cleaner way to do this update.
        self.entities.entry(EntityID::RightPaddle).and_modify(|x| {
            x.pos.Y += self.right_player.y_movement * PADDLE_VELOCITY * millis as f32;
        });

        // TODO There's gotta be a cleaner way to do this update.
        self.entities.entry(EntityID::LeftPaddle).and_modify(|x| {
            x.pos.Y += self.left_player.y_movement * PADDLE_VELOCITY * millis as f32;
        });

        // TODO apply movement
        // TODO check collisions

        // TODO There's gotta be a cleaner way to do this update.
        self.entities.entry(EntityID::RightPaddle).and_modify(|x| {
            if x.pos.Y < 0.03 {
                x.pos.Y = 0.03;
            } else if x.pos.Y > (1.0 - 0.03 - 0.15) {
                x.pos.Y = 1.0 - 0.03 - 0.15;
            }
        });

        // TODO There's gotta be a cleaner way to do this update.
        self.entities.entry(EntityID::LeftPaddle).and_modify(|x| {
            if x.pos.Y < 0.03 {
                x.pos.Y = 0.03;
            } else if x.pos.Y > (1.0 - 0.03 - 0.15) {
                x.pos.Y = 1.0 - 0.03 - 0.15;
            }
        });
    }

    pub fn on_key_down(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => self.right_player.y_movement = -1.0,
            VK_DOWN => self.right_player.y_movement = 1.0,
            VK_W => self.left_player.y_movement = -1.0,
            VK_S => self.left_player.y_movement = 1.0,
            _ => (),
        }
    }

    pub fn on_key_up(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => {
                if self.right_player.y_movement < 0.0 {
                    self.right_player.y_movement = 0.0;
                }
            }
            VK_DOWN => {
                if self.right_player.y_movement > 0.0 {
                    self.right_player.y_movement = 0.0;
                }
            }
            VK_W => {
                if self.left_player.y_movement < 0.0 {
                    self.left_player.y_movement = 0.0;
                }
            }
            VK_S => {
                if self.left_player.y_movement > 0.0 {
                    self.left_player.y_movement = 0.0;
                }
            }
            _ => (),
        }
    }
}

/// An object representing a single item in the game world.
pub struct Entity {
    pub pos: Vector2,
    pub geo: Box<dyn Geometry>,
}
