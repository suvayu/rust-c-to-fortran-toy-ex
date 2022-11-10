extern crate libc;

use std::env;
use std::ffi::CString;

extern "C" {
    fn area(
        name: *const libc::c_char,
        name_length: libc::size_t,
        a: libc::c_float, // f32
        b: libc::c_float, // f32
    ) -> f32;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        4 => {
            let name = CString::new(args[1].as_str()).unwrap();
            let a: f32 = args[2].parse::<f32>().unwrap();
            let b: f32 = args[3].parse::<f32>().unwrap();
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
            println!("Usage: {} <name> <side1> <side2>", args[0]);
            return;
        }
    }
}
