use std::time::Duration;

use crate::graphics::Graphics;

/// An application state which handles visible entities and execution logic.
pub trait Scene {
    /// Update the scene logic and physics simulation with the given time step.
    ///
    /// Returned value is used as the next scene or application exit (if None).
    fn tick(self: Box<Self>, dt: Duration) -> Option<Box<dyn Scene>>;

    /// Render the scene contents on the screen with the given graphics context.
    fn draw(&self, ctx: &Graphics);

    /// Tell the scene that a keyboard key is being pressed.
    ///
    /// Returned value is used as the next scene or application exit (if None).
    fn key_down(self: Box<Self>, key: u16) -> Option<Box<dyn Scene>>;

    /// Tell the scene that a keyboard key is being released.
    ///
    /// Returned value is used as the next scene or application exit (if None).
    fn key_up(self: Box<Self>, key: u16) -> Option<Box<dyn Scene>>;
}
