use std::time::Instant;

use game::*;
use graphics::Graphics;
use windows::core::Result;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

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

unsafe fn create_window() -> HWND {
    // Acquire the module handle of the application.
    let instance = GetModuleHandleA(None).unwrap();
    debug_assert!(!instance.is_invalid());

    // Register a window class for the application.
    let class_name = s!("window");
    let class_result = RegisterClassA(&WNDCLASSA {
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hInstance: instance,
        lpszClassName: class_name,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    });
    debug_assert!(class_result != 0);

    // Build the application window.
    CreateWindowExA(
        WINDOW_EX_STYLE::default(),
        class_name,
        s!("Pong"),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        None,
        None,
        instance,
        None,
    )
}

/// A message router for the incoming operating system messages for the application.
unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let app = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut App;
    if !app.is_null() {
        return (*app).message_handler(msg, wparam, lparam);
    }
    DefWindowProcA(hwnd, msg, wparam, lparam)
}

struct App {
    hwnd: HWND,
    graphics: Graphics,
    game: Game,
    tick_time: Instant,
}

impl App {
    fn new() -> Self {
        let hwnd = unsafe { create_window() };
        let graphics = Graphics::new(hwnd).unwrap();
        let game = Game::new();
        let mut app = App {
            hwnd,
            graphics,
            game,
            tick_time: Instant::now(),
        };
        unsafe { SetWindowLongPtrA(app.hwnd, GWLP_USERDATA, &mut app as *mut _ as _) };
        app
    }

    /// A handler for the incoming operating system messages for the application.
    unsafe fn message_handler(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
            _ => DefWindowProcA(self.hwnd, msg, wparam, lparam),
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.tick_time);
        self.tick_time = now;

        self.game.tick(delta_time)
    }

    fn draw(&mut self) -> Result<()> {
        self.graphics.draw(&self.game.entities)
    }
}
