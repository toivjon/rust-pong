use std::time::Duration;

use crate::graphics::Graphics;

pub trait Scene {
    fn tick(self: Box<Self>, dt: Duration) -> Box<dyn Scene>;
    fn draw(&self, ctx: &Graphics);
    fn on_key_down(self: Box<Self>, key: u16) -> Box<dyn Scene>;
    fn on_key_up(self: Box<Self>, key: u16) -> Box<dyn Scene>;
    fn running(&self) -> bool {
        true
    }
}
