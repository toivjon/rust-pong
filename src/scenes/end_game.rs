use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_RETURN};

use crate::{
    geometry::{Text, TextSize},
    graphics::Graphics,
    scenes::MainMenu,
    scenes::Scene,
};

/// The scene which shows the end game results.
pub struct EndGame {
    topic: Text,
    result: Text,
    help: Text,
}

impl EndGame {
    pub fn new(l_score: u8, r_score: u8) -> Self {
        Self {
            topic: Text {
                x: 0.5,
                y: 0.25,
                text: "GAME OVER".encode_utf16().collect(),
                size: TextSize::Big,
            },
            result: Text {
                x: 0.5,
                y: 0.525,
                text: format!("{} - {}", r_score, l_score)
                    .encode_utf16()
                    .collect(),
                size: TextSize::Big,
            },
            help: Text {
                x: 0.5,
                y: 0.75,
                text: "Press ENTER to go back to main menu"
                    .encode_utf16()
                    .collect(),
                size: TextSize::Small,
            },
        }
    }
}

impl Scene for EndGame {
    fn tick(self: Box<Self>, _dt: Duration) -> Option<Box<dyn Scene>> {
        Some(self)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_text(&self.topic);
        ctx.draw_text(&self.result);
        ctx.draw_text(&self.help);
    }

    fn key_down(self: Box<Self>, _key: u16) -> Option<Box<dyn Scene>> {
        Some(self)
    }

    fn key_up(self: Box<Self>, key: u16) -> Option<Box<dyn Scene>> {
        if VIRTUAL_KEY(key) == VK_RETURN {
            return Some(Box::new(MainMenu::new()));
        }
        Some(self)
    }
}
