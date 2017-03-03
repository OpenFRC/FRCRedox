// To Keep consistent with NiFpga
pub type int8_t = i8;
pub type uint8_t = u8;

pub type int16_t = i16;
pub type uint16_t = u16;

pub type int32_t = i32;
pub type uint32_t = u32;

pub type int64_t = i64;
pub type uint64_t = u64;

pub type size_t = uint64_t;

//pub type c_string = *c_char;

// NiFpga Types. Will be cleaned up later
pub type NiFpga_Bool = uint8_t;
pub type NiFpga_Status = int32_t;

pub type NiFpga_Session = uint32_t;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NiFpga_OpenAttribute {
    NiFpga_OpenAttribute_NoRun = 1,
    Dummy
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NiFpga_CloseAttribute {
    NiFpga_CloseAttribute_NoResetIfLastSession = 1,
    Dummy
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum NiFpga_RunAttribute {
    NiFpga_RunAttribute_WaitUntilDone = 1,
    Dummy
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[cfg_attr(feature = "cargo-clippy", allow(identity_op))]
pub enum NiFpga_Irq {
    NiFpga_Irq_0  = 1 << 0,
    NiFpga_Irq_1  = 1 << 1,
    NiFpga_Irq_2  = 1 << 2,
    NiFpga_Irq_3  = 1 << 3,
    NiFpga_Irq_4  = 1 << 4,
    NiFpga_Irq_5  = 1 << 5,
    NiFpga_Irq_6  = 1 << 6,
    NiFpga_Irq_7  = 1 << 7,
    NiFpga_Irq_8  = 1 << 8,
    NiFpga_Irq_9  = 1 << 9,
    NiFpga_Irq_10 = 1 << 10,
    NiFpga_Irq_11 = 1 << 11,
    NiFpga_Irq_12 = 1 << 12,
    NiFpga_Irq_13 = 1 << 13,
    NiFpga_Irq_14 = 1 << 14,
    NiFpga_Irq_15 = 1 << 15,
    NiFpga_Irq_16 = 1 << 16,
    NiFpga_Irq_17 = 1 << 17,
    NiFpga_Irq_18 = 1 << 18,
    NiFpga_Irq_19 = 1 << 19,
    NiFpga_Irq_20 = 1 << 20,
    NiFpga_Irq_21 = 1 << 21,
    NiFpga_Irq_22 = 1 << 22,
    NiFpga_Irq_23 = 1 << 23,
    NiFpga_Irq_24 = 1 << 24,
    NiFpga_Irq_25 = 1 << 25,
    NiFpga_Irq_26 = 1 << 26,
    NiFpga_Irq_27 = 1 << 27,
    NiFpga_Irq_28 = 1 << 28,
    NiFpga_Irq_29 = 1 << 29,
    NiFpga_Irq_30 = 1 << 30,
    NiFpga_Irq_31 = 1 << 31
}

type NiFpga_IrqContext = *mut ::std::os::raw::c_void;
