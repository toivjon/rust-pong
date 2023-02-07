use windows::core::Result;
use windows::Win32::Foundation::{D2DERR_RECREATE_TARGET, HWND};
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

/// A context object for graphics operations.
pub struct Graphics {
    hwnd: HWND,
    factory: ID2D1Factory1,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
}

impl Graphics {
    pub fn new(hwnd: HWND) -> Result<Self> {
        let factory = create_factory()?;
        Ok(Graphics {
            hwnd,
            factory,
            target: None,
            brush: None,
        })
    }

    pub fn draw(&mut self) -> Result<()> {
        if self.target.is_none() {
            self.create_target()?;
        }

        let ctx = self.target.as_ref().unwrap();
        let brush = self.brush.as_ref().unwrap();

        unsafe {
            ctx.BeginDraw();
            ctx.Clear(Some(&D2D1_COLOR_F::default()));
            ctx.FillRectangle(
                &D2D_RECT_F {
                    right: 100.0,
                    bottom: 100.0,
                    ..Default::default()
                },
                brush,
            );
            if let Err(error) = ctx.EndDraw(None, None) {
                if error.code() == D2DERR_RECREATE_TARGET {
                    self.release_target();
                }
            }
        }
        Ok(())
    }

    /// Resize the graphics by changing the size of the render target.
    pub fn resize(&self) -> Result<()> {
        if self.target.is_some() {
            let ctx = self.target.as_ref().unwrap();
            let size = hwnd_size(unsafe { ctx.GetHwnd() });
            unsafe { ctx.Resize(&size)? }
        }
        Ok(())
    }

    /// Create rendering target and related items. This function should be used
    /// during the first draw or when the render target should be re-created.
    fn create_target(&mut self) -> Result<()> {
        unsafe {
            let target = self.factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: self.hwnd,
                    pixelSize: hwnd_size(self.hwnd),
                    ..Default::default()
                },
            )?;
            let brush = target.CreateSolidColorBrush(
                &D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                None,
            )?;
            self.target = Some(target);
            self.brush = Some(brush);
        }
        Ok(())
    }

    /// Release rendering target and related items. These will be automatically
    /// re-created during the next time the draw function is being called.
    fn release_target(&mut self) {
        self.target = None;
        self.brush = None;
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
