use asap2_derive::{Node,add_node_field, add_namenode_field};

pub mod ccp;
pub mod xcp;
pub mod kwp;



#[derive(Copy,Clone,strum_macros::Display,Debug)]
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

impl From<NodeType> for String {
    fn from(_: NodeType) -> Self {
        todo!()
    }
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

/// The trait node is implemented for all a2l objects.
/// It gives location infomation of each object in an a2l file.
/// TODO: because Rust doesn't support data inheritance, we need a derive macro to auto generate some common field.
pub trait Node {
    /// get the first line in the original file.
    fn start_line(&self)->u32;
    /// get the last line in the original file.
    fn end_line(&self)->u32;
    /// get name of this node, for nodes without name, return it's node type as string.
    fn name(&self) -> String {
        self.node_type().to_string()
    }
    /// get the type of this node
    fn node_type(&self)->NodeType;
}

/// A `NamedNode` is a a2l objects with a name in an a2l file, such as `/Project` `/Module` ...
pub trait NamedNode:Node {
    fn node_name(&self)->String;
    fn get_name(&self)->String{
        self.node_name()
    }
}

/// An `UnsupprotedNode` is an a2l object the crate doesn't support, such as A2ML and IF_DATA
pub trait UnsupportedNode:NamedNode {
    fn node_type(&self)->NodeType{
        NodeType::Unsupported
    }
    fn content(&self)->String;
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
pub struct A2ML {
    content:String,
}

impl UnsupportedNode for A2ML {
    fn content(&self)->String {
        self.content.clone()
    }

    fn node_type(&self)->NodeType{
        NodeType::A2ML
    }
}

impl NamedNode for A2ML {
    fn node_name(&self)->String {
        self.name.clone()
    }
}
