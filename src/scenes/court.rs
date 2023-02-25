use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

use crate::{
    geometry::{Rectangle, Text, TextSize},
    graphics::Graphics,
    scenes::EndGame,
    scenes::Scene,
};

/// A constant for the paddle movement velocity.
const PADDLE_VELOCITY: f32 = 0.001;

/// Initial ball movement velocity.
const BALL_VELOCITY: f32 = 0.0003;

/// A scalar to speed up ball velocity on each paddle hit.
const BALL_VELOCITY_SCALAR: f32 = 1.1;

/// The maximum movement velocity for the ball.
const BALL_MAX_VELOCITY: f32 = 0.0006;

/// The duration of the countdown at the beginning of each round.
const COUNTDOWN: Duration = Duration::from_millis(500);

/// The amount of additional push added to collision handling.
const NUDGE: f32 = 0.001;

pub struct Court {
    ball: Rectangle,
    l_paddle: Rectangle,
    r_paddle: Rectangle,
    top_wall: Rectangle,
    bottom_wall: Rectangle,
    left_score: Text,
    right_score: Text,

    ball_x_movement: f32,
    ball_y_movement: f32,

    l_movement: f32,
    r_movement: f32,

    l_points: u8,
    r_points: u8,

    countdown: Duration,
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
            l_paddle: Rectangle {
                x: 0.05,
                y: 0.5 - 0.075,
                w: 0.025,
                h: 0.15,
            },
            r_paddle: Rectangle {
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
                size: TextSize::Big,
            },
            right_score: Text {
                x: 0.65,
                y: 0.15,
                text: "0".encode_utf16().collect(),
                size: TextSize::Big,
            },
            ball_x_movement: BALL_VELOCITY,
            ball_y_movement: -BALL_VELOCITY,
            l_movement: 0.0,
            l_points: 0,
            r_movement: 0.0,
            r_points: 0,
            countdown: COUNTDOWN,
        }
    }

    /// Apply the movement for all dynamic entities based on the provided delta time.
    fn apply_movement(&mut self, dt: Duration) {
        let millis = dt.as_millis() as f32;
        self.r_paddle.y += self.r_movement * PADDLE_VELOCITY * millis;
        self.l_paddle.y += self.l_movement * PADDLE_VELOCITY * millis;
        self.ball.y += self.ball_y_movement * millis;
        self.ball.x += self.ball_x_movement * millis;
    }

    /// Clear the gameyard state by centering the ball and paddles and starting a new countdown.
    fn clear_state(&mut self) {
        self.ball.x = 0.5 - (self.ball.w / 2.0);
        self.ball.y = 0.5 - (self.ball.h / 2.0);
        self.l_paddle.y = 0.5 - (self.l_paddle.h / 2.0);
        self.r_paddle.y = 0.5 - (self.r_paddle.h / 2.0);
        self.countdown = COUNTDOWN;
        self.ball_x_movement *= 1.0 / self.ball_x_movement * BALL_VELOCITY;
        self.ball_y_movement *= 1.0 / self.ball_y_movement * BALL_VELOCITY;
    }

    /// Increase the speed of the ball if the max speed is not yet reached.
    fn accelerate_ball(&mut self) {
        self.ball_y_movement *= BALL_VELOCITY_SCALAR;
        self.ball_x_movement *= BALL_VELOCITY_SCALAR;
        self.ball_y_movement = f32::min(self.ball_y_movement, BALL_MAX_VELOCITY);
        self.ball_x_movement = f32::min(self.ball_x_movement, BALL_MAX_VELOCITY);
    }
}

impl Scene for Court {
    fn tick(mut self: Box<Self>, dt: Duration) -> Option<Box<dyn Scene>> {
        // Skip physics if countdown is still in progress.
        self.countdown -= Duration::min(self.countdown, dt);
        if !self.countdown.is_zero() {
            return Some(self);
        }
        self.apply_movement(dt);

        // don't let right paddle to go out of wall limits
        if self.r_paddle.collides(&self.top_wall) {
            self.r_paddle.y = self.top_wall.y + self.top_wall.h + NUDGE;
        } else if self.r_paddle.collides(&self.bottom_wall) {
            self.r_paddle.y = self.bottom_wall.y - self.r_paddle.h - NUDGE;
        }

        // don't let left paddle to go out of wall limits
        if self.l_paddle.collides(&self.top_wall) {
            self.l_paddle.y = self.top_wall.y + self.top_wall.h + NUDGE;
        } else if self.l_paddle.collides(&self.bottom_wall) {
            self.l_paddle.y = self.bottom_wall.y - self.l_paddle.h - NUDGE;
        }

        // reflect ball Y-movement if it hits the walls.
        if self.bottom_wall.collides(&self.ball) {
            self.ball.y = self.bottom_wall.y - self.ball.h - NUDGE;
            self.ball_y_movement = -self.ball_y_movement;
            self.accelerate_ball();
        } else if self.top_wall.collides(&self.ball) {
            self.ball.y = self.top_wall.y + self.top_wall.h + NUDGE;
            self.ball_y_movement = -self.ball_y_movement;
            self.accelerate_ball();
        }

        // reflect ball X-movement if it hits the paddles.
        if self.l_paddle.collides(&self.ball) {
            self.ball.x = self.l_paddle.x + self.l_paddle.w + NUDGE;
            self.ball_x_movement = -self.ball_x_movement;
            self.accelerate_ball();
        } else if self.r_paddle.collides(&self.ball) {
            self.ball.x = self.r_paddle.x - self.ball.w - NUDGE;
            self.ball_x_movement = -self.ball_x_movement;
            self.accelerate_ball();
        }

        // Check whether ball hits the goals.
        if self.ball.x <= 0.0 {
            self.clear_state();
            self.r_points += 1;
            if self.r_points >= 10 {
                return Some(Box::new(EndGame::new(self.l_points, self.r_points)));
            }
            self.right_score.set_text(self.r_points);
        } else if (self.ball.x + self.ball.w) >= 1.0 {
            self.clear_state();
            self.l_points += 1;
            if self.l_points >= 10 {
                return Some(Box::new(EndGame::new(self.l_points, self.r_points)));
            }
            self.left_score.set_text(self.l_points);
        }
        Some(self)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_rectangle(&self.ball);
        ctx.draw_rectangle(&self.ball);
        ctx.draw_rectangle(&self.l_paddle);
        ctx.draw_rectangle(&self.r_paddle);
        ctx.draw_rectangle(&self.top_wall);
        ctx.draw_rectangle(&self.bottom_wall);
        ctx.draw_text(&self.left_score);
        ctx.draw_text(&self.right_score);
    }

    fn key_down(mut self: Box<Self>, key: u16) -> Option<Box<dyn Scene>> {
        match VIRTUAL_KEY(key) {
            VK_UP => self.r_movement = -1.0,
            VK_DOWN => self.r_movement = 1.0,
            VK_W => self.l_movement = -1.0,
            VK_S => self.l_movement = 1.0,
            _ => (),
        }
        Some(self)
    }

    fn key_up(mut self: Box<Self>, key: u16) -> Option<Box<dyn Scene>> {
        match VIRTUAL_KEY(key) {
            VK_UP => self.r_movement = f32::max(self.r_movement, 0.0),
            VK_DOWN => self.r_movement = f32::min(self.r_movement, 0.0),
            VK_W => self.l_movement = f32::max(self.l_movement, 0.0),
            VK_S => self.l_movement = f32::min(self.l_movement, 0.0),
            _ => (),
        }
        Some(self)
    }
}
