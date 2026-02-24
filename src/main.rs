//! RGB LED Demo
//!
//! This example drives an SK68XX RGB LED, which is connected to a pin on the
//! official DevKits.
//!
//! The demo will leverage the [`smart_leds`](https://crates.io/crates/smart-leds)
//! crate functionality to circle through the HSV hue color space (with
//! saturation and value both at 255). Additionally, we apply a gamma correction
//! and limit the brightness to 10 (out of 255).
//!
//! The following wiring is assumed for ESP32:
//! - LED => GPIO33
//! The following wiring is assumed for ESP32C3:
//! - LED => GPIO8
//! The following wiring is assumed for ESP32C6, ESP32H2:
//! - LED => GPIO8
//! The following wiring is assumed for ESP32S2:
//! - LED => GPIO18
//! The following wiring is assumed for ESP32S3:
//! - LED => GPIO48
//!
//! You might need to adjust the color order and timing types during the [`RmtSmartLeds`] initialization,
//! depending on what your board exactly has.

//% CHIPS: esp32 esp32c3 esp32c6 esp32h2 esp32s2 esp32s3

#![no_std]
#![no_main]

use embedded_graphics::{Pixel, image::{ImageDrawable, ImageDrawableExt}, pixelcolor::Rgb565, prelude::{Point, Size}, primitives::Rectangle};
use esp_backtrace as _;
use esp_hal::{delay::Delay, rmt::Rmt, time::Rate};
use esp_hal_smartled::{RmtSmartLeds, Ws2812Timing, buffer_size, color_order};
use esp_println::println;
use smart_leds::{
    RGB8, SmartLedsWrite, brightness, gamma,
    hsv::{Hsv, hsv2rgb},
};

use crate::screen::PixelCollector;
mod screen;
esp_bootloader_esp_idf::esp_app_desc!();

const LEDS: usize = 153;

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Each devkit uses a unique GPIO for the RGB LED, so in order to support
    // all chips we must unfortunately use `#[cfg]`s:
    let led_pin = peripherals.GPIO33;

    // Configure RMT peripheral globally
    let freq = Rate::from_mhz(80);
    type LedColor = RGB8;
    let mut led = {
        let rmt = Rmt::new(peripherals.RMT, freq).expect("Failed to initialize RMT0");
        // Configure color order and timing implementation as needed.
        RmtSmartLeds::<{ buffer_size::<LedColor>(LEDS) }, _, LedColor, color_order::Rgb, Ws2812Timing>::new_with_memsize(
            rmt.channel0,
            led_pin,
            4,
        ).unwrap()
    };
    let delay = Delay::new();

    let image = tinygif::Gif::<Rgb565>::from_slice(include_bytes!("../content/smile.gif")).unwrap();

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut buf: [Pixel<Rgb565>; LEDS] = [Pixel::default(); LEDS]; // ajuste pro seu caso
    let mut collector = PixelCollector::new(&mut buf);
    let frames = image.frames();
    for frame in image.frames(){
        frame.draw(&mut collector).unwrap();
    }
    for pixel in collector.pixels {
        println!("{pixel:?}")
    }
    let mut data: [smart_leds::RGB<u8>; LEDS] = [Default::default(); LEDS];

    loop {
        // Iterate over the rainbow!
        for hue in 0..=255 {
            color.hue = hue;
            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            for i in 0..LEDS{
                data[i] = hsv2rgb(color); 
            }
            // When sending to the LED, we do a gamma correction first (see smart_leds
            // documentation for details) and then limit the brightness to 10 out of 255 so
            // that the output it's not too bright.
            led.write(brightness(gamma(data.iter().cloned()), 255))
                .unwrap();
            delay.delay_millis(20);
        }
    }
}