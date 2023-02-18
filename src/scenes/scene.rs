use std::time::Duration;

use crate::graphics::Graphics;

pub trait Scene {
    fn tick(&mut self, dt: Duration) -> (Option<Box<dyn Scene>>, bool);
    fn draw(&self, ctx: &Graphics);
    fn on_key_down(&mut self, key: u16);
    fn on_key_up(&mut self, key: u16);
}
