extern crate libc;

#[macro_use] extern crate rocket;

use std::ffi::CString;

extern "C" {
    fn area(
        name: *const libc::c_char,
        name_length: libc::size_t,
        a: libc::c_float, // f32
        b: libc::c_float, // f32
    ) -> f32;
}

#[get("/getarea/<name>/<a>/<b>")]
fn getarea(name: &str, a: f32, b: f32) -> String {
    let name = CString::new(name).unwrap();
    let res = unsafe {
        area(
            name.as_ptr(),
            name.into_string().expect("into_string() failed").len(),
            a,
            b
        )
    };
    format!("{name}, area: {res}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![getarea])
}
