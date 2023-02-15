use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::{w, Foundation::Numerics::Vector2};

use crate::graphics::Geometry::Rectangle;
use crate::graphics::Geometry::Text;

use crate::graphics::Geometry;

/// A constant for the paddle movement velocity.
const PADDLE_VELOCITY: f32 = 0.001;

/// A constant for the ball movement velocity.
const BALL_VELOCITY: f32 = 0.0003;

pub struct Game {
    pub ball: Entity,
    pub left_paddle: Entity,
    pub right_paddle: Entity,
    pub top_wall: Entity,
    pub bottom_wall: Entity,
    pub left_score: Entity,
    pub right_score: Entity,

    ball_x_movement: f32,
    ball_y_movement: f32,

    left_player: Player,
    right_player: Player,

    countdown: Duration,
}

struct Player {
    movement: f32,
    points: u8,
}

impl Game {
    pub fn new() -> Self {
        Game {
            ball: Entity {
                pos: Vector2 { X: 0.5, Y: 0.5 },
                geo: Rectangle {
                    w: 0.025,
                    h: 0.0325,
                },
            },
            left_paddle: Entity {
                pos: Vector2 {
                    X: 0.05,
                    Y: 0.5 - 0.075,
                },
                geo: Rectangle { w: 0.025, h: 0.15 },
            },
            right_paddle: Entity {
                pos: Vector2 {
                    X: 1.0 - 0.025 - 0.05,
                    Y: 0.5 - (0.15 / 2.0),
                },
                geo: Rectangle { w: 0.025, h: 0.15 },
            },
            top_wall: Entity {
                pos: Vector2 { X: 0.0, Y: 0.0 },
                geo: Rectangle { w: 1.0, h: 0.03 },
            },
            bottom_wall: Entity {
                pos: Vector2 {
                    X: 0.0,
                    Y: 1.0 - 0.03,
                },
                geo: Rectangle { w: 1.0, h: 0.03 },
            },
            left_score: Entity {
                pos: Vector2 { X: 0.35, Y: 0.15 },
                geo: Text {
                    text: unsafe { w!("0").as_wide().to_vec() },
                },
            },
            right_score: Entity {
                pos: Vector2 { X: 0.65, Y: 0.15 },
                geo: Text {
                    text: unsafe { w!("0").as_wide().to_vec() },
                },
            },
            ball_x_movement: 1.0,
            ball_y_movement: -1.0,
            left_player: Player {
                movement: 0.0,
                points: 0,
            },
            right_player: Player {
                movement: 0.0,
                points: 0,
            },
            countdown: Duration::from_secs(1),
        }
    }

    pub fn tick(&mut self, dt: Duration) {
        let millis = dt.as_millis();

        // Skip physics if countdown is still in progress.
        self.countdown -= Duration::min(self.countdown, dt);
        if !self.countdown.is_zero() {
            return;
        }

        self.right_paddle.pos.Y += self.right_player.movement * PADDLE_VELOCITY * millis as f32;
        self.left_paddle.pos.Y += self.left_player.movement * PADDLE_VELOCITY * millis as f32;

        self.ball.pos.Y += self.ball_y_movement * BALL_VELOCITY * millis as f32;
        self.ball.pos.X += self.ball_x_movement * BALL_VELOCITY * millis as f32;

        // reflect ball Y-movement if it hits the bottom wall.
        if self.bottom_wall.pos.Y <= (self.ball.pos.Y + 0.0325) {
            self.ball.pos.Y = self.bottom_wall.pos.Y - 0.0325 - 0.001; // nudge
            self.ball_y_movement = -self.ball_y_movement;
        }

        // reflect ball Y-movement if it hits the top wall.
        if (self.top_wall.pos.Y + 0.03) >= self.ball.pos.Y {
            self.ball.pos.Y = self.top_wall.pos.Y + 0.03 + 0.001; // nudge
            self.ball_y_movement = -self.ball_y_movement;
        }

        // don't let right paddle to go out of wall limits
        if self.right_paddle.pos.Y < 0.03 {
            self.right_paddle.pos.Y = 0.03;
        } else if self.right_paddle.pos.Y > (1.0 - 0.03 - 0.15) {
            self.right_paddle.pos.Y = 1.0 - 0.03 - 0.15;
        }

        // don't let left paddle to go out of wall limits
        if self.left_paddle.pos.Y < 0.03 {
            self.left_paddle.pos.Y = 0.03;
        } else if self.left_paddle.pos.Y > (1.0 - 0.03 - 0.15) {
            self.left_paddle.pos.Y = 1.0 - 0.03 - 0.15;
        }

        // reflect ball X-movement if it hits the left paddle.
        if Self::collides(
            &self.left_paddle.pos,
            &Vector2 { X: 0.025, Y: 0.15 },
            &self.ball.pos,
            &Vector2 {
                X: 0.025,
                Y: 0.0325,
            },
        ) {
            self.ball.pos.X = self.left_paddle.pos.X + 0.025 + 0.001; // nudge
            self.ball_x_movement = -self.ball_x_movement;
        }

        // reflect ball X-movement if it hits the right paddle.
        if Self::collides(
            &self.right_paddle.pos,
            &Vector2 { X: 0.025, Y: 0.15 },
            &self.ball.pos,
            &Vector2 {
                X: 0.025,
                Y: 0.0325,
            },
        ) {
            self.ball.pos.X = self.right_paddle.pos.X - 0.025 - 0.001; // nudge
            self.ball_x_movement = -self.ball_x_movement;
        }

        // Check whether ball hits the left goal.
        if self.ball.pos.X <= 0.0 {
            self.clear_state();
            self.right_player.points = u8::min(9, self.right_player.points + 1);
            // TODO check if game is over?
            let boom: Vec<u16> = self
                .right_player
                .points
                .to_string()
                .encode_utf16()
                .collect();
            match &mut self.right_score.geo {
                Geometry::Text { text } => *text = boom,
                _ => (),
            }
        }

        // Check whether ball hits the right goal.
        if (self.ball.pos.X + 0.025) >= 1.0 {
            self.clear_state();
            self.left_player.points = u8::min(9, self.left_player.points + 1);
            // TODO check if game is over?
            let boom: Vec<u16> = self.left_player.points.to_string().encode_utf16().collect();
            match &mut self.left_score.geo {
                Geometry::Text { text } => *text = boom,
                _ => (),
            }
        }
    }

    /// Clear the gameyard state by centering the ball and paddles and starting a new countdown.
    fn clear_state(&mut self) {
        self.ball.pos.X = 0.5;
        self.ball.pos.Y = 0.5;
        self.left_paddle.pos.Y = 0.425;
        self.right_paddle.pos.Y = 0.425;
        self.countdown = Duration::from_secs(1);
    }

    fn collides(a_pos: &Vector2, a_size: &Vector2, b_pos: &Vector2, b_size: &Vector2) -> bool {
        a_pos.X < (b_pos.X + b_size.X)
            && a_pos.Y < (b_pos.Y + b_size.Y)
            && (a_pos.X + a_size.X) > b_pos.X
            && (a_pos.Y + a_size.Y) > b_pos.Y
    }

    pub fn on_key_down(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => self.right_player.movement = -1.0,
            VK_DOWN => self.right_player.movement = 1.0,
            VK_W => self.left_player.movement = -1.0,
            VK_S => self.left_player.movement = 1.0,
            _ => (),
        }
    }

    pub fn on_key_up(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => self.right_player.movement = f32::max(self.right_player.movement, 0.0),
            VK_DOWN => self.right_player.movement = f32::min(self.right_player.movement, 0.0),
            VK_W => self.left_player.movement = f32::max(self.left_player.movement, 0.0),
            VK_S => self.left_player.movement = f32::min(self.left_player.movement, 0.0),
            _ => (),
        }
    }
}

/// An object representing a single item in the game world.
pub struct Entity {
    pub pos: Vector2,
    pub geo: Geometry,
}
