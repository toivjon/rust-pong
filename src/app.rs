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
        self.graphics.resize().unwrap();
    }

    pub fn tick(&mut self) -> Result<()> {
        if let Some(s) = self.scene.take() {
            let next_scene = s.tick(self.timer.time());
            self.graphics.draw(next_scene.as_ref())?;
            self.scene = Some(next_scene);
        }
        Ok(())
    }

    pub fn on_key_down(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = Some(s.on_key_down(key));
        }
    }

    pub fn on_key_up(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = Some(s.on_key_up(key));
        }
    }

    pub fn running(&self) -> bool {
        match self.scene {
            Some(ref s) => s.running(),
            _ => false,
        }
    }
}
