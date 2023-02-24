use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
    timer::Timer,
};
use windows::core::Result;

pub struct App {
    graphics: Graphics,
    scene: Option<Box<dyn Scene>>,
    timer: Timer,
}

impl App {
    pub fn new(gfx: Graphics) -> Self {
        App {
            graphics: gfx,
            scene: Some(Box::new(MainMenu::new())),
            timer: Timer::new(),
        }
    }

    pub fn resize(&mut self) {
        self.graphics.resize();
    }

    pub fn tick(&mut self) -> Result<()> {
        if let Some(s) = self.scene.take() {
            let next_scene = s.tick(self.timer.time());
            if let Some(s) = next_scene.as_ref() {
                self.graphics.draw(s.as_ref())?;
            }
            self.scene = next_scene;
        }
        Ok(())
    }

    pub fn key_down(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = s.key_down(key);
        }
    }

    pub fn key_up(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = s.key_up(key);
        }
    }

    pub fn running(&self) -> bool {
        self.scene.is_some()
    }
}
