use std::time::Duration;

use crate::graphics::Graphics;

pub trait Scene {
    fn tick(self: Box<Self>, dt: Duration) -> Option<Box<dyn Scene>>;
    fn draw(&self, ctx: &Graphics);
    fn key_down(self: Box<Self>, key: u16) -> Option<Box<dyn Scene>>;
    fn key_up(self: Box<Self>, key: u16) -> Option<Box<dyn Scene>>;
}
