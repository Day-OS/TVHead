# TVHead

As it uses a Raspberry Pi Pico as the brain, most of the content researched for the usaged came from an [official documentation]([https://link](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)).

####Compiling on Windows 10
Use the VS2019 CLI for compiling it (Might work with older versions)

Command:
```sh
    cd /build
    cmake -G "NMake Makefiles" ..
    nmake
```