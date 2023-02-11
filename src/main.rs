use app::App;
use windows::core::Result;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

mod app;
mod game;
mod graphics;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut msg = MSG::default();
    loop {
        unsafe {
            // Check and acquire system message from the message queue.
            while PeekMessageA(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    return Ok(());
                }
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
        app.tick();
        app.draw()?;
    }
}
