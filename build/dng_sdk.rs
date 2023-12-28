pub fn build() -> anyhow::Result<()> {
    let files = [
        "source/dng_1d_function.cpp",
        "source/dng_1d_table.cpp",
        "source/dng_abort_sniffer.cpp",
        "source/dng_area_task.cpp",
        "source/dng_bad_pixels.cpp",
        "source/dng_big_table.cpp",
        "source/dng_bmff.cpp",
        "source/dng_bottlenecks.cpp",
        "source/dng_camera_profile.cpp",
        "source/dng_color_space.cpp",
        "source/dng_color_spec.cpp",
        "source/dng_date_time.cpp",
        "source/dng_exceptions.cpp",
        "source/dng_exif.cpp",
        "source/dng_file_stream.cpp",
        "source/dng_filter_task.cpp",
        "source/dng_fingerprint.cpp",
        "source/dng_gain_map.cpp",
        "source/dng_globals.cpp",
        "source/dng_host.cpp",
        "source/dng_hue_sat_map.cpp",
        "source/dng_ifd.cpp",
        "source/dng_image.cpp",
        "source/dng_image_writer.cpp",
        "source/dng_info.cpp",
        "source/dng_iptc.cpp",
        "source/dng_jpeg_image.cpp",
        "source/dng_jpeg_memory_source.cpp",
        "source/dng_jxl.cpp",
        "source/dng_lens_correction.cpp",
        "source/dng_linearization_info.cpp",
        "source/dng_local_string.cpp",
        "source/dng_lossless_jpeg.cpp",
        "source/dng_lossless_jpeg_shared.cpp",
        "source/dng_matrix.cpp",
        "source/dng_memory.cpp",
        "source/dng_memory_stream.cpp",
        "source/dng_misc_opcodes.cpp",
        "source/dng_mosaic_info.cpp",
        "source/dng_mutex.cpp",
        "source/dng_negative.cpp",
        "source/dng_opcode_list.cpp",
        "source/dng_opcodes.cpp",
        "source/dng_orientation.cpp",
        "source/dng_parse_utils.cpp",
        "source/dng_pixel_buffer.cpp",
        "source/dng_point.cpp",
        "source/dng_preview.cpp",
        "source/dng_pthread.cpp",
        "source/dng_rational.cpp",
        "source/dng_read_image.cpp",
        "source/dng_rect.cpp",
        "source/dng_ref_counted_block.cpp",
        "source/dng_reference.cpp",
        "source/dng_render.cpp",
        "source/dng_resample.cpp",
        "source/dng_safe_arithmetic.cpp",
        "source/dng_shared.cpp",
        "source/dng_simple_image.cpp",
        "source/dng_spline.cpp",
        "source/dng_stream.cpp",
        "source/dng_string.cpp",
        "source/dng_string_list.cpp",
        "source/dng_tag_types.cpp",
        "source/dng_temperature.cpp",
        "source/dng_tile_iterator.cpp",
        "source/dng_tone_curve.cpp",
        "source/dng_update_meta.cpp",
        "source/dng_utils.cpp",
        // "source/dng_validate.cpp",
        "source/dng_xmp.cpp",
        "source/dng_xmp_sdk.cpp",
        "source/dng_xy_coord.cpp",
    ];

    let folder = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let vendor = std::path::Path::new(&folder).join("vendor");
    let dng_sdk = vendor.join("dng_sdk");
    let xmp_include =
        std::env::var("DEP_XMP_TOOLKIT_INCLUDE").expect("Failed to get includes from xmp");

    let includes = [
        xmp_include.into(),
        vendor
            .join("libjxl")
            .join("libjxl")
            .join("lib")
            .join("include"),
        vendor
            .join("libjxl")
            .join("client_projects")
            .join("include"),
        dng_sdk.join("source"),
    ];
    let platform = Platform::from_env();

    let mut build = cc::Build::new();
    build.cpp(true);
    build.includes(includes);
    build.files(files.iter().map(|f| dng_sdk.join(f)));
    build.file("dng/reader.cpp");
    build.define(platform.define(), None);
    // build.define("qDNGUseXMP", None);
    // build.define("qDNGSupportJXL", None);
    build.define("TXMP_STRING_TYPE", "std::string");
    build.define("XML_STATIC", "1");
    build.define("XMP_StaticBuild", "1");
    build.define("HAVE_EXPAT_CONFIG_H", "1");
    build.define("qDNGXMPDocOps", "0");
    build.std("c++11");
    build.static_flag(true);
    build.static_crt(true);
    build.compile("dng");
    println!("cargo:rustc-link-lib=static=dng");

    Ok(())
}

#[derive(Debug)]
pub enum Platform {
    Mac,
    Win,
    Android,
    Linux,
    IPhone,
    Web,
}

impl Platform {
    pub fn from_env() -> Self {
        let target = std::env::var("TARGET").expect("CARGO_BUILD_TARGET not set");
        if target.contains("darwin") {
            Self::Mac
        } else if target.contains("windows") {
            Self::Win
        } else if target.contains("android") {
            Self::Android
        } else if target.contains("linux") {
            Self::Linux
        } else if target.contains("ios") {
            Self::IPhone
        } else if target.contains("wasm") {
            Self::Web
        } else {
            panic!("Unknown target: {}", target);
        }
    }

    pub fn define(&self) -> &'static str {
        use Platform::*;
        match self {
            Mac => "qMacOS",
            Win => "qWinOS",
            Android => "qAndroid",
            Linux => "qLinux",
            IPhone => "qiPhone",
            Web => "qWeb",
        }
    }
}
