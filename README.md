# TVHead

As it uses a Raspberry Pi Pico as the brain, most of the content researched for it came from an [official documentation]([https://link](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)).

####Compiling on Windows 10
Use the VS2019 CLI for compiling it (Might work with older versions)
Original Hardware             |  Animation Test | Creeper Face
:-------------------------:|:-------------------------:|:-------------------------:
![](https://cdn.discordapp.com/attachments/650490503439056908/1078367105650540554/IMG_20211201_152809.jpg)  |  ![](https://cdn.discordapp.com/attachments/650490503439056908/1078367106241921104/20220301_022326.jpg) | ![](https://media.discordapp.net/attachments/650490503439056908/1078367106783002695/20220302_033851.jpg)


Command:
```sh
    cd /build
    cmake -G "NMake Makefiles" ..
    nmake
```
