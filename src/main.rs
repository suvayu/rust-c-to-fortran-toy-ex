extern crate libc;

use std::env;
use std::ffi::CString;

extern "C" {
    // fn area(a: &f32, b: &f32) -> f32;
    fn area(
        name: *const libc::c_char,
        name_length: libc::size_t,
        a: libc::c_float,
        b: libc::c_float,
    ) -> f32;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Missing name");
        }
        2 => {
            let name = CString::new(args[1].as_str()).unwrap();
            let a: f32 = 3.0;
            let b: f32 = 4.0;
            let res = unsafe {
                area(
                    name.as_ptr(),
                    name.into_string().expect("into_string() failed").len(),
                    a,
                    b,
                )
            };
            println!("area: {}", res);
        }
        _ => {
            println!("Usage: {} <name>", args[0]);
            return;
        }
    }
}
