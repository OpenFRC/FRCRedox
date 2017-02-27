pub mod ni;
fn main() {
    let test: ni::fpga::fpga_types::NiFpga_Bool = ni::fpga::fpga_constants::NiFpga_False;
    let s:String = test.to_string();
    println!("{}",s);
}
