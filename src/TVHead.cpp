#include "pico/stdlib.h"
#include "PicoLed.hpp"
#include "string.h"
#include "iostream"
#include "VTNCRW/src/vtncrw.h"

//REMINDER: "data.c" is generated when CMake gets all the files from "assets" directory and transforms it into a char array!
#include "data.c"

#define width 17
#define height 11

int main() {
    VTNCRW VTNCLib;
    std::vector<unsigned char> fileToBeLoaded(totallynotamongus_vtnc, totallynotamongus_vtnc + totallynotamongus_vtnc_size);
    VTNCFile file = VTNCLib.read(fileToBeLoaded);

    stdio_init_all();
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    gpio_put(LED_PIN, 1);

    PicoLed::PicoLedController ledStrip = PicoLed::addLeds<PicoLed::WS2812B>(pio0, 0, 0, width*height, PicoLed::FORMAT_GRB);
    ledStrip.setBrightness(64);

    bool inverter = false;
    int xres = file.layersResolution[0].x;
    int yres = file.layersResolution[0].y;
    while (true) {

        for (size_t frame = 0; frame < file.framesQuantity; frame++)
        {
            for (size_t y = 0; y < yres; y++)
            {
                for (size_t x = 0; x < xres; x++)
                {
                    int linesoffset = y*xres;
                    int currentIndex = !inverter? (xres - x) + linesoffset - 1: x + linesoffset;
                    std::cout << y <<": " << currentIndex << " | " <<(x + linesoffset) << std::endl;
                    RGBA COLOR = file.Colors[file.Layers->framesArray[frame].Pixels[currentIndex]];
                    ledStrip.setPixelColor(x + linesoffset,  PicoLed::RGB(COLOR.R, COLOR.G, COLOR.B));
                }
                inverter = !inverter;
            }
            inverter = false;
        ledStrip.show();
        sleep_ms(file.Layers[0].framesArray[frame].msDuration);
        }
    }
    return 0;
}