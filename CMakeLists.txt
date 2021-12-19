cmake_minimum_required(VERSION 3.13)

#from Youka: https://stackoverflow.com/questions/11813271/embed-resources-eg-shader-code-images-into-executable-library-with-cmake
# Creates C resources file from files in given directory
function(create_resources dir output)
    # Create empty output file
    file(WRITE ${output} "")
    # Collect input files
    file(GLOB bins ${dir}/*)
    # Iterate through input files
    foreach(bin ${bins})
        # Get short filename
        string(REGEX MATCH "([^/]+)$" filename ${bin})
        # Replace filename spaces & extension separator for C compatibility
        string(REGEX REPLACE "\\.| |-" "_" filename ${filename})
        # Read hex data from file
        file(READ ${bin} filedata HEX)
        # Convert hex data for C compatibility
        string(REGEX REPLACE "([0-9a-f][0-9a-f])" "0x\\1," filedata ${filedata})
        # Append data to output file
        file(APPEND ${output} "const unsigned char ${filename}[] = {${filedata}};\nconst unsigned ${filename}_size = sizeof(${filename});\n")
    endforeach()
endfunction()

create_resources("assets/" "src/data.c")


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
add_subdirectory(include/gifdec)
# Add executable. Default name is the project name, version 0.1

add_executable(TVHead src/TVHead.cpp)


target_include_directories(TVHead PUBLIC include)

pico_set_program_name(TVHead "TVHead")
pico_set_program_version(TVHead "0.1")

# no_flash means the target is to run from RAM
#pico_set_binary_type(TVHead no_flash)


pico_enable_stdio_uart(TVHead 0)
pico_enable_stdio_usb(TVHead 1)

# Add the standard library to the build
target_link_libraries(TVHead pico_stdlib PicoLed GIFDec)

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

