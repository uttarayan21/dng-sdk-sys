#[cxx::bridge]
pub mod dng_host {
    unsafe extern "C++" {
        include!("dng_host.h");
        type DngHost;
    }
}
