use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
    timer::Timer,
};

pub struct App {
    pub graphics: Graphics,
    pub scene: Box<dyn Scene>,
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

    pub fn tick(&mut self) {
        let next_scene = self.scene.tick(self.timer.time());
        if next_scene.0.is_some() {
            self.scene = next_scene.0.unwrap();
        } else if next_scene.1 {
            self.running = false;
        }
    }
}
