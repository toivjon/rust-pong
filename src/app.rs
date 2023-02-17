use std::time::{Duration, Instant};

use windows::core::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{graphics::Graphics, main_menu::MainMenu};

pub struct App {
    graphics: Graphics,
    scene: Box<dyn Scene>,
    tick_time: Instant,
}

pub trait Scene {
    fn tick(&mut self, dt: Duration) -> Option<Box<dyn Scene>>;
    fn draw(&self, ctx: &Graphics);
    fn on_key_down(&mut self, key: u16);
    fn on_key_up(&mut self, key: u16);
}

impl App {
    pub fn new(window: HWND) -> Self {
        let mut app = App {
            graphics: Graphics::new(window).unwrap(),
            scene: Box::new(MainMenu::new()),
            tick_time: Instant::now(),
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
        if next_scene.is_some() {
            self.scene = next_scene.unwrap();
        }
    }

    pub fn draw(&mut self) -> Result<()> {
        self.graphics.begin_draw()?;
        self.scene.draw(&self.graphics);
        self.graphics.end_draw();
        Ok(())
    }
}
