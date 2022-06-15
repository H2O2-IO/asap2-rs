pub enum ChecksumSxI
{
    NotSet = -1,
    NoChecksum,
    ChecksumByte,
    ChecksumWord
}

pub enum CommModeBasic
{
    NotSet = -1, // may modify
    BigEndian = 1,
    AddressGranularityBYTE = 0,
    AddressGranularityWORD = 2,
    AddressGranularityDWORD = 4,
    SlaveBlockMode = 64,
    Optional = 128
}

pub enum DtoCtrDaqMode
{
    NotSet,
    InsertCounter,
    InsertSTIMCounterCopy
}

pub enum DtoCtrStimMode
{
    NotSet,
    DoNotCheckCounter,
    CheckCounter,
}

pub enum PAGProperties
{
    NotSet = 0,
    FreezeSupported = 1
}
pub enum ParityType
{
    None,
    Odd,
    Even
}

pub enum StopBitsType
{
    OneStopBit = 1,
    TwoStopBits,
}

pub enum SyncModeSize
{
    NotSet,
    Byte,
    Word,
    DWord = 4
}

pub enum XCPAddressEXT
{
    Free,
    ODT,
    DAQ
}

pub enum XCPAddressMode
{
    NotSet,
    DAQ,
    Event,
    ODT,
    None
}

pub enum XCPAlignment
{
    _8BIT = 1,
    _16BIT = 2,
    _32BIT = 4,
    _64BIT = 8
}

pub enum XCPBlockMode
{
    Standard = 0,
    Slave = 1,
    Master = 2,
    Interleaved = 4
}

pub enum XCPClockEpoch
{
    AtomicTime,
    UniversalCoordinatedTime,
    Arbitrary
}

pub enum XCPClockMode
{
    XCPSlaveClock,
    ECUClock,
    XCPSlaveGrandmasterClock,
    ECUGrandmasterClock
}

pub enum XCPClockReadAbility
{
    RandomlyReadable,
    LimitedReadable,
    NotReadable
}

pub enum XCPClockSyncFeature
{
    SynUnsupported,
    SynChronizationOnly,
    SynTonizationOnly,
    SynAll
}

pub enum XCPDAQListCANSampleRate
{
    NotSet,
    Single,
    Triple
}

pub enum XCPDAQListCANType
{
    NotSet,
    Variable,
    Fixed
}

pub enum XCPDAQListType
{
    NotSet,
    DAQ,
    STIM,
    DAQSTIM
}

pub enum XCPDAQMode
{
    Static,
    Dynamic
}

pub enum XCPECUAccess
{
    NotAllowed = 1,
    WithoutXCPOnly = 2,
    WithXCPOnly = 4,
    DontCare = 8
}

pub enum XCPEndpoint
{
    NotSet,
    In,
    Out
}

pub enum XCPGroupMode
{
    ElementGrouped,
    EventGrouped,
}

pub enum XCPHeaderLen
{
    NotSet,
    Byte,
    CTRByte,
    FillByte,
    Word,
    CTRWord,
    FillWord
}

pub enum XCPIDFieldType
{
    Absolute,
    Byte,
    Word,
    Aligned
}

pub enum XCPMemoryAccess
{
    NotAllowed,
    Allowed,
}

pub enum XCPMessagePacking
{
    Single,
    Multiple,
    Streaming
}

pub enum XCPNativeTimestampSize
{
    FourByte,
    EightByte
}

pub enum XCPODTEntrySize
{
    Byte = 1,
    Word = 2,
    DWord = 4,
    DLong = 8
}

pub enum XCPOptimisationType
{
    DEFAULT = 0,
    ODTType16 = 1,
    ODTType32 = 2,
    ODTType64 = 3,
    ODTTypeAlignment = 4,
    MaxEntrySize = 5
}

pub enum XCPOverloadIND
{
    NotSet,
    Indication,
    PID,
    Event
}

pub enum XCPPacketAligment
{
    _8,
    _16,
    _32,
    NotSet
}

pub enum XCPPackMode
{
    Optional,
    Mandatory
}

pub enum XCPPGMMode
{
    Absolute = 1,
    Functional = 2
}

pub enum XCPReadWriteAccess
{
    NotAllowed = 1,
    WithoutECUOnly = 2,
    WithECUOnly = 4,
    DontCare = 8
}

pub enum XCPResourceState
{
    NotActive,
    Active
}

pub enum XCPSTSMode
{
    Last,
    First
}

pub enum XCPSyncEdge
{
    NotSet,
    Single,
    Dual
}

pub enum XCPTimestampResolution
{
    _1NS,
    _10NS,
    _100NS,
    _1US,
    _10US,
    _100US,
    _1MS,
    _10MS,
    _100MS,
    _1S,
    _1PS,
    _10PS,
    _100PS
}

pub enum XCPTimestampSize
{
    NotSet,
    BYTE,
    WORD,
    DWORD = 4
}

pub enum XCPTransceiverDelayCompensation
{
    NotSet,
    On,
    Off,
}

pub enum XCPTransfer
{
    BulkTransfer,
    InterruptTransfer,
}

pub enum XCPTSRelation
{
    XCPSlaveClock,
    ECUClock
}