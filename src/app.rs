use std::time::Instant;

use windows::core::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{game::Game, graphics::Graphics};

pub struct App {
    graphics: Graphics,
    game: Game,
    tick_time: Instant,
}

impl App {
    pub fn new(window: HWND) -> Self {
        let mut app = App {
            graphics: Graphics::new(window).unwrap(),
            game: Game::new(),
            tick_time: Instant::now(),
        };
        unsafe { SetWindowLongPtrA(window, GWLP_USERDATA, &mut app as *mut _ as _) };
        app
    }

    pub fn on_resize(&mut self) {
        self.graphics.resize().unwrap()
    }

    pub fn on_key_down(&mut self, key: u16) {
        self.game.on_key_down(key);
    }

    pub fn on_key_up(&mut self, key: u16) {
        self.game.on_key_up(key);
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.tick_time);
        self.tick_time = now;

        self.game.tick(delta_time)
    }

    pub fn draw(&mut self) -> Result<()> {
        self.graphics.begin_draw()?;
        self.game.draw(&self.graphics);
        self.graphics.end_draw();
        Ok(())
    }
}
