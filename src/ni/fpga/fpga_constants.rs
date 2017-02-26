use self::fpga_types;

#[allow(dead_code,
        non_camel_case_types,
        non_snake_case_variables,
        non_upper_case_globals)]

mod fpga_constants {

    const NiFpga_False: fpga_types::NiFpga_Bool = 0;
    const NiFpga_True: fpga_types::NiFpga_Bool = 1;

    const NiFpga_Status_FifoTimeout: fpga_types::NiFpga_Status = -50400;
    const NiFpga_Status_TransferAborted: fpga_types::NiFpga_Status = -50405;
    const NiFpga_Status_MemoryFull: fpga_types::NiFpga_Status = -52000;
    const NiFpga_Status_SoftwareFault: fpga_types::NiFpga_Status = -52003;
    const NiFpga_Status_InvalidParameter: fpga_types::NiFpga_Status = -52005;
    const NiFpga_Status_ResourceNotFound: fpga_types::NiFpga_Status = -52006;
    const NiFpga_Status_ResourceNotInitialized: fpga_types::NiFpga_Status = -52010;
    const NiFpga_Status_FpgaAlreadyRunning: fpga_types::NiFpga_Status = -61003;
    const NiFpga_Status_DownloadError: fpga_types::NiFpga_Status = -61018;
    const NiFpga_Status_DeviceTypeMismatch: fpga_types::NiFpga_Status = -61024;
    const NiFpga_Status_CommunicationTimeout: fpga_types::NiFpga_Status = -61046;
    const NiFpga_Status_IrqTimeout: fpga_types::NiFpga_Status = -61060;
    const NiFpga_Status_CorruptBitfile: fpga_types::NiFpga_Status = -61070;
    const NiFpga_Status_BadDepth: fpga_types::NiFpga_Status = -61072;
    const NiFpga_Status_BadReadWriteCount: fpga_types::NiFpga_Status = -61073;
    const NiFpga_Status_ClockLostLoc: fpga_types::NiFpga_Status = -61083;
    const NiFpga_Status_FpgaBusy: fpga_types::NiFpga_Status = -61141;
    const NiFpga_Status_FpgaBusyFpgaInterfaceCApi: fpga_types::NiFpga_Status = -61200;
    const NiFpga_Status_FpgaBusyScanInterface: fpga_types::NiFpga_Status = -61201;
    const NiFpga_Status_FpgaBusyFpgaInterface: fpga_types::NiFpga_Status = -61202;
    const NiFpga_Status_FpgaBusyFpgaInteractive: fpga_types::NiFpga_Status = -61203;
    const NiFpga_Status_FpgaBusyFpgaEmulation: fpga_types::NiFpga_Status = -61204;
    const NiFpga_Status_ResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61211;
    const NiFpga_Status_AbortCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61212;
    const NiFpga_Status_CloseAndResetCallWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61213;
    const NiFpga_Status_ImplicitEnableRemovalButNotYetRun: fpga_types::NiFpga_Status = -61214;
    const NiFpga_Status_RunAfterStoppedCalledWithImplicitEnableRemoval: fpga_types::NiFpga_Status = -61215;
    const NiFpga_Status_GatedClockHandshakingViolation: fpga_types::NiFpga_Status = -61216;




}
