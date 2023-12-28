use dng_sdk_sys::dng_read;
fn main() {
    let res = unsafe { dng_read() };
    dbg!(res);
}
