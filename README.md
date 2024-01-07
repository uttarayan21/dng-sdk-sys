# Adobe DNG Sdk

Rust build system for building and linking to adobe dng sdk and xmp-toolkit

This crates compiles adobe dng_sdk and links all the required libraries.
Only tested on macos and linux ( Will add windows support later )

You can find the source to dng_sdk [here](https://www.adobe.com/go/dng_sdk)

TODO: 
- Write bindings to the dng_sdk in rust.
- Download dng_sdk automatically instead of bundling it with the source code.
- Windows support
- Link to libjxl ( compile libjxl and link using rust build.rs not cmake )
