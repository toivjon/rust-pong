use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::{
    app::Scene,
    geometry::{Rectangle, Text},
    graphics::Graphics,
    scenes::EndGame,
};

/// A constant for the paddle movement velocity.
const PADDLE_VELOCITY: f32 = 0.001;

/// A constant for the ball movement velocity.
const BALL_VELOCITY: f32 = 0.0003;

/// The duration of the countdown at the beginning of each round.
const COUNTDOWN: Duration = Duration::from_secs(1);

/// The amount of additional push added to collision handling.
const NUDGE: f32 = 0.001;

pub struct Court {
    pub ball: Rectangle,
    pub left_paddle: Rectangle,
    pub right_paddle: Rectangle,
    pub top_wall: Rectangle,
    pub bottom_wall: Rectangle,
    pub left_score: Text,
    pub right_score: Text,

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

impl Court {
    pub fn new() -> Self {
        Court {
            ball: Rectangle {
                x: 0.5 - (0.025 / 2.0),
                y: 0.5 - (0.0325 / 2.0),
                w: 0.025,
                h: 0.0325,
            },
            left_paddle: Rectangle {
                x: 0.05,
                y: 0.5 - 0.075,
                w: 0.025,
                h: 0.15,
            },
            right_paddle: Rectangle {
                x: 1.0 - 0.025 - 0.05,
                y: 0.5 - (0.15 / 2.0),
                w: 0.025,
                h: 0.15,
            },
            top_wall: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 0.03,
            },
            bottom_wall: Rectangle {
                x: 0.0,
                y: 1.0 - 0.03,
                w: 1.0,
                h: 0.03,
            },
            left_score: Text {
                x: 0.35,
                y: 0.15,
                text: "0".encode_utf16().collect(),
                big: true,
            },
            right_score: Text {
                x: 0.65,
                y: 0.15,
                text: "0".encode_utf16().collect(),
                big: true,
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
            countdown: COUNTDOWN,
        }
    }

    /// Apply the movement for all dynamic entities based on the provided delta time.
    fn apply_movement(&mut self, dt: Duration) {
        let millis = dt.as_millis() as f32;
        self.right_paddle.y += self.right_player.movement * PADDLE_VELOCITY * millis;
        self.left_paddle.y += self.left_player.movement * PADDLE_VELOCITY * millis;
        self.ball.y += self.ball_y_movement * BALL_VELOCITY * millis;
        self.ball.x += self.ball_x_movement * BALL_VELOCITY * millis;
    }

    /// Clear the gameyard state by centering the ball and paddles and starting a new countdown.
    fn clear_state(&mut self) {
        self.ball.x = 0.5 - (self.ball.w / 2.0);
        self.ball.y = 0.5 - (self.ball.h / 2.0);
        self.left_paddle.y = 0.5 - (self.left_paddle.h / 2.0);
        self.right_paddle.y = 0.5 - (self.right_paddle.h / 2.0);
        self.countdown = COUNTDOWN;
    }
}

impl Scene for Court {
    fn tick(&mut self, dt: Duration) -> (Option<Box<dyn Scene>>, bool) {
        // Skip physics if countdown is still in progress.
        self.countdown -= Duration::min(self.countdown, dt);
        if !self.countdown.is_zero() {
            return (None, false);
        }
        self.apply_movement(dt);

        // don't let right paddle to go out of wall limits
        if self.right_paddle.collides(&self.top_wall) {
            self.right_paddle.y = self.top_wall.y + self.top_wall.h + NUDGE;
        } else if self.right_paddle.collides(&self.bottom_wall) {
            self.right_paddle.y = self.bottom_wall.y - self.right_paddle.h - NUDGE;
        }

        // don't let left paddle to go out of wall limits
        if self.left_paddle.collides(&self.top_wall) {
            self.left_paddle.y = self.top_wall.y + self.top_wall.h + NUDGE;
        } else if self.left_paddle.collides(&self.bottom_wall) {
            self.left_paddle.y = self.bottom_wall.y - self.left_paddle.h - NUDGE;
        }

        // reflect ball Y-movement if it hits the walls.
        if self.bottom_wall.collides(&self.ball) {
            self.ball.y = self.bottom_wall.y - self.ball.h - NUDGE;
            self.ball_y_movement = -self.ball_y_movement;
        } else if self.top_wall.collides(&self.ball) {
            self.ball.y = self.top_wall.y + self.top_wall.h + NUDGE;
            self.ball_y_movement = -self.ball_y_movement;
        }

        // reflect ball X-movement if it hits the paddles.
        if self.left_paddle.collides(&self.ball) {
            self.ball.x = self.left_paddle.x + self.left_paddle.w + NUDGE;
            self.ball_x_movement = -self.ball_x_movement;
        } else if self.right_paddle.collides(&self.ball) {
            self.ball.x = self.right_paddle.x - self.ball.w - NUDGE;
            self.ball_x_movement = -self.ball_x_movement;
        }

        // Check whether ball hits the goals.
        if self.ball.x <= 0.0 {
            self.clear_state();
            self.right_player.points += 1;
            if self.right_player.points >= 10 {
                return (
                    Some(Box::new(EndGame::new(
                        self.left_player.points,
                        self.right_player.points,
                    ))),
                    false,
                );
            }
            self.right_score.set_text(self.right_player.points);
        } else if (self.ball.x + self.ball.w) >= 1.0 {
            self.clear_state();
            self.left_player.points += 1;
            if self.left_player.points >= 10 {
                return (
                    Some(Box::new(EndGame::new(
                        self.left_player.points,
                        self.right_player.points,
                    ))),
                    false,
                );
            }
            self.left_score.set_text(self.left_player.points);
        }
        (None, false)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_rectangle(&self.ball);
        ctx.draw_rectangle(&self.ball);
        ctx.draw_rectangle(&self.left_paddle);
        ctx.draw_rectangle(&self.right_paddle);
        ctx.draw_rectangle(&self.top_wall);
        ctx.draw_rectangle(&self.bottom_wall);
        ctx.draw_text(&self.left_score);
        ctx.draw_text(&self.right_score);
    }

    fn on_key_down(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => self.right_player.movement = -1.0,
            VK_DOWN => self.right_player.movement = 1.0,
            VK_W => self.left_player.movement = -1.0,
            VK_S => self.left_player.movement = 1.0,
            _ => (),
        }
    }

    fn on_key_up(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP => self.right_player.movement = f32::max(self.right_player.movement, 0.0),
            VK_DOWN => self.right_player.movement = f32::min(self.right_player.movement, 0.0),
            VK_W => self.left_player.movement = f32::max(self.left_player.movement, 0.0),
            VK_S => self.left_player.movement = f32::min(self.left_player.movement, 0.0),
            _ => (),
        }
    }
}
