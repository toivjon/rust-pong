use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
    timer::Timer,
};

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

    /// Resize the application graphics rendering canvas.
    pub fn resize(&mut self) {
        self.graphics.resize();
    }

    /// Update the current state of the application logic.
    pub fn tick(&mut self) {
        if let Some(s) = self.scene.take() {
            self.scene = s.tick(self.timer.time());
        }
    }

    /// Render the current state of the application on the screen.
    pub fn draw(&mut self) {
        if let Some(s) = self.scene.as_ref() {
            self.graphics.draw(s.as_ref()).unwrap();
        }
    }

    /// Tell the application that a keyboard key is being pressed.
    pub fn key_down(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = s.key_down(key);
        }
    }

    /// Tell the application that a keyboard key is being released.
    pub fn key_up(&mut self, key: u16) {
        if let Some(s) = self.scene.take() {
            self.scene = s.key_up(key);
        }
    }

    /// Returns a boolean indicating whether the application has an active scene i.e is running.
    pub fn running(&self) -> bool {
        self.scene.is_some()
    }
}
