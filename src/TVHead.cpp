#include "pico/stdlib.h"
#include "iostream"
#include "PicoLed.hpp"

#define LED_LENGTH 250
  
int main() {
    stdio_init_all();
    PicoLed::PicoLedController ledStrip = PicoLed::addLeds<PicoLed::WS2812B>(pio0, 0, 0, LED_LENGTH, PicoLed::FORMAT_RGB);
    ledStrip.setBrightness(64);
    //ledStrip.fill( PicoLed::RGB(255, 0, 0) );
    ledStrip.fillRainbow(0,255/LED_LENGTH);
    ledStrip.show();
 
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    gpio_put(LED_PIN, 1);
    //printf("AAAAAAAAAAAAAAAAAAAAAAAA\n");
}