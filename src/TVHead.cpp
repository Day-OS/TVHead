
#include "pico/stdlib.h"
#include "PicoLed.hpp"
#include "data.c"
#include "gifdec/gifdec.h"
#include "string.h"
#define LED_LENGTH 21


int main() {
    stdio_init_all();
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    gpio_put(LED_PIN, 1);

    while (true) {
        printf("seeeeeeeeeeeeeeeekkusu\n");
        printf("canvas size: %ux%u\n", gif->width, gif->height);
        printf("number of colors: %d\n", gif->palette->size);
        //printf(strcat((char*) onepixelheight_gif, (char*)"\n"));
        sleep_ms(1000);
    }
    return 0;
}


/*


  
int main() {

    PicoLed::PicoLedController ledStrip = PicoLed::addLeds<PicoLed::WS2812B>(pio0, 0, 0, LED_LENGTH, PicoLed::FORMAT_RGB);
    //ledStrip.setPixelColor(0, PicoLed::Color(2))
    ledStrip.setBrightness(64);
    //ledStrip.fill( PicoLed::RGB(255, 0, 0) );
    //ledStrip.fillRainbow(0,255/LED_LENGTH);

    try
    {
        
        
        uint8_t *buffer;
        gd_render_frame(gif, buffer);  
        gd_get_frame(gif);

        
        uint8_t *color = buffer;
        for (int y = 0; y < gif->height; y++) {
            for (int x = 0; x < gif->width; x++) {
                ledStrip.setPixelColor(x,  PicoLed::RGB(255, 0, 0));
                ledStrip.show();
                if (gd_is_bgcolor(gif, color))
                    printf("aaa");
                color += 3;
            }
        }

    }
    catch(const std::exception& e){

        while (true){
            //printf(e);
            sleep_ms(100);
            
            sleep_ms(200);
            gpio_put(LED_PIN, 0);
        }
    }
    
    
    
    
    //printf((const char*)oi_txt);
   
    
 
    printf("AAAAAAAAAAAAAAAAAAAAAAAA\n");
}
*/