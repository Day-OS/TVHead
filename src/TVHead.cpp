#include "pico/stdlib.h"
#include "PicoLed.hpp"
#include "data.c"
#include "string.h"
#include "iostream"
#include "VTNCRW/src/lib.h"
#define LED_LENGTH 20

int main() {
    VTNCRW VTNCLib;
    std::vector<unsigned char> fileToBeLoaded(prototipo_vtnc, prototipo_vtnc + prototipo_vtnc_size);
    VTNCFile file = VTNCLib.read(fileToBeLoaded);

    stdio_init_all();
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    gpio_put(LED_PIN, 1);

    PicoLed::PicoLedController ledStrip = PicoLed::addLeds<PicoLed::WS2812B>(pio0, 0, 0, LED_LENGTH, PicoLed::FORMAT_GRB);
    ledStrip.setBrightness(64);
    


    while (true) {

        for (size_t frame = 0; frame < file.framesQuantity; frame++)
        {
            for (size_t i = 0; i < file.layersResolution[0].x * file.layersResolution[0].y; i++)
            {
                RGB COLOR = file.Colors[file.Layers->framesArray[frame].Pixels[i]];
                ledStrip.setPixelColor(i,  PicoLed::RGB(COLOR.R, COLOR.G, COLOR.B));
                

            }
        ledStrip.show();
        sleep_ms(100);
        //file.Layers->framesArray->msDuration
        }

        printf("seeeeeeeeeeeeeeeekkusu\n");
        

        std::cout << std::endl << "FILE: Is .VTNC? R: " << std::boolalpha <<  file.isFile;
        std::cout << std::endl << "FILE: Layers Quantity: " << int(file.layersQuantity);
        //std::cout << std::endl << "FILE: Layer Keys: " << int(file.Layers[0].layerKey);
        std::cout << std::endl << "FILE: Layers Resolution: "<< std::dec << file.layersResolution[0].x << "x" << file.layersResolution[0].y;
        std::cout << std::endl << "FILE: Colors Quantity: " << int(file.colorsQuantity);
        std::cout << std::endl << "FILE: Colors: " << std::hex << int(file.Colors[1].R) << "|" << int(file.Colors[1].G) << "|" << int(file.Colors[1].B);
        std::cout << std::endl << "FILE: Frames Quantity: " << int(file.framesQuantity);
        std::cout << std::endl << "FILE: Frames Array: " << int(file.Layers->framesArray->Pixels[0]);
        
        //sleep_ms(1000);
    }
    return 0;
}