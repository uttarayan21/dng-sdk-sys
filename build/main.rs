mod dng_sdk;
pub fn main() -> anyhow::Result<()> {
    dng_sdk::build()?;
    pkg_config::Config::new().statik(true).probe("libjxl_threads").unwrap();
    pkg_config::Config::new().statik(true).probe("libjxl").unwrap();
    pkg_config::Config::new().statik(true).probe("zlib").unwrap();
    // libjxl_src::build();
    // libjxl_src::print_cargo_link();
    Ok(())
}
