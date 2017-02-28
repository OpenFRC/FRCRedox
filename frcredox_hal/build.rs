use std::env;

extern crate gcc;

fn main() {
    //gcc::compile_library("../lib/ni-wrapper/libNiWrapper.so", &["../cpp/"])
    if env::var("TARGET").unwrap() == "arm-unknown-linux-gnueabi" {
        for lib in ["NiFpga",
                    "NiFpgaLv"].iter()
        {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
        println!("cargo:rustc-link-search=native=lib/ni-libraries");
    }
}
