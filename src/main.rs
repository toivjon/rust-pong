use windows::s;
use windows::Win32::UI::WindowsAndMessaging::*;

fn main() {
    unsafe {
        MessageBoxA(
            None,
            s!("Hello world!"),
            s!("Important Note"),
            MB_ICONASTERISK | MB_OK,
        )
    };
}
