pub enum CCPAddressMode
{
    NotSet,
    DAQ,
    ODT
}

pub enum CCPChecksumCalcType
{
    NotSet,
    ActivePage,
    BitOrWithOPTPage,
}

pub enum CCPDAQMode
{
    NotSet,
    Alternating,
    Burst
}

pub enum CCPMemoryPageType
{
    NotSet = 0,
    RAM = 1,
    ROM = 2,
    Flash = 4,
    EEPROM = 8,
    RAMInitByECU = 16,
    RAMInitByTool = 32,
    AutoFlashBack = 64,
    FlashBack = 128,
    Default = 256
}

pub enum CCPScalingUnit
{
    _1US,
    _10US,
    _100US,
    _1MS,
    _10MS,
    _100MS,
    _1S,
    _10S,
    _1MIN,
    _1HOUR,
    _1DAY,
    AngularDegrees = 100,
    Revolutions360Degrees,
    Cycle720Degrees,
    CylinderSegment,
    Event = 998,
    OnValueChanged,
    NonDeterministic
}

