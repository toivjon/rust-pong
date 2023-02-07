use graphics::Graphics;
use windows::s;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

mod graphics;

fn main() {
    let window = unsafe { create_window() };
    let mut graphics = Graphics::new(window).unwrap();
    let mut msg = MSG::default();
    loop {
        while unsafe { PeekMessageA(&mut msg, HWND(0), 0, 0, PM_REMOVE).into() } {
            if msg.message == WM_QUIT {
                return;
            }
            unsafe { DispatchMessageA(&msg) };
        }
        graphics.draw();
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

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
