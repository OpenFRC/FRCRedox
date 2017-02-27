use std::env;

fn main() {
    if env::var("TARGET").unwrap() == "arm-unknown-linux-gnueabi" {
        for lib in ["NiFpga",
                    "NiFpgaLv"].iter()
        {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
        println!("cargo:rustc-link-search=native=lib/ni-libraries");
    }
}
