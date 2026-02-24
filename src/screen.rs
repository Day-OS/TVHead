use core::result::Result;
use core::iter::IntoIterator;
use embedded_graphics::prelude::*;
use embedded_graphics::Pixel;
use crate::LEDS;

pub struct PixelCollector<'a, C: PixelColor> {
    pub pixels: &'a mut [Pixel<C>],
    pub len: usize,
}

impl<'a, C: PixelColor> PixelCollector<'a, C> {
    pub fn new(buf: &'a mut [Pixel<C>]) -> Self {
        Self { pixels: buf, len: 0 }
    }
}

impl<'a, C: PixelColor> DrawTarget for PixelCollector<'a, C> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<C>>,
    {
        for p in pixels {
            if self.len < self.pixels.len() {
                self.pixels[self.len] = p;
                self.len += 1;
            }
        }
        Ok(())
    }
}

impl<'a, C: PixelColor> OriginDimensions for PixelCollector<'a, C> {
    fn size(&self) -> Size {
        Size::new(0, 0)
    }
}
