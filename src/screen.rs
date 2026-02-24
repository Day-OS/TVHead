use core::result::Result;
use core::iter::IntoIterator;
use embedded_graphics::prelude::*;
use embedded_graphics::Pixel;
use embedded_graphics::pixelcolor::RgbColor;
use crate::LEDS;

pub struct WsScreen<'a, C: PixelColor + RgbColor> {
    pub buffer: &'a mut [smart_leds::RGB<u8>],
    pub width: u32,
    pub height: u32,
    pub zigzag: bool,
    pub invert_v: bool,
    pub invert_h: bool,
    _color: core::marker::PhantomData<C>,
}

impl<'a, C: PixelColor + RgbColor> WsScreen<'a, C> {
    pub fn new(buffer: &'a mut [smart_leds::RGB<u8>], width: u32, height: u32, zigzag: Option<bool>, invert_v: Option<bool>, invert_h: Option<bool>) -> Self {
        Self { buffer, width, height, zigzag: zigzag.unwrap_or(true), invert_v: invert_v.unwrap_or(false), invert_h: invert_h.unwrap_or(false), _color: core::marker::PhantomData }
    }
}

impl<'a, C: PixelColor + RgbColor> DrawTarget for WsScreen<'a, C> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<C>>,
    {
        for Pixel(point, color) in pixels {
            if point.x < 0 || point.y < 0 {
        continue;
    }
            let mut x = point.x as u32;
            let mut y = point.y as u32;
            if self.invert_h {
                x = self.width - 1 - x;
            }
            if self.invert_v {
                y = self.height - 1 - y;
            }
            let idx = if self.zigzag && (y % 2 == 1) {
                (y * self.width + (self.width - 1 - x)) as usize
            } else {
                (y * self.width + x) as usize
            };
            if idx < LEDS {
                self.buffer[idx] = smart_leds::RGB { r: color.r(), g: color.g(), b: color.b(), };
            }
        }
        Ok(())
    }
}

impl<'a, C: PixelColor + RgbColor> OriginDimensions for WsScreen<'a, C> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
