mod dng_sdk;
pub fn main() -> anyhow::Result<()> {
    dng_sdk::build()?;
    libjxl_src::build();
    Ok(())
}
