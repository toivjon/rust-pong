use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{VIRTUAL_KEY, VK_DOWN, VK_RETURN, VK_UP};

use crate::{
    geometry::{Rectangle, Text},
    graphics::Graphics,
    scenes::Court,
    scenes::Scene,
};

/// The menu where player can select to start or quit the game.
pub struct MainMenu {
    header: Rectangle,
    topic: Text,
    topic_underline: Rectangle,
    start: Text,
    quit: Text,
    highlighter: Rectangle,
    footer: Rectangle,

    selected: bool,
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
                big: true,
            },
            topic_underline: Rectangle {
                x: 0.25,
                y: 0.325,
                w: 0.5,
                h: 0.03,
            },
            start: Text {
                x: 0.5,
                y: 0.5,
                text: "START".encode_utf16().collect(),
                big: false,
            },
            quit: Text {
                x: 0.5,
                y: 0.7,
                text: "QUIT".encode_utf16().collect(),
                big: false,
            },
            highlighter: Rectangle {
                x: 0.3,
                y: 0.485,
                w: 0.03,
                h: 0.03,
            },
            footer: Rectangle {
                x: 0.0,
                y: 1.0 - 0.03,
                w: 1.0,
                h: 0.03,
            },
            selected: false,
        }
    }

    /// Change the menu selection based on the current selection.
    fn toggle_selection(&mut self) {
        if self.highlighter.y < self.start.y {
            self.highlighter.y = 0.685;
        } else {
            self.highlighter.y = 0.485;
        }
    }
}

impl Scene for MainMenu {
    fn tick(&mut self, _dt: Duration) -> (Option<Box<dyn Scene>>, bool) {
        if self.selected {
            if self.highlighter.y < self.start.y {
                return (Some(Box::new(Court::new())), false);
            } else {
                return (None, true);
            }
        }
        (None, false)
    }

    fn draw(&self, ctx: &Graphics) {
        ctx.draw_rectangle(&self.header);
        ctx.draw_text(&self.topic);
        ctx.draw_rectangle(&self.topic_underline);
        ctx.draw_text(&self.start);
        ctx.draw_text(&self.quit);
        ctx.draw_rectangle(&self.highlighter);
        ctx.draw_rectangle(&self.footer);
    }

    fn on_key_down(&mut self, _key: u16) {
        // ...nothing
    }

    fn on_key_up(&mut self, key: u16) {
        match VIRTUAL_KEY(key) {
            VK_UP | VK_DOWN if !self.selected => self.toggle_selection(),
            VK_RETURN => self.selected = true,
            _ => (),
        }
    }
}
