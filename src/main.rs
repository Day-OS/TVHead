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

use embedded_graphics::{image::{ImageDrawable}, pixelcolor::Rgb888};
use esp_backtrace as _;
use esp_hal::{delay::Delay, rmt::Rmt, time::Rate};
use esp_hal_smartled::{RmtSmartLeds, Ws2812bTiming, buffer_size, color_order};
use smart_leds::{
    RGB8, SmartLedsWrite, brightness, gamma,
};


use crate::screen::WsScreen;
mod screen;
esp_bootloader_esp_idf::esp_app_desc!();


const WIDTH: u32 = 17;
const HEIGHT: u32 = 9;
const LEDS: usize = (WIDTH * HEIGHT) as usize;

#[esp_hal::main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Each devkit uses a unique GPIO for the RGB LED, so in order to support
    // all chips we must unfortunately use `#[cfg]`s:
    let led_pin = peripherals.GPIO33;

    // Configure RMT peripheral globally
    let freq = Rate::from_mhz(80);
    let mut led: RmtSmartLeds<'_, _, esp_hal::Blocking, smart_leds::RGB<u8>, color_order::Rgb, Ws2812bTiming> = {
        let rmt = Rmt::new(peripherals.RMT, freq).expect("Failed to initialize RMT0");
        // Configure color order and timing implementation as needed.
        RmtSmartLeds::<{ buffer_size::<RGB8>(LEDS) }, _, RGB8, color_order::Rgb, Ws2812bTiming>::new_with_memsize(
            rmt.channel0,
            led_pin,
            3,
        ).unwrap()
    };
    let delay = Delay::new();

    let image = tinygif::Gif::<Rgb888>::from_slice(include_bytes!("../content/smile.gif")).unwrap();

    let mut buf: [smart_leds::RGB<u8>; LEDS] = [Default::default(); LEDS]; // ajuste pro seu caso
    let mut screen = WsScreen::new(
        &mut buf,
        WIDTH,
        HEIGHT,
        Some(true),  // zigzag
        Some(true), // invert_v
        Some(false), // invert_h
    );

    loop {
        for frame in image.frames() {

            frame.draw(&mut screen).unwrap();

            let delay_ms = frame.delay_centis * 10;
            
            led.write(brightness(gamma(screen.buffer.iter().cloned()), 255))
                .unwrap();
            delay.delay_millis(delay_ms as u32);           
        }
    }
}