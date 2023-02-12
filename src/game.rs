use std::collections::HashMap;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::{w, Foundation::Numerics::Vector2};

use crate::graphics::{Geometry, Rectangle, Text};

/// A constant for the paddle movement velocity.
const PADDLE_VELOCITY: f32 = 0.001;

pub struct Game {
    pub ball: Entity,
    pub left_paddle: Entity,
    pub right_paddle: Entity,
    pub top_wall: Entity,
    pub bottom_wall: Entity,
    pub left_score: Entity,
    pub right_score: Entity,

    left_player: Player,
    right_player: Player,
}

struct Player {
    y_movement: f32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            ball: Entity {
                pos: Vector2 { X: 0.5, Y: 0.5 },
                geo: Box::new(Rectangle {
                    w: 0.025,
                    h: 0.0325,
                }),
            },
            left_paddle: Entity {
                pos: Vector2 {
                    X: 0.05,
                    Y: 0.5 - 0.075,
                },
                geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
            },
            right_paddle: Entity {
                pos: Vector2 {
                    X: 1.0 - 0.025 - 0.05,
                    Y: 0.5 - (0.15 / 2.0),
                },
                geo: Box::new(Rectangle { w: 0.025, h: 0.15 }),
            },
            top_wall: Entity {
                pos: Vector2 { X: 0.0, Y: 0.0 },
                geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
            },
            bottom_wall: Entity {
                pos: Vector2 {
                    X: 0.0,
                    Y: 1.0 - 0.03,
                },
                geo: Box::new(Rectangle { w: 1.0, h: 0.03 }),
            },
            left_score: Entity {
                pos: Vector2 { X: 0.35, Y: 0.15 },
                geo: Box::new(Text {
                    text: unsafe { w!("0").as_wide().to_vec() },
                }),
            },
            right_score: Entity {
                pos: Vector2 { X: 0.65, Y: 0.15 },
                geo: Box::new(Text {
                    text: unsafe { w!("0").as_wide().to_vec() },
                }),
            },
            left_player: Player { y_movement: 0.0 },
            right_player: Player { y_movement: 0.0 },
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let millis = dt.as_millis();

        self.right_paddle.pos.Y += self.right_player.y_movement * PADDLE_VELOCITY * millis as f32;
        self.left_paddle.pos.Y += self.left_player.y_movement * PADDLE_VELOCITY * millis as f32;

        // TODO apply movement
        // TODO check collisions

        if self.right_paddle.pos.Y < 0.03 {
            self.right_paddle.pos.Y = 0.03;
        } else if self.right_paddle.pos.Y > (1.0 - 0.03 - 0.15) {
            self.right_paddle.pos.Y = 1.0 - 0.03 - 0.15;
        }

        if self.left_paddle.pos.Y < 0.03 {
            self.left_paddle.pos.Y = 0.03;
        } else if self.left_paddle.pos.Y > (1.0 - 0.03 - 0.15) {
            self.left_paddle.pos.Y = 1.0 - 0.03 - 0.15;
        }
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
