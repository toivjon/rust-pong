use windows::core::Result;
use windows::w;
use windows::Foundation::Numerics::{Matrix3x2, Vector2};
use windows::Win32::Foundation::{D2DERR_RECREATE_TARGET, HWND};
use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::*;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::geometry::{Rectangle, Text, TextSize};
use crate::scenes::Scene;

/// A constant for the view aspect ratio.
const ASPECT: f32 = 1.3;

/// A context object for graphics operations.
pub struct Graphics {
    hwnd: HWND,
    factory: ID2D1Factory1,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    transform: Matrix3x2,
    big_text_format: IDWriteTextFormat,
    medium_text_format: IDWriteTextFormat,
    small_text_format: IDWriteTextFormat,
    tiny_text_format: IDWriteTextFormat,
}

impl Graphics {
    pub fn new(hwnd: HWND) -> Result<Self> {
        Ok(Graphics {
            hwnd,
            factory: create_factory()?,
            target: None,
            brush: None,
            transform: create_aspect_transform(hwnd),
            big_text_format: create_text_format(0.2),
            medium_text_format: create_text_format(0.1),
            small_text_format: create_text_format(0.05),
            tiny_text_format: create_text_format(0.025),
        })
    }

    pub fn draw(&mut self, scene: &dyn Scene) {
        if self.target.is_none() {
            self.create_target().unwrap();
            self.rebuild_text_formats();
        }
        if let Some(ctx) = self.target.as_ref() {
            unsafe { ctx.BeginDraw() };
            unsafe { ctx.Clear(Some(&D2D1_COLOR_F::default())) };
            scene.draw(self);
            if let Err(error) = unsafe { ctx.EndDraw(None, None) } {
                if error.code() == D2DERR_RECREATE_TARGET {
                    self.release_target();
                }
            }
        }
    }

    pub fn draw_rectangle(&self, rectangle: &Rectangle) {
        let transform = Matrix3x2::translation(rectangle.x, rectangle.y);
        let rect = D2D_RECT_F {
            right: rectangle.w,
            bottom: rectangle.h,
            ..Default::default()
        };
        if let Some(ctx) = self.target.as_ref() {
            unsafe { ctx.SetTransform(&(transform * self.transform)) };
            if let Some(brush) = self.brush.as_ref() {
                unsafe { ctx.FillRectangle(&rect, brush) }
            }
        }
    }

    pub fn draw_text(&self, text: &Text) {
        let format = match text.size {
            TextSize::Tiny => &self.tiny_text_format,
            TextSize::Small => &self.small_text_format,
            TextSize::Medium => &self.medium_text_format,
            TextSize::Big => &self.big_text_format,
        };
        if let Some(ctx) = self.target.as_ref() {
            let size = get_window_size(unsafe { ctx.GetHwnd() });
            let offset = get_aspect_offset(&size);
            let transform = Matrix3x2::translation(
                offset.X + text.x * (size.width as f32 - offset.X * 2.0),
                offset.Y + text.y * (size.height as f32 - offset.Y * 2.0),
            );
            unsafe { ctx.SetTransform(&transform) };
            if let Some(brush) = self.brush.as_ref() {
                unsafe {
                    ctx.DrawText(
                        &text.text,
                        format,
                        &D2D_RECT_F::default(),
                        brush,
                        D2D1_DRAW_TEXT_OPTIONS_NONE,
                        DWRITE_MEASURING_MODE_NATURAL,
                    )
                }
            }
        }
    }

    /// Resize the graphics by changing the size of the render target.
    pub fn resize(&mut self) {
        if self.target.is_some() {
            self.transform = create_aspect_transform(self.hwnd);
            if let Some(ctx) = self.target.as_ref() {
                let hwnd = unsafe { ctx.GetHwnd() };
                let size = get_window_size(hwnd);
                unsafe { ctx.Resize(&size).unwrap() }
            }
            self.rebuild_text_formats();
        }
    }

    /// Create rendering target and related items. This function should be used
    /// during the first draw or when the render target should be re-created.
    fn create_target(&mut self) -> Result<()> {
        unsafe {
            let target = self.factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: self.hwnd,
                    pixelSize: get_window_size(self.hwnd),
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

    /// Rebuild the text formats based on the current window size.
    fn rebuild_text_formats(&mut self) {
        if let Some(ctx) = self.target.as_ref() {
            let size = get_window_size(unsafe { ctx.GetHwnd() });
            let offset = get_aspect_offset(&size);
            let scalar = size.height as f32 - offset.Y * 2.0;

            self.tiny_text_format = create_text_format(0.025 * scalar);
            self.small_text_format = create_text_format(0.05 * scalar);
            self.medium_text_format = create_text_format(0.1 * scalar);
            self.big_text_format = create_text_format(0.2 * scalar);
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
fn get_window_size(hwnd: HWND) -> D2D_SIZE_U {
    let mut rect = windows::Win32::Foundation::RECT::default();
    unsafe { GetClientRect(hwnd, &mut rect) };
    D2D_SIZE_U {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    }
}

/// Get the aspect ratio specific offset for the given window size.
fn get_aspect_offset(size: &D2D_SIZE_U) -> Vector2 {
    let mut result = Vector2::default();
    let x = size.width as f32;
    let y = size.height as f32;
    let aspect = x / y;
    if (aspect - ASPECT).abs() > 0.0 {
        if aspect > ASPECT {
            result.X = (x - y * ASPECT) / 2.0;
        } else {
            result.Y = (y - x / ASPECT) / 2.0;
        }
    }
    result
}

/// Create a transform matrix for the given window based on the static aspect ratio.
fn create_aspect_transform(hwnd: HWND) -> Matrix3x2 {
    let size = get_window_size(hwnd);
    let offset = get_aspect_offset(&size);
    let translation = Matrix3x2::translation(offset.X, offset.Y);
    let scale = Matrix3x2 {
        M11: (size.width as f32 - offset.X * 2.0),
        M22: (size.height as f32 - offset.Y * 2.0),
        ..Default::default()
    };
    scale * translation
}

/// Create the big text format used to draw large texts on the buffer.
fn create_text_format(size: f32) -> IDWriteTextFormat {
    unsafe {
        let factory: IDWriteFactory3 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap();
        let text_format = factory
            .CreateTextFormat(
                w!("Calibri"),
                None,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                size,
                w!("en-us"),
            )
            .unwrap();
        text_format
            .SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)
            .unwrap();
        text_format
            .SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)
            .unwrap();
        text_format
            .SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP)
            .unwrap();
        text_format
    }
}
