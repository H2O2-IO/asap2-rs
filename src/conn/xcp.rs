#![allow(non_upper_case_globals)] // for bit flags, we want it's more like enum

use bitflags::bitflags;

pub enum BasicAddressModeType
{
    Address,
    Length
}

pub enum CalPageMode
{
    ECU = 1,
    XCP = 2,
    ALL = 128
}

pub enum ClockInfo
{
    None,
    SlvClkInfo,
    GrandmClkInfo,
    ClkRelation = 4,
    ECUClkInfo = 8,
    ECUGrandmClkInfo = 16
}

pub enum CmdResult
{
    /// Command processor synchronization
    ErrCmdSynch,
    /// Command was not executed
    ErrCmdBusy = 16,
    /// Command rejected because DAQ is running
    ErrDAQActive,
    /// Command rejected because PGM is running
    ErrPGMActive,
    /// Unknown command or not implemented optional command
    ErrCmdUnknown = 32,
    /// Command syntax invalid
    ErrCmdSyntax,
    /// Command syntax valid but command parameter(s) out of range
    ErrOutOfRange,
    /// The memory location is write protected
    ErrWriteProtected,
    /// The memory location is not accessible
    ErrAccessDenied,
    /// Access denied, Seed & Key is required
    ErrAccessLocked,
    /// Selected page not available
    ErrPageNotValid,
    /// Selected page mode not available
    ErrModeNotValid,
    /// Selected segment not valid
    ErrSegmentNotValid,
    /// Sequence error
    ErrSequence,
    /// DAQ configuration not valid
    ErrDAQConfig,
    /// Memory overflow error
    ErrMemoryOverflow = 48,
    /// Generic error
    ErrGeneric,
    /// The slave internal program verify routine detects an error
    ErrVerify,
    /// Access to the requested resource is temporary not possible
    ErrResourceTemporaryNotAccessible,
    /// Unknown sub command or not implemented optional sub command
    ErrSubCmdUnknown,
    /// Command was canelled by user
    ErrCancelled = 251,
    /// Invalid argument failure (e.g. a call to upload with 0 bytes length)
    ErrInvalidArgument,
    /// Timeout (disconnected?)
    ErrTimeout,
    /// Protocol failure (unexpected response length)
    ErrProtocolFailure,
    /// Successful
    Ok
}

pub enum CommandCode
{
    Connect = 255,
    Disconnect = 254,
    GetStatus = 253,
    Synch = 252,
    GetCommModeInfo = 251,
    GetId = 250,
    SetRequest = 249,
    GetSeed = 248,
    Unlock = 247,
    SetMTA = 246,
    Upload = 245,
    ShortUpload = 244,
    BuildChecksum = 243,
    TransportLayerCmd = 242,
    UserCmd = 241,
    Download = 240,
    DownloadNext = 239,
    DownloadMax = 238,
    ShortDownload = 237,
    ModifyBits = 236,
    SetCalPage = 235,
    GetCalPage = 234,
    GetPAGProcessorInfo = 233,
    GetSegmentInfo = 232,
    GetPageInfo = 231,
    SetSegmentMode = 230,
    GetSegmentMode = 229,
    CopyCALPage = 228,
    SetDAQPtr = 226,
    WriteDAQ = 225,
    SetDAQListMode = 224,
    StartStopDAQList = 222,
    StartStopSynch = 221,
    WriteDAQMultiple = 199,
    ReadDAQ = 219,
    GetDAQClock,
    GetDAQProcessorInfo = 218,
    GetDAQResolutionInfo = 217,
    GetDAQListMode = 223,
    GetDAQEventInfo = 215,
    DtoCtrPproperties = 197,
    ClearDaqList = 227,
    GetDAQListInfo = 216,
    FreeDAQ = 214,
    AllocDAQ = 213,
    AllocODT = 212,
    AllocODTEntry = 211,
    ProgramStart = 210,
    ProgramClear = 209,
    Program = 208,
    ProgramReset = 207,
    GetPGMProcessorInfo = 206,
    GetSectorInfo = 205,
    ProgramPrepare = 204,
    ProgramFormat = 203,
    ProgramNext = 202,
    ProgramMax = 201,
    ProgramVerify = 200,
    TimeCorrelationProperties = 198
}

pub enum CommModeOptional
{
    None = 0,
    MasterBlockMode = 1,
    InterleavedMode = 2
}

pub enum CommModeProgram
{
    None = 0,
    MasterBlockMode = 1,
    InterleavedMode = 2,
    SlaveBlockMode = 64
}

pub enum ConnectMode
{
    Normal,
    UserDefined
}

pub enum DAQEventProperties
{
    DAQ = 4,
    STIM = 8,
    ConsistencyDAQ = 64,
    ConsistencyEvent = 128
}

pub enum DAQKeyByte
{
    OptimisationType0 = 1,
    OptimisationType1 = 2,
    OptimisationType01 = 3,
    OptimisationType2 = 4,
    OptimisationType02 = 5,
    OptimisationType3 = 8,
    AdrExtensionODT = 16,
    AdrExtensionDAQ = 32,
    AdrExtensionODTDAQ = 48,
    IDFieldType0 = 64,
    IDFieldType1 = 128,
    IDFieldType11 = 192
}

// pub enum DAQListMode
// {
//     None = 0,
//     Alternating = 1,
//     Selected = 1,
//     Direction = 2,
//     DtoCtr = 8,
//     Timestamp = 16,
//     PIDOff = 32,
//     Running = 64,
//     Resume = 128
// }

bitflags! {
    struct DAQListMode:u8 {
        const None = 0;
        const Alternating = 1;
        const Selected = 1;
        const Direction = 2;
        const DtoCtr = 8;
        const Timestamp = 16;
        const PIDOff = 32;
        const Running = 64;
        const Resume = 128;
    }
}

pub enum DAQListProperties
{
    Predefined = 1,
    EventFixed = 2,
    DAQ = 4,
    STIM = 8
}

pub enum DAQProperties
{
    Dynamic = 1,
    PrescalerSupported = 2,
    ResumeSupported = 4,
    BitSTIMSupported = 8,
    TimestampSupported = 16,
    PIDOffSupported = 32,
    OverloadMSB = 64,
    OverloadEvent = 128
}

pub enum DAQTimestampMode
{
    Size0 = 1,
    Size1 = 2,
    Size2 = 4,
    TimestampFixed = 8,
    Uint0 = 16,
    Uint1 = 32,
    Uint2 = 64,
    Uint3 = 128
}

pub enum DTOCtrMode
{
    None,
    DAQ,
    STIM
}

pub enum DTOCtrModifier
{
    None,
    STIM,
    DAQ,
    RelatedEvent = 4
}

pub enum DTOCtrProperties
{
    RelatedEventFixed = 1,
    DAQModeFixed,
    STIMModeFixed = 4,
    RelatedEventPresent = 8,
    DAQModePresent = 16,
    STIMModePresent = 32,
    STIMCtrCpyPresent = 64,
    EctCtrPresent = 128
}

pub enum EventCodes
{
    /// Slave starting in RESUME mode
    ResumeMode,
    /// The DAQ configuration in non-volatile memory has been cleared. 
    ClearDAQ,
    /// The DAQ configuration has been stored into non-volatile memory.
    StoreDAQ,
    /// The calibration data has been stored into non-volatile memory.
    StoreCAL,
    /// Slave requesting to restart time-out
    CmdPending = 5,
    /// DAQ processor overload.
    DAQOverload,
    /// Session terminated by slave device.
    SessionTerminated,
    /// Transfer of externally triggered timestamp
    TimeSync,
    /// Indication of a STIM timeout
    STIMTimeout,
    /// Slave entering SLEEP mode
    Sleep,
    /// Slave leaving SLEEP mode
    WakeUp,
    /// Lost DAQ data
    DAQDataLost = 253,
    /// User-defined event
    User,
    /// Transport layer specific event
    Transport
}

pub enum GetIDRespType
{
    TransferMode = 1,
    CompressedEncrypted = 2
}

pub enum GetIDType
{
    ASCII,
    ASAP2FileNameWithoutExt,
    ASAP2FileName,
    ASAP2URL,
    ASAP2File2Upload
}

pub enum GetSectorInfoMode
{
    StartAddress,
    Length,
    NameLength
}

pub enum GetSegmentModeType
{
    Address,
    Standard,
    Mapping
}

pub enum GetSlaveIDMode
{
    IdentifyByEcho,
    ConfirmByInverseEcho
}

pub enum GranODTEntrySize
{
    Byte = 1,
    Word,
    DWord = 4,
    QWord = 8
}

pub enum MappingInfoModeType
{
    SourceAddress,
    DestinationAddress,
    LengthAddress
}

pub enum ObservableClocks
{
    None,
    XCPSlvClkAvail,
    XCPSlvClkNotAvail,
    GrandmClkAvail = 4,
    GrandmClkNotAvail = 8,
    ECUClkCanReadRandom = 16,
    ECUClkCanNotReadRandom = 32,
    ECUClkCanNotRead = 48
}

pub enum PageProperties
{
    ECUAccessWithoutXCP = 1,
    ECUAccessWithXCP = 2,
    XCPReadAccessWithoutECU = 4,
    XCPReadAccessWithECU = 8,
    XCPWriteAccessWithoutECU = 16,
    XCPWriteAccessWithECU = 32
}

pub enum PayloadFmt
{
    None,
    XCPSlvDWord,
    XCPSlvDLong,
    GrandmDWord = 4,
    GrandmDLong = 8,
    ECUDWord = 16,
    ECUDLong = 32,
    ClusterIdentifier = 64
}

pub enum PIDSlaveMaster
{
    SERV = 252,
    EV,
    ERR,
    RES
}

pub enum ProgramClearMode
{
    /// Absolute
    AbsoluteAccess,
    /// Functional
    FunctionalAccess
}

pub enum ProgramProperties
{
    None = 0,
    AbsoluteMode = 1,
    FunctionalMode = 2,
    CompressionSupported = 4,
    CompressionRequired = 8,
    EncryptionSupported = 16,
    EncryptionRequired = 32,
    NonSeqProgramSupported = 64,
    NonSeqProgramRequired = 128
}

pub enum ProgramVerifyMode
{
    /// Request to start internal Routine
    RequestToStartInternalRoutine,
    /// Sending verification Value
    SendingVerificationValue,
    None
}

pub enum ResourceType
{
    None = 0,
    CALPAG = 1,
    DAQ = 4,
    STIM = 8,
    PGM = 16
}

pub enum SeedModeType
{
    FirstPart,
    RemainingPart
}

pub enum SegmentInfoMode
{
    GetBasicAddress,
    GetStandardInfo,
    GetAddressMapping
}

pub enum SegmentMode
{
    None = 0,
    Freeze = 1
}

pub enum ServiceRequestCode
{
    /// Slave requesting to be reset
    Reset,
    /// Slave transferring a byte stream of plain ASCII text.
    Text
}

pub enum SessionState
{
    None = 0,
    StoreCALRequest = 1,
    StoreDAQRequest = 4,
    ClearDAQRequest = 8,
    DAQRunning = 64,
    Resume = 128
}

pub enum SetRequestMode
{
    StoreCALRequest = 1,
    StoreDAQRequestNoResume = 2,
    StoreDAQRequestResume = 4,
    ClearDAQRequest = 8
}

pub enum SlaveConfig
{
    None,
    ResponseFmtSendOnInitTrigger,
    ResponseFmtSendOnAllTrigger,
    DAQTsRelation = 4,
    TimeSyncBridgeDisabled = 8,
    TimeSyncBridgeEnabled = 16
}

pub enum STIMTimeoutMode
{
    EventChannelNo,
    DAQListNo
}

pub enum SyncState
{
    None,
    SlvClkIsSyncronized,
    SlvClkIsSyntonized = 3,
    SlvClkIsNotSupported,
    GrandmClk = 8,
    ECUClk = 16,
    ECUClkUnknown = 32
}

pub enum TimeCorrGetPropsReq
{
    None,
    GetClkInfo
}

pub enum TimeCorrSetProps
{
    None,
    ResponseFmtSendOnInitTrigger,
    ResponseFmtSendOnAllTrigger,
    TimeSyncBridgeEnable = 4,
    TimeSyncBridgeDisable = 8,
    SetClusterId = 16
}

pub enum TimeOfTSSampling
{
    DuringCmdProcessing,
    LowJitter,
    PhysicalTransmission,
    PhysicalReception
}

pub enum TriggerInitiator
{
    HWTrigger,
    EventDerived,
    MultiCast,
    MultiCastViaTimeSync,
    StateChangeInSynSync,
    LeapSecondOccured,
    ReleaseECUReset,
    Reserved
}

pub enum USBEndpointType
{
    Configurable,
    Fixxed
}

pub enum XCPonCAN
{
    GetSlaveID = 255,
    GetDAQID = 254,
    SetDAQID = 253,
    GetDAQClockMulticast = 250
}

pub enum XCPType
{
    Unknown,
    UDP,
    TCP,
    CAN,
    USB,
    FlexRay,
    SxI
}