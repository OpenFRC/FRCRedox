use self::types;

mod handle {

    const INVALID_HANDLE_INDEX :i16 = -1;

    enum HALHandleType {
        Undefined = 0,
        DIO = 1,
        Port = 2,
        Notifier = 3,
        Interrupt = 4,
        AnalogOutput = 5,
        AnalogInput = 6,
        AnalogTrigger = 7,
        Relay = 8,
        PWM = 9,
        DigitalPWM = 10,
        Counter = 11,
        FPGAEncoder = 12,
        Encoder = 13,
        Compressor = 14,
        Solenoid = 15,
        AnalogGyro = 16,
        Vendor = 17
    }

    #[inline]
    fn get_handler_index(handle: types::HALHandle) -> i16 {
        return (hand & 0xffff) as i16
    }

    #[inline]
    fn get_handle_type(handle: types::HALHandle) -> HALHandleType {
        return match (handle >> 24) & 0xff {
            0 => HALHandleType::Undefined,
            1 => HALHandleType::DIO,
            2 => HALHandleType::Port,
            3 => HALHandleType::Notifier,
            4 => HALHandleType::Interrupt,
            5 => HALHandleType::AnalogOutput,
            6 => HALHandleType::AnalogInput,
            7 => HALHandleType::AnalogTrigger,
            8 => HALHandleType::Relay,
            9 => HALHandleType::PWM,
            10 => HALHandleType::DigitalPWM,
            11 => HALHandleType::Counter,
            12 => HALHandleType::FPGAEncoder,
            13 => HALHandleType::Encoder,
            14 => HALHandleType::Compressor,
            15 => HALHandleType::Solenoid,
            16 => HALHandleType::AnalogGyro,
            17 => HALHandleType::Vendor,
            _ => HALHandleType::Undefined,
        }
    }

    #[inline]
    fn is_handle_type(handle: types::HALHandle, handle_type: HALHandleType) -> bool {
        return handle_type == get_handle_type(handle)
    }

    #[inline]
    fn get_handle_typed_index(handle: types::HALHandle, handle_type: HALHandleType) {
        if !is_handle_type(handle, handle_type) {
            return INVALID_HANDLE_INDEX;
        }
        return get_handler_index(handle);
    }

    // Copied from FRC WPILib

    /* specialized functions for Port handle
     * Port Handle Data Layout
     * Bits 0-7:   Channel Number
     * Bits 8-15:  Module Number
     * Bits 16-23: Unused
     * Bits 24-30: Handle Type
     * Bit 31:     1 if handle error, 0 if no error
     */

    #[inline]
    pub fn get_port_handle_channel(handle: types::HALPortHandle) -> i16{
        if !is_handle_type(handle, HALHandleType::Port) {
            return INVALID_HANDLE_INDEX
        }
        return (handle & 0xff) as u8
    }

    #[inline]
    pub fn get_port_handle_module(handle: types::HALPortHandle) -> i16 {
        if !is_handle_type(handle, HALHandleType::Port) {
            return INVALID_HANDLE_INDEX
        }
        return ((handle >> 8) & 0xff) as u8
    }

    #[inline]
    pub fn get_port_handle_module_spi_enable(handle: types::HALPortHandle) -> i16{
        if !is_handle_type(handle, HALHandleType::Port) {
            return INVALID_HANDLE_INDEX
        }
        return ((handle >> 16) & 0xff) as u8
    }

    pub fn create_port_handle(channel: u8, module: u8) -> types::HALPortHandle {

        let mut handle: types::HALPortHandle = HALHandleType::Port as types::HALPortHandle;

        handle += ((module << 8) & 0xff00) + channel;

        return handle;

    }

    pub fn create_port_handle_for_spi(channel: u8, module: u8) -> types::HALPortHandle {

        let mut handle: types::HALPortHandle = HALHandleType::Port as types::HALPortHandle;

        handle += ((module << 8) & 0xff00);

        handle = handle << 8;

        handle += channel;

        return handle;

    }

    pub fn create_handle(index: i16, handle_type: HALHandleType) {
        if index < 0 {
            return types::INVALID_HANDLE
        }

        let handle_type: u8 = HALHandleType;
        if  handle_type == 0 || handle_type > 127 {
            return types::INVALID_HANDLE
        }

        types::HALHandle handle = handle_type

        handle = handle << 24;

        handle += index;
        return handle;
    }

}