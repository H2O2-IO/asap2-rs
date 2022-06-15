pub enum AddressLocation
{
    Intern,
    Extern
}

pub enum KWPCopyMode
{
    RAMInitByECU,
    RAMInitByTool
}

pub enum KWPDataAccessFlags
{
    ReadData = 1,
    VerifyCode = 2,
    ReadCode = 4,
    RWOnlyOnActivePage = 8
}

pub enum KWPFlashMode
{
    NoFlashBack,
    AutoFlashBack,
    ToolFlashBack
}

pub enum KWPFlashResult
{
    RequestRoutineResults,
    StartRoutine,
    CodedResult
}

pub enum KWPMeasurementMode
{
    AddressMode,
    BlockMode
}

pub enum KWPPageSwitchMode
{
    EscapeCode,
    LocalRoutine
}

pub enum KWPPhysicalLayer
{
    NotSet,
    KLine,
    CAN,
    KLineOnCAN,
    KLineAndCAN
}

pub enum KWPStimulationMode
{
    NotSet,
    _WuP,
    _5Baud
}

pub enum KWPVersion
{
    NotSet,
    VDA1996,
}