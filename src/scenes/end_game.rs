use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_RETURN};

use crate::{app::Scene, geometry::Text, graphics::Graphics, scenes::MainMenu};

pub struct EndGame {
    topic: Text,
    result: Text,
    help: Text,
    selected: bool,
}

impl EndGame {
    pub fn new(l_score: u8, r_score: u8) -> Self {
        Self {
            topic: Text {
                x: 0.5,
                y: 0.2,
                text: "GAME OVER".encode_utf16().collect(),
                big: true,
            },
            result: Text {
                x: 0.5,
                y: 0.5,
                text: format!("{} - {}", r_score, l_score)
                    .encode_utf16()
                    .collect(),
                big: true,
            },
            help: Text {
                x: 0.5,
                y: 0.8,
                text: "Press ENTER to go back to main menu"
                    .encode_utf16()
                    .collect(),
                big: false,
            },
            selected: false,
        }
    }
}

impl Scene for EndGame {
    fn tick(&mut self, _dt: Duration) -> (Option<Box<dyn Scene>>, bool) {
        if self.selected {
            return (Some(Box::new(MainMenu::new())), false);
        }
        (None, false)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_text(&self.topic);
        ctx.draw_text(&self.result);
        ctx.draw_text(&self.help);
    }

    fn on_key_down(&mut self, _key: u16) {
        // ...nothing
    }

    fn on_key_up(&mut self, key: u16) {
        if VIRTUAL_KEY(key) == VK_RETURN {
            self.selected = true;
        }
    }
}
