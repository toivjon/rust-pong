use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

/// A context object for graphics operations.
pub struct Graphics {
    ctx: ID2D1HwndRenderTarget,
    brush: ID2D1SolidColorBrush,
}

impl Graphics {
    pub fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        let pixel_size = hwnd_size(hwnd);

        let ctx = unsafe {
            factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd,
                    pixelSize: pixel_size,
                    ..Default::default()
                },
            )?
        };
 
        let brush = unsafe { ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }, None)? };

        Ok(Graphics { ctx, brush })
    }

    pub fn draw(&mut self) {
        unsafe {
            self.ctx.BeginDraw();
            self.ctx.Clear(Some(&D2D1_COLOR_F::default()));
            self.ctx.FillRectangle(
                &D2D_RECT_F {
                    right: 100.0,
                    bottom: 100.0,
                    ..Default::default()
                },
                &self.brush,
            );
            self.ctx.EndDraw(None, None).unwrap();
        }
    }
}

/// Construct a new Direct2D factory used to build Direct2D specific items.
fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();
    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }
    unsafe { D2D1CreateFactory::<ID2D1Factory1>(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}

/// Get the client rect size of the provided window handle.
fn hwnd_size(hwnd: HWND) -> D2D_SIZE_U {
    let mut rect = windows::Win32::Foundation::RECT::default();
    unsafe { GetClientRect(hwnd, &mut rect) };
    D2D_SIZE_U {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    }
}
