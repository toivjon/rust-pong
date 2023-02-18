use std::time::Instant;

use windows::core::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{
    graphics::Graphics,
    scenes::{MainMenu, Scene},
};

pub struct App {
    graphics: Graphics,
    scene: Box<dyn Scene>,
    tick_time: Instant,
    pub running: bool,
}

impl App {
    pub fn new(window: HWND) -> Self {
        let mut app = App {
            graphics: Graphics::new(window).unwrap(),
            scene: Box::new(MainMenu::new()),
            tick_time: Instant::now(),
            running: true,
        };
        unsafe { SetWindowLongPtrA(window, GWLP_USERDATA, &mut app as *mut _ as _) };
        app
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
        let now = Instant::now();
        let delta_time = now.duration_since(self.tick_time);
        self.tick_time = now;

        let next_scene = self.scene.tick(delta_time);
        if next_scene.0.is_some() {
            self.scene = next_scene.0.unwrap();
        } else if next_scene.1 {
            self.running = false;
        }
    }

    pub fn draw(&mut self) -> Result<()> {
        self.graphics.begin_draw()?;
        self.scene.draw(&self.graphics);
        self.graphics.end_draw();
        Ok(())
    }
}
