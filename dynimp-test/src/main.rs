#![allow(non_snake_case)]
#[macro_use]
extern crate dynimp_macros;

#[dy_fn(name = "RtlGetCurrentProcessorNumber")]
pub fn rtl_get_current_processor_number() -> u64 {}

#[dy_fn(name = "LoadLibraryA")]
pub fn load_library_a(name: *const u8) -> u64 {}

#[dy_fn]
pub fn MessageBoxA(fh: u32, text: *const u8, capt: *const u8, btn: u32) -> u64 {}

fn main() {
    unsafe {
        load_library_a("user32.dll\0".as_ptr());

        MessageBoxA(0, "Hello,World\0".as_ptr(), "XXX\0".as_ptr(), 0);

        loop {
            println!(
                "rtl_get_current_processor_number : {:#?}",
                rtl_get_current_processor_number()
            );
            std::thread::sleep(std::time::Duration::from_millis(111));
        }
    }
}
