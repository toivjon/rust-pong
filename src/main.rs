use app::App;
use graphics::Graphics;
use windows::core::Result;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

mod app;
mod geometry;
mod graphics;
mod scenes;
mod timer;

fn main() -> Result<()> {
    let window = create_window();
    let gfx = Graphics::new(window)?;
    let mut app = App::new(gfx);
    let mut msg = MSG::default();
    unsafe { SetWindowLongPtrA(window, GWLP_USERDATA, &mut app as *mut _ as _) };
    while app.running() {
        unsafe {
            // Check and acquire system messages from the message queue.
            while PeekMessageA(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    return Ok(());
                }
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }
        app.tick();
        app.draw();
    }
    Ok(())
}

fn create_window() -> HWND {
    unsafe {
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
}

/// A message router for the incoming operating system messages for the application.
unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let app = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut App;
    if !app.is_null() {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                return LRESULT(0);
            }
            WM_SIZE => {
                (*app).resize();
                return LRESULT(0);
            }
            WM_KEYDOWN => {
                (*app).key_down(wparam.0 as u16);
                return LRESULT(0);
            }
            WM_KEYUP => {
                (*app).key_up(wparam.0 as u16);
                return LRESULT(0);
            }
            _ => (),
        }
    }
    DefWindowProcA(hwnd, msg, wparam, lparam)
}
