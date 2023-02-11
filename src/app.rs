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
        let graphics = Graphics::new(window).unwrap();
        let game = Game::new();
        let mut app = App {
            graphics,
            game,
            tick_time: Instant::now(),
        };
        unsafe { SetWindowLongPtrA(window, GWLP_USERDATA, &mut app as *mut _ as _) };
        app
    }

    /// A handler for the incoming operating system messages for the application.
    pub unsafe fn message_handler(
        &mut self,
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SIZE => {
                self.graphics.resize().unwrap();
                LRESULT(0)
            }
            WM_KEYDOWN => {
                self.game.on_key_down(wparam.0 as u16);
                LRESULT(0)
            }
            WM_KEYUP => {
                self.game.on_key_up(wparam.0 as u16);
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, msg, wparam, lparam),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.tick_time);
        self.tick_time = now;

        self.game.tick(delta_time)
    }

    pub fn draw(&mut self) -> Result<()> {
        self.graphics.draw(&self.game.entities)
    }
}
