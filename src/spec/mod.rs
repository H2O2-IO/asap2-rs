pub mod ccp;
pub mod xcp;
pub mod kwp;
pub enum NodeType
{
    Unsupported,
    Project,
    Header,
    Module,
    A2ML,
    ModPar,
    MemoryLayout,
    IfData,
    ModCommon,
    AxisPts,
    Characteristic,
    AxisDescr,
    Measurement,
    Function,
    Group,
    DefCharacteristic,
    RefCharacteristic,
    RefMeasurement,
    RefGroup,
    InMeasurement,
    OutMeasurement,
    LocMeasurement,
    SubFunction,
    FunctionList,
    SubGroup,
    CompuMethod,
    CompuVTabRange,
    CompuVTab,
    RecordLayout,
    CalibrationMethod,
    CalibrationHandle,
    MemorySegment,
    Annotation,
    AnnotationText,
    CompuTab,
    Formula,
    FixAxisParList,
    Frame,
    DependentCharacteristic,
    VirtualCharacteristic,
    Unit,
    UserRights,
    Virtual,
    VarCriterion,
    VarCharacteristic,
    VarAddress,
    VarForbiddenComb,
    BitOperation,
    VariantCoding,
    Blob,
    Instance,
    TypedefStructure,
    TypedefBlob,
    TypedefMeasurement,
    TypedefAxis,
    TypedefCharacteristic,
    StructureComponent,
    Overwrite,
    Transformer,
    TransformerInObjects,
    TransformerOutObjects,
    ArComponent,
    DAQListCANID,
    DefinedPages,
    CANParam,
    ChecksumParam,
    TPBlob,
    SeedKey,
    Source,
    Raster,
    EventGroup,
    QPBlob,
    DPBlob,
    KPBlob,
    KLine,
    CAN,
    Address,
    NetworkLimits,
    TimeDef,
    Copy,
    FlashCopy,
    Flash,
    DiagBaud,
    PageSwitch,
    RoutinePara,
    DataAddress,
    EventCANIDList,
    CANFD,
    ProtocolLayer,
    Segment,
    Checksum,
    Page,
    AddressMapping,
    DAQ,
    STIM,
    DAQList,
    Event,
    DAQEvent,
    AvailableEventList,
    DefaultEventList,
    ConsistencyEventList,
    TimestampSupported,
    PAG,
    PGM,
    Sector,
    XCPOnUDPIP,
    XCPOnTCPIP,
    XCPOnCAN,
    TimeCorrelation,
    Clock,
    TimestampCharacterization,
    Predefined,
    ODT,
    ECUStates,
    State,
    MemoryAccess,
    DAQMemoryConsumption,
    MinCycleTime,
    BufferReserve,
    BufferReserveEvent,
    DAQPackedMode,
    CPULoadConsumptionDAQ,
    CPULoadConsumptionSTIM,
    CPULoadConsumptionQueue,
    CPULoadConsumptionQueueSTIM,
    ODTEntrySizeFactorTable,
    CoreLoadMax,
    CoreLoadEP,
    XCPOnUSB,
    DAQListUSBEndpoint,
    OutEPCmdSTIM,
    OutEPOnlySTIM,
    InEPOnlyDAQ,
    InEPOnlyEVSERV,
    InEPResErrDAQEVSERV,
    XCPOnSxI,
    Framing
}

pub enum AddrType
{
    Direct,
    PByte,
    PWord,
    PLong,
    PLongLong
}

pub enum AxisValueType
{
    AxisX,
    AxisY,
    AxisZ,
    Axis4,
    Axis5,
}

pub enum AxisType
{
    StdAxis,
    FixAxis,
    ComAxis,
    ResAxis,
    CurveAxis
}

pub enum BitOperationType
{
    NotSet,
    RightShift,
    LeftShift
}

pub enum CalibrationAccess
{
    NotSet,
    Calibration,
    NoCalibration,
    NotInMcdSystem,
    OfflineCalibration
}

pub enum CharacteristicType {
    NotSet,
    Ascii,
    Curve,
    Map,
    Cuboid,
    Cube4,
    Cube5,
    ValBlk,
    Value,
}

pub enum ChecksumType
{
    NotSet,
    Add11,
    Add12,
    Add14,
    Crc8,
    Crc16,
    Crc32,
    Crc2_16,
    Add22,
    Add24,
    Add44,
    UserDefined,
    Crc16CITT
}

pub enum ConversionType {
    Identical,
    Form,
    Linear,
    RatFunc,
    TabIntp,
    TabNointp,
    TabVerb,
}

pub enum DAQEventType {
    None,
    XCP,
    CCP
}

pub enum DataSize {
    Byte,
    Word,
    Long,
}

pub enum DepositType {
    NotSet,
    Absolute,
    Difference,
}

pub enum ECUPage
{
    Flash,
    RAM,
    NotSet = 255
}

pub enum EncodingType
{
    ASCII,
    UTF8,
    UTF16,
    UTF32
}

pub enum IndexMode {
    NotSet,
    AlternateCurves,
    AlternateWithX,
    AlternateWithY,
    ColumnDir,
    RowDir,
}

pub enum IndexOrder {
    IndexIncr,
    IndexDecr,
}

pub enum MemoryPRGType
{
    Unknown,
    Code,
    Data,
    OfflineData,
    Variables,
    SERAM,
    Reserved,
    CalibrationVariables,
    ExcludeFromFlash
}

pub enum MemoryAttribute {
    Intern,
    Extern,
}

pub enum MemoryType {
    EEPROM,
    EPROM,
    Flash,
    RAM,
    ROM,
    Register,
    NotInECU,
}

pub enum MonotonyType {
    NotSet,
    MonDecrease,
    MonIncrease,
    StrictDecrease,
    StrictIncrease,
    Monotonous,
    StrictMon,
    NotMon,
}

pub enum PRGType
{
    Code,
    Data,
    Reserved
}

pub enum ScalingUnits
{
    Time1uSec,
    Time10uSec,
    Time100uSec,
    Time1mSec,
    Time10mSec,
    Time100mSec,
    Time1Sec,
    Time10Sec,
    Time1Min,
    Time1Hour,
    Time1Day,
    AngularDegrees = 100,
    Revolutions,
    Cycle,
    CylinderSegmentCombustion,
    FrameAvailableEvent = 998,
    AlwaysOnNewValue,
    NonDeternministic,
    NotSet = 2147483647
}

pub enum TriggerType
{
    OnChange,
    OnUserRequest
}

pub enum UnitType {
    Derived,
    ExtendedSI,
}

pub enum VarNamingType
{
    NotSet,
    Numeric
}

