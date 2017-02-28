#[macro_use]
extern crate error_chain;
extern crate futures;

pub mod hal;
#[cfg(target="arm-unknown-linux-gnueabi")]
pub mod ni;

#[cfg(target="arm-unknown-linux-gnueabi")]
pub use ni::*;
pub use hal::*;
