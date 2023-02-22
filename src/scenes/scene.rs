use std::time::Duration;

use crate::graphics::Graphics;

pub trait Scene {
    fn tick(self: Box<Self>, dt: Duration) -> Box<dyn Scene>;
    fn draw(&self, ctx: &Graphics);
    fn on_key_down(&mut self, key: u16);
    fn on_key_up(&mut self, key: u16);
    fn running(&self) -> bool {
        true
    }
}
