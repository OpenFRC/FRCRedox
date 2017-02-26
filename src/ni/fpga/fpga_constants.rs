
use ni::fpga::fpga_types;

#[allow(dead_code,
        non_camel_case_types,
        non_snake_case_variables,
        non_upper_case_globals)]

pub const NiFpga_False: fpga_types::NiFpga_Bool = 0;
pub const NiFpga_True: fpga_types::NiFpga_Bool = 1;
pub const NiFpga_Status_FifoTimeout: fpga_types::NiFpga_Status = -50400;
pub const NiFpga_Status_TransferAborted: fpga_types::NiFpga_Status = -50405;
pub const NiFpga_Status_MemoryFull: fpga_types::NiFpga_Status = -52000;
pub const NiFpga_Status_SoftwareFault: fpga_types::NiFpga_Status = -52003;
pub const NiFpga_Status_InvalidParameter: fpga_types::NiFpga_Status = -52005;
pub const NiFpga_Status_ResourceNotFound: fpga_types::NiFpga_Status = -52006;
pub const NiFpga_Status_ResourceNotInitialized: fpga_types::NiFpga_Status = -52010;
pub const NiFpga_Status_FpgaAlreadyRunning: fpga_types::NiFpga_Status = -61003;
pub const NiFpga_Status_DownloadError: fpga_types::NiFpga_Status = -61018;
pub const NiFpga_Status_DeviceTypeMismatch: fpga_types::NiFpga_Status = -61024;
pub const NiFpga_Status_CommunicationTimeout: fpga_types::NiFpga_Status = -61046;
pub const NiFpga_Status_IrqTimeout: fpga_types::NiFpga_Status = -61060;
pub const NiFpga_Status_CorruptBitfile: fpga_types::NiFpga_Status = -61070;
pub const NiFpga_Status_BadDepth: fpga_types::NiFpga_Status = -61072;
pub const NiFpga_Status_BadReadWriteCount: fpga_types::NiFpga_Status = -61073;
pub const NiFpga_Status_ClockLostLoc: fpga_types::NiFpga_Status = -61083;
pub const NiFpga_Status_FpgaBusy: fpga_types::NiFpga_Status = -61141;
pub const NiFpga_Status_FpgaBusyFpgaInterfaceCApi: fpga_types::NiFpga_Status = -61200;
pub const NiFpga_Status_FpgaBusyScanInterface: fpga_types::NiFpga_Status = -61201;
pub const NiFpga_Status_FpgaBusyFpgaInterface: fpga_types::NiFpga_Status = -61202;
pub const NiFpga_Status_FpgaBusyFpgaInteractive: fpga_types::NiFpga_Status = -61203;
pub const NiFpga_Status_FpgaBusyFpgaEmulation: fpga_types::NiFpga_Status = -61204;
pub const NiFpga_Status_ResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61211;
pub const NiFpga_Status_AbortCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61212;
pub const NiFpga_Status_CloseAndResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61213;
pub const NiFpga_Status_ImplicitEnableRemovalButNotYetRun: fpga_types::NiFpga_Status = -61214;
pub const NiFpga_Status_RunAfterStoppedCalledWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61215;
pub const NiFpga_Status_GatedClockHandshakingViolation: fpga_types::NiFpga_Status = -61216;
