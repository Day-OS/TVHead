# Generated Cmake Pico project file

cmake_minimum_required(VERSION 3.13)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# initalize pico_sdk from installed location
# (note this can come from environment, CMake cache etc)
set(PICO_SDK_PATH "C:/Users/danie/Documents/Pico/pico-sdk")

# Pull in Raspberry Pi Pico SDK (must be before project)
include(pico_sdk_import.cmake)

project(TVHead C CXX ASM)

set(PICO_CXX_ENABLE_EXCEPTIONS 1)

set(PICO_CXX_ENABLE_RTTI 1)

# Initialise the Raspberry Pi Pico SDK
pico_sdk_init()

include("include/PicoLed/PicoLed.cmake")

# Add executable. Default name is the project name, version 0.1

add_executable(TVHead src/TVHead.cpp)

pico_set_program_name(TVHead "TVHead")
pico_set_program_version(TVHead "0.1")

# no_flash means the target is to run from RAM
#pico_set_binary_type(TVHead no_flash)


pico_enable_stdio_uart(TVHead 0)
pico_enable_stdio_usb(TVHead 1)

# Add the standard library to the build
target_link_libraries(TVHead pico_stdlib PicoLed)

# Add any user requested libraries
target_link_libraries(TVHead
        hardware_spi
        hardware_i2c
        hardware_dma
        hardware_pio
        hardware_interp
        hardware_timer
        hardware_watchdog
        hardware_clocks
        )

pico_add_extra_outputs(TVHead)

