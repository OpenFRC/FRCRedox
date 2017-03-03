use std::mem;

// FIXME: Come up with good names and finalize them.

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HandleType {
	Undefined     = 0,
	DIO           = 1,  // 1 bit
	Port          = 2,  // 2 bits
	Notifier      = 3,
	Interrupt     = 4,  // 3 bits
	AnalogOutput  = 5,
	AnalogInput   = 6,
	AnalogTrigger = 7,
	Relay         = 8,  // 4 bits
	PWM           = 9,
	DigitalPWM    = 10,
	Counter       = 11,
	FPGAEncoder   = 12,
	Encoder       = 13,
	Compressor    = 14,
	Solenoid      = 15,
	AnalogGyro    = 16, // 5 bits
	Vendor        = 17
}

impl HandleType {
    /// Converts a `u8` to a `HandleType`. This produces Undefined Behavior when `num > 17`.
    pub unsafe fn from_u8_unchecked(num: u8) -> HandleType {
        mem::transmute(num)
    }

    /// Converts a `u8` to a `HandleType`, returning `None` if none of the variants were matched.
    pub fn from_u8(num: u8) -> Option<HandleType> {
        if num <= 17 {
            Some(unsafe { mem::transmute(num) })
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TypeByte(u8);

impl TypeByte {
    pub fn new(byte: u8) -> Option<TypeByte> {
        if byte & !(1 << 7) <= 17 {
            unsafe { Some(mem::transmute(byte)) }
        } else {
            None
        }
    }

    /// Set the error bit
    #[inline]
    pub fn set_error_bit(&mut self, error: bool) {
        self.0 = (self.0 & !(1 << 7)) | ((error as u8) << 7)
    }

    /// Get the error bit
    #[inline]
    pub fn error_bit(&self) -> bool {
        (self.0 >> 7) & 1 != 0
    }

    pub fn get_type(&self) -> HandleType {
        unsafe { mem::transmute(self.0 & !(1 << 7)) }
    }
}

impl From<HandleType> for TypeByte {
	// Handle type never has the error bit set by itself
	fn from(handle_type: HandleType) -> TypeByte {
		unsafe { mem::transmute(handle_type) }
	}
}

// TODO: Is there a better way to structure this?
/// A sort of "protocol" for handles.
///
/// | Bits  | Meaning     | Type |
/// |-------|-------------|:----:|
/// | 31    | Error       | bool |
/// | 24-30 | Handle Type |  u8  |
/// | 16-23 | Unused      |  u8  |
/// | 0-15  | Payload     |  u16 |
///
/// ### Payload
/// The `Data` type is what type the payload should be converted into.
/// For normal handles, this is just a `u16` representing the index.
/// For port handles, this is two `u8`s, representing the module and the channel, respectively.
pub trait HandleProtocol: Sized {
    /// The data derived from the last 16 bits of the struct
    type Data;

    /// Transform the last 16 bits into whatever data type was defined
    fn to_data(&self) -> Self::Data;
}

/// Struct representing some sort of indexed data.
///
/// General Handle Data Layout
///
/// * Bits 0-15:  Handle Index
/// * Bits 16-23: Unused
/// * Bits 24-30: Handle Type
/// * Bit  31:    1 if handle error, 0 if no error
///
/// Other specialized handles will use different formats, however Bits 24-31 are
/// always reserved for type and error handling.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Handle {
    /// The type of the handle. This is the `HandleType` enum, only the top
    /// bit denotes an error status
    pub type_byte: TypeByte,
    /// 8 unused bits
    _padding: u8,
    /// TODO
    pub index: u16
}

impl Handle {
	/// Creates a new handle from its parts
	pub fn new(type_byte: HandleType, index: u16) -> Handle {
		Handle { type_byte: type_byte.into(), _padding: 0, index: index }
	}

	/// Convert an i32 to a Handle. Unsafe, as there is no associated variant of `HandleType` for `num > 17`
	pub unsafe fn from_i32_unchecked(num: i32) -> Handle {
		mem::transmute(num)
	}

    /// Convert an i32 to a Handle. Returns `None` if the type byte of `num` is out of bounds
    pub fn from_i32(num: i32) -> Option<Handle> {
        if (num >> 24) & !(1 << 7) <= 17 {
            Some(unsafe { mem::transmute(num) })
        } else {
            None
        }
    }
}

impl HandleProtocol for Handle {
    type Data = u16;

    fn to_data(&self) -> u16 {
        self.index
    }
}

impl From<Handle> for i32 {
	fn from(handle: Handle) -> i32 {
		// Our struct has the same memory layout as the i32 representation
		unsafe { mem::transmute(handle) }
	}
}

/// Struct representing some port/module on the robot
///
/// Port Handle Data Layout
///
/// * Bits 0-7:   Channel Number
/// * Bits 8-15:  Module Number
/// * Bits 16-23: Unused
/// * Bits 24-30: Handle Type
/// * Bit  31:    1 if handle error, 0 if no error
///
/// This struct is 32 bits wide.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PortHandle {
	/// The type of the handle. This is the `HandleType` enum, only the top
	/// bit denotes an error status
	pub type_byte: TypeByte,
	/// 8 unused bits
	_padding: u8,
	/// The module ID (TODO: possibly from the cRIO days?)
	pub module: u8,
	/// The channel ID
	pub channel: u8
}

impl PortHandle {
	/// Creates a new handle from its parts
	pub fn new(module: u8, channel: u8) -> PortHandle {
        // A `PortHandle` always has a handle type of `Port`
		PortHandle {
            type_byte: HandleType::Port.into(),
            _padding: 0,
            module: module,
            channel: channel
        }
	}

	/// Convert an i32 to a Handle. Unsafe, as there is no associated variant for `num > 17`
	pub unsafe fn from_i32_unchecked(num: i32) -> PortHandle {
		mem::transmute(num)
	}

    /// Convert an `i32` to a `PortHandle`
    pub fn from_i32(num: i32) -> Option<PortHandle> {
        if (num >> 24) & !(1 << 7) <= 17 {
            Some(unsafe { mem::transmute(num) })
        } else {
            None
        }
    }
}

impl HandleProtocol for PortHandle {
    type Data = (u8, u8);

    fn to_data(&self) -> (u8, u8) {
        (self.module, self.channel)
    }
}

impl From<PortHandle> for i32 {
	fn from(handle: PortHandle) -> i32 {
		// Our struct has the same memory layout as the i32 representation
		unsafe { mem::transmute(handle) }
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    const HANDLE_TYPES: [HandleType; 18] = [
        HandleType::Undefined,
        HandleType::DIO,
        HandleType::Port,
        HandleType::Notifier,
        HandleType::Interrupt,
        HandleType::AnalogOutput,
        HandleType::AnalogInput,
        HandleType::AnalogTrigger,
        HandleType::Relay,
        HandleType::PWM,
        HandleType::DigitalPWM,
        HandleType::Counter,
        HandleType::FPGAEncoder,
        HandleType::Encoder,
        HandleType::Compressor,
        HandleType::Solenoid,
        HandleType::AnalogGyro,
        HandleType::Vendor
    ];

    #[test]
    fn check_handle_type() {
        for (i, t) in HANDLE_TYPES.iter().enumerate() {
            assert_eq!(HandleType::from_u8(i as u8).unwrap(), *t);
        }
    }

    mod type_byte {
        use super::*;

        #[test]
        fn check_constructor_error() {
            let err = TypeByte::new(0b0011_1011);
            let normal = TypeByte::new(0b0000_1011);

            assert!(err.is_none());
            assert!(normal.is_some());
        }

        #[test]
        fn check_error_bit() {
            let mut byte = TypeByte::new(0b0000_1011).unwrap();

            assert_eq!(byte.error_bit(), false);
            byte.set_error_bit(true);
            assert_eq!(byte.error_bit(), true);
            byte.set_error_bit(false);
            assert_eq!(byte.error_bit(), false);
        }

        #[test]
        fn check_types() {
            for (i, t) in HANDLE_TYPES.iter().enumerate() {
                let byte = TypeByte(i as u8);
                let error_byte = TypeByte(i as u8 | (1 << 7));

                assert_eq!(byte.get_type(), *t);
                assert_eq!(error_byte.get_type(), *t);
            }
        }

        #[test]
        fn check_into_type_byte() {
            for (i, t) in HANDLE_TYPES.iter().enumerate() {
                let value: TypeByte = (*t).into();
                assert_eq!(value, TypeByte(i as u8));
            }
        }
    }

    mod handle {
        use super::*;

        #[test]
        fn check_constructor() {
            for (i, t) in HANDLE_TYPES.iter().enumerate() {
                let handle = Handle::new(*t, i as u16);
                assert_eq!(handle.index, i as u16);
                assert_eq!(handle.type_byte, (*t).into());
            }
        }

        #[test]
        fn check_from_i32() {
            let handle = Handle::from_i32(0b0000_0101_0000_0001_0000_0000_0000_1001).unwrap();
            assert_eq!(handle.type_byte.get_type(), HandleType::PWM);
            assert_eq!(handle.index, 0b0000_0101_0000_0001);
        }

        #[test]
        fn check_into_i32() {
            let handle = Handle::new(HandleType::PWM, 0b0000_0101_0000_0001);
            let handle_i32: i32 = handle.into();
            assert_eq!(handle_i32, 0b0000_0101_0000_0001_0000_0000_0000_1001)
        }
    }

    mod port_handle {
        use super::*;

        #[test]
        fn check_constructor() {
            let handle = PortHandle::new(7, 24);
            assert_eq!(handle.module, 7);
            assert_eq!(handle.channel, 24);
            assert_eq!(handle.type_byte, HandleType::Port.into());
        }

        #[test]
        fn check_from_i32() {
            let handle = PortHandle::from_i32(0b0000_0101_0000_0001_0000_0000_0000_0010).unwrap();
            assert_eq!(handle.type_byte.get_type(), HandleType::Port);
            assert_eq!(handle.channel, 0b0000_0101);
            assert_eq!(handle.module, 0b0000_0001);
        }

        #[test]
        fn check_into_i32() {
            let handle = PortHandle::new(0b0000_0001, 0b0000_0101);
            let handle_i32: i32 = handle.into();
            assert_eq!(handle_i32, 0b0000_0101_0000_0001_0000_0000_0000_0010)
        }
    }

}
