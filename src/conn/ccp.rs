pub enum CmdResult {
    /// Successful
    Ok,
    /// DAQ processor overload
    DAQProcOverload,
    /// command processor busy
    CMDProcBusy = 16,
    /// DAQ processor busy
    DAQActive,
    /// internal timeout
    InternalTimeout,
    /// key request
    KeyRequest = 24,
    /// session status request
    SessionStateRequest,
    /// cold start request
    ColdStartRequest = 32,
    /// cal. data init. request
    CALDataInitRequest,
    /// DAQ list init. request
    DAQListInitRequest,
    /// code update request
    CodeUpdateRequest,
    /// unknown command
    UnknownCommand = 48,
    /// command syntax
    CommandSyntax,
    /// parameter(s) out of range
    ParOutOfRange,
    /// access denied
    AccessDenied,
    /// overload
    Overload,
    /// access locked
    AccessLocked,
    /// resource/function not available
    ResFuncNotAvailable,
    /// Protocol failure (unexpected response length)
    ProtocolFailure = 253,
    /// Unspecified failure
    Generic,
    /// Timeout
    Timeout,
}

pub enum CommandCode {
    Connect = 1,
    GetCCPVersion = 27,
    ExchangeID = 23,
    GetSeed = 18,
    Unlock,
    SetMTA = 2,
    Dnload,
    Dnload6 = 35,
    Upload = 4,
    ShortUp = 15,
    SelectCALPage = 17,
    GetDAQSize = 20,
    SetDAQPtr,
    WriteDAQ,
    StartStop = 6,
    Disconnect,
    SetSStatus = 12,
    GetSStatus,
    BuildChksum,
    ClearMemory = 16,
    Program = 24,
    Program6 = 34,
    Move = 25,
    Test = 5,
    GetActiveCALPage = 9,
    StartStopAll = 8,
    DiagService = 32,
    ActionService,
}

pub enum DisconnectMode {
    Temporary,
    EndOfSession,
}

pub enum PIDSlaveMaster {
    RES = 255,
    EV = 254,
}

pub enum ResourceType {
    None = 0,
    CAL = 1,
    DAQ = 2,
    PGM = 64,
}

pub enum SessionState {
    None = 0,
    CAL = 1,
    DAQ = 2,
    Resume = 4,
    Store = 64,
    Run = 128,
}
