extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/triangle.f90")
        .compile("triangle");
    println!("cargo:rustc-link-lib=gfortran");
}
