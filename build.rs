fn main() {
    for lib in ["NiFpga",
                "NiFpgaLv"].iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib);
                }
    println!("cargo:rustc-link-search=native=lib/ni-libraries");
}
