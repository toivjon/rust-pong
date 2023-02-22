use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
    timer::Timer,
};

pub struct App {
    pub graphics: Graphics,
    pub scene: Box<dyn Scene>,
    pub timer: Timer,
}

impl App {
    pub fn new(gfx: Graphics) -> Self {
        App {
            graphics: gfx,
            scene: Box::new(MainMenu::new()),
            timer: Timer::new(),
        }
    }
}
