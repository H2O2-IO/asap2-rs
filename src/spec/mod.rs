use asap2_derive::{
    Node,
    add_node_field,
    add_namenode_field,
    add_addressnode_field,
    NamedNode,
};

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

#[derive(Copy,Clone)]
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
#[derive(Copy,Clone)]
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

pub enum WriterIndent
{
    Space,
    Tab
}

pub enum WriterSortMode
{
    None,
    ByName,
    ByTypeAndName,
    ByAddressAndName
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
    fn name(&self)->String{
        self.node_name()
    }
}

/// An `UnsupprotedNode` is an a2l object the crate doesn't support, such as A2ML and IF_DATA
pub trait UnsupportedNode:NamedNode {
    fn content(&self)->String;
}

// pub trait DescriptableObject {
//     fn name(&self) -> String;
//     fn display_name(&self) -> String;
//     fn description(&self) -> String;
//     fn pretty_annotation(&self) -> String;
// }

pub trait MemoryObject {
    fn contains(address: u32) -> bool;
    fn is_readonly() -> bool;
    fn address() -> u32;
    fn size() -> u32;
    fn offsets() -> [i32];
    fn set_offsets(offsets:[i32]);
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
#[derive(NamedNode)]
pub struct A2ML {
    content:String,
}

impl UnsupportedNode for A2ML {
    fn content(&self)->String {
        self.content.clone()
    }
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
#[derive(NamedNode)]
pub struct IfData {
    content:String,
}

impl UnsupportedNode for IfData {
    fn content(&self)->String {
        self.content.clone()
    }
}

pub trait AddressNode : NamedNode {
    fn address(&self)->u32;
    fn address_ext(&self)->i32;
    fn calib_access(&self)->CalibrationAccess;
    // fn instanced(&self)->bool{
    //     return self.start_line() == 0x7FFF_FFFF;
    // }
    fn max_refresh_rate(&self)->i32;
    fn max_refresh_unit(&self)->ScalingUnits;
    fn model_link(&self)->String;
    fn ref_instance(&self)->u32;
    fn symbol_link(&self)->String;
    fn symbol_offset(&self)->i32;

    fn get_memory_size(&self)->u32;
}

#[add_node_field]
#[add_namenode_field]
#[add_addressnode_field]
#[derive(Node)]
#[derive(NamedNode)]
pub struct Blob {
    addr_type: AddrType,
    size:u32,
}

impl AddressNode for Blob {
    fn address(&self)->u32 {
        self.address
    }

    fn address_ext(&self)->i32 {
        self.address_ext
    }

    fn calib_access(&self)->CalibrationAccess {
        self.calib_access
    }

    fn max_refresh_rate(&self)->i32 {
        self.max_refresh_rate
    }

    fn max_refresh_unit(&self)->ScalingUnits {
        self.max_refresh_unit
    }

    fn model_link(&self)->String {
        self.model_link.clone()
    }

    fn ref_instance(&self)->u32 {
        0
    }

    fn symbol_link(&self)->String {
        self.symbol_link.clone()
    }

    fn symbol_offset(&self)->i32 {
        self.symbol_offset
    }

    fn get_memory_size(&self)->u32 {
        self.size
    }
}
