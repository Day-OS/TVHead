#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use alloc::vec;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::peripherals::LEDC;
use esp_hal::rmt::Rmt;
use esp_hal::time::{Instant, Rate};
use esp_hal_smartled::SmartLedsAdapter;
use log::info;
use smart_leds::{RGB8, SmartLedsWrite};
use smart_leds::brightness;
use smart_leds;
use esp_hal_smartled::smart_led_buffer;
use esp_backtrace as _;
use smart_leds::{ gamma,
    hsv::{Hsv, hsv2rgb},
};


extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    info!("Initial Setup");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
  
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(size: 64 * 1024);

    info!("Screen setup");


    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to initialize RMT0");

    let rmt_channel = rmt.channel0;
    let mut rmt_buffer = smart_led_buffer!(151);

    let mut led = SmartLedsAdapter::new(rmt_channel, peripherals.GPIO33, &mut rmt_buffer);

    let delay = Delay::new();

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data: RGB8;
    let level = 255;



    // Maybe it doesnt work in wokwi?    // let (mut _wifi_controller, _interfaces) =
    //     esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
    //         .expect("Failed to initialize Wi-Fi controller");
    
    // let _connector = BleConnector::new(&radio_init, peripherals.BT, Default::default());


    loop {
        for hue in 0..=255 {
            color.hue = hue;
            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            // data = hsv2rgb(color);
            data = RGB8::new(255, 0, 0);
            // When sending to the LED, we do a gamma correction first (see smart_leds docs
            // for details <https://docs.rs/smart-leds/latest/smart_leds/struct.Gamma.html>)
            // and then limit the brightness level to 10 out of 255 so that the output
            // is not too bright.
            let mut colors= vec![];

            for _ in 1..151 {
                colors.push(data);
            }

            led.write(brightness(gamma(colors.into_iter()), level))
                .unwrap();
            delay.delay_millis(20);
        }
        info!("Hello world!");
        // let delay_start = Instant::now();
        // while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
