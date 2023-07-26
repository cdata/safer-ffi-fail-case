use safer_ffi::prelude::*;

#[ffi_export]
const SOME_CONSTANT: u32 = 0;

#[ffi_export]
pub fn foo() {}

#[cfg(feature = "headers")]
pub fn generate_headers() -> std::io::Result<()> {
    safer_ffi::headers::builder().to_file("tmp.h")?.generate()
}

fn main() {
    println!("Hello, world!");
}
