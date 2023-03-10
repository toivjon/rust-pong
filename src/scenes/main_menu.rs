use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_DOWN, VK_RETURN, VK_UP};

use crate::{
    geometry::{Rectangle, Text, TextSize},
    graphics::Graphics,
    scenes::Court,
    scenes::Scene,
};

/// The menu where player can select to start or quit the game.
pub struct MainMenu {
    header: Rectangle,
    topic: Text,
    help: Text,
    start: Text,
    quit: Text,
    highlighter: Rectangle,
    footer: Rectangle,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            header: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 1.0,
                h: 0.03,
            },
            topic: Text {
                x: 0.5,
                y: 0.20,
                text: "PONG".encode_utf16().collect(),
                size: TextSize::Big,
            },
            help: Text {
                x: 0.5,
                y: 0.4,
                text: "Select a menu item with UP or DOWN arrows and press ENTER."
                    .encode_utf16()
                    .collect(),
                size: TextSize::Tiny,
            },
            start: Text {
                x: 0.5,
                y: 0.525,
                text: "START".encode_utf16().collect(),
                size: TextSize::Medium,
            },
            quit: Text {
                x: 0.5,
                y: 0.7,
                text: "QUIT".encode_utf16().collect(),
                size: TextSize::Medium,
            },
            highlighter: Rectangle {
                x: 0.3,
                y: 0.51,
                w: 0.03,
                h: 0.03,
            },
            footer: Rectangle {
                x: 0.0,
                y: 1.0 - 0.03,
                w: 1.0,
                h: 0.03,
            },
        }
    }

    /// Change the menu selection based on the current selection.
    fn toggle_selection(&mut self) {
        if self.highlighter.y < self.start.y {
            self.highlighter.y = 0.685;
        } else {
            self.highlighter.y = 0.51;
        }
    }
}

impl Scene for MainMenu {
    fn tick(self: Box<Self>, _dt: Duration) -> Option<Box<dyn Scene>> {
        Some(self)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_rectangle(&self.header);
        ctx.draw_text(&self.topic);
        ctx.draw_text(&self.help);
        ctx.draw_text(&self.start);
        ctx.draw_text(&self.quit);
        ctx.draw_rectangle(&self.highlighter);
        ctx.draw_rectangle(&self.footer);
    }

    fn key_down(self: Box<Self>, _key: u16) -> Option<Box<dyn Scene>> {
        Some(self)
    }

    fn key_up(mut self: Box<Self>, key: u16) -> Option<Box<dyn Scene>> {
        match VIRTUAL_KEY(key) {
            VK_UP | VK_DOWN => self.toggle_selection(),
            VK_RETURN => {
                if self.highlighter.y < self.start.y {
                    return Some(Box::new(Court::new()));
                } else {
                    return None;
                }
            }
            _ => (),
        }
        Some(self)
    }
}
