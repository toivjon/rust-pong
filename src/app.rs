use windows::core::Result;

use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
    timer::Timer,
};

pub struct App {
    graphics: Graphics,
    scene: Box<dyn Scene>,
    timer: Timer,
    pub running: bool,
}

impl App {
    pub fn new(gfx: Graphics) -> Self {
        App {
            graphics: gfx,
            scene: Box::new(MainMenu::new()),
            timer: Timer::new(),
            running: true,
        }
    }

    pub fn on_resize(&mut self) {
        self.graphics.resize().unwrap()
    }

    pub fn on_key_down(&mut self, key: u16) {
        self.scene.on_key_down(key);
    }

    pub fn on_key_up(&mut self, key: u16) {
        self.scene.on_key_up(key);
    }

    pub fn tick(&mut self) {
        let next_scene = self.scene.tick(self.timer.time());
        if next_scene.0.is_some() {
            self.scene = next_scene.0.unwrap();
        } else if next_scene.1 {
            self.running = false;
        }
    }

    pub fn draw(&mut self) -> Result<()> {
        self.graphics.draw_scene(self.scene.as_ref())
    }
}
