use indextree::{Arena, NodeId};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use asap2_derive::{add_addressnode_field, add_namenode_field, add_node_field, NamedNode, Node};

pub mod ccp;
pub mod kwp;
pub mod xcp;

#[derive(Copy, Clone, strum_macros::Display, Debug)]
pub enum NodeType {
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
    Framing,
}

impl From<NodeType> for String {
    fn from(_: NodeType) -> Self {
        todo!()
    }
}

pub enum AddrType {
    Direct,
    PByte,
    PWord,
    PLong,
    PLongLong,
}

pub enum AxisValueType {
    AxisX,
    AxisY,
    AxisZ,
    Axis4,
    Axis5,
}

pub enum AxisType {
    StdAxis,
    FixAxis,
    ComAxis,
    ResAxis,
    CurveAxis,
}

pub enum BitOperationType {
    NotSet,
    RightShift,
    LeftShift,
}

#[derive(Copy, Clone)]
pub enum CalibrationAccess {
    NotSet,
    Calibration,
    NoCalibration,
    NotInMcdSystem,
    OfflineCalibration,
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

pub enum ChecksumType {
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
    Crc16CITT,
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
    CCP,
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

pub enum ECUPage {
    Flash,
    RAM,
    NotSet = 255,
}

pub enum EncodingType {
    ASCII,
    UTF8,
    UTF16,
    UTF32,
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

pub enum MemoryPRGType {
    Unknown,
    Code,
    Data,
    OfflineData,
    Variables,
    SERAM,
    Reserved,
    CalibrationVariables,
    ExcludeFromFlash,
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

pub enum PRGType {
    Code,
    Data,
    Reserved,
}
#[derive(Copy, Clone)]
pub enum ScalingUnits {
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
    NotSet = 2147483647,
}

pub enum TriggerType {
    OnChange,
    OnUserRequest,
}

pub enum UnitType {
    Derived,
    ExtendedSI,
}

pub enum VarNamingType {
    NotSet,
    Numeric,
}

pub enum WriterIndent {
    Space,
    Tab,
}

pub enum WriterSortMode {
    None,
    ByName,
    ByTypeAndName,
    ByAddressAndName,
}

pub enum ByteOrderType {
    MSBFirst,  // = 1,
    BigEndian, // = 1,
    MSBLast,
    LittleEndian, // = 2,
    NotSet,       // = 0
}

pub enum DataType {
    UByte,
    SByte,
    UWord,
    SWord,
    ULong,
    SLong,
    IEEEFloat32,
    AUInt64,
    AInt64,
    IEEEFloat64,
    IEEEFloat16,
    BitField,
}

/// SeedAndKey Type
pub enum SKType {
    Unknown,
    CCP,
    XCP,
    UDS = 4,
}

/// The trait node is implemented for all a2l objects.
/// It gives location infomation of each object in an a2l file.
/// TODO: because Rust doesn't support data inheritance, we need a derive macro to auto generate some common field.
pub trait Node {
    /// get the type of this node
    fn node_type(&self) -> NodeType;
    /// add childs to childs vec
    fn add_child(&mut self, elem: Box<dyn Any>);
    /// find node in childs
    fn find_child_by_line(&mut self, line: u32);
    // fn find_child_by_name<T: 'static + NamedNode>(&mut self, name: &str)->Option<&T>;
}

/// A `NamedNode` is a a2l objects with a name in an a2l file, such as `/Project` `/Module` ...
pub trait NamedNode: Node {
    fn node_name(&self) -> String;
    fn name(&self) -> String {
        self.node_name()
    }
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct A2ML {
    pub content: String,
}

impl A2ML {
    fn find_child_by_name<T: 'static + NamedNode>(&mut self, name: &str) -> Option<&T> {
        let mut childs = vec![];
        childs.push(self.childs.iter().as_slice());

        self.childs
            .iter()
            .find(|x| {
                x.downcast_ref::<T>()
                    .map_or(None, |y| {
                        println!("{}", NamedNode::name(y));
                        if NamedNode::name(y) == name {
                            Some(1)
                        } else {
                            None
                        }
                    })
                    .is_some()
            })
            .map_or(None, |z| z.downcast_ref::<T>())
    }
}

#[cfg(test)]
mod test {
    use crate::spec::Node;
    use indextree::Arena;

    use super::{Blob, IfData, NamedNode, A2ML};
    #[test]
    fn hello_childs() {
        let mut x = A2ML {
            content: "some_text".to_string(),
            node_type: super::NodeType::A2ML,
            start_line: 100,
            end_line: 123,
            childs: vec![],
            name: "some".to_string(),
        };

        x.childs.push(Box::new(IfData {
            content: "ifdata".to_string(),
            node_type: super::NodeType::IfData,
            start_line: 101,
            end_line: 111,
            childs: vec![],
            name: "iddata".to_string(),
        }));

        x.childs.push(Box::new(Blob {
            addr_type: super::AddrType::Direct,
            size: 12,
            node_type: super::NodeType::Blob,
            start_line: 111,
            end_line: 124,
            childs: vec![],
            name: "blob".to_string(),
            address: 123,
            address_ext: 456,
            calib_access: None,
            description: "sss".to_string(),
            max_refresh_rate: 234,
            max_refresh_unit: None,
            model_link: "link".to_string(),
            symbol_link: "zzzz".to_string(),
            symbol_offset: -2,
            ref_instance: None,
        }));

        assert!(x.find_child_by_name::<Blob>("xxx").is_none());
        // assert_eq!(x.find_child_by_name::<Blob>("blob"),1);

        let x = x
            .find_child_by_name::<Blob>("blob")
            .map(|z| z.model_link.clone())
            .unwrap();
        assert_eq!(x, "link")
    }

    #[test]
    fn test_arena() {
        let arena: &mut Arena<Box<dyn Node>> = &mut Arena::new();
        let a2ml = arena.new_node(Box::new(A2ML {
            content: "some_text".to_string(),
            node_type: super::NodeType::A2ML,
            start_line: 100,
            end_line: 123,
            childs: vec![],
            name: "some".to_string(),
        }));

        let ifdata1 = arena.new_node(Box::new(IfData {
            content: "ifdata1".to_string(),
            node_type: super::NodeType::IfData,
            start_line: 101,
            end_line: 111,
            childs: vec![],
            name: "iddata1".to_string(),
        }));

        let ifdata2 = arena.new_node(Box::new(IfData {
            content: "ifdata2".to_string(),
            node_type: super::NodeType::IfData,
            start_line: 101,
            end_line: 111,
            childs: vec![],
            name: "iddata2".to_string(),
        }));

        let ifdata3 = arena.new_node(Box::new(IfData {
            content: "ifdata3".to_string(),
            node_type: super::NodeType::IfData,
            start_line: 101,
            end_line: 111,
            childs: vec![],
            name: "iddata3".to_string(),
        }));

        let blob = arena.new_node(Box::new(Blob {
            addr_type: super::AddrType::Direct,
            size: 12,
            node_type: super::NodeType::Blob,
            start_line: 111,
            end_line: 124,
            childs: vec![],
            name: "blob".to_string(),
            address: 123,
            address_ext: 456,
            calib_access: None,
            description: "sss".to_string(),
            max_refresh_rate: 234,
            max_refresh_unit: None,
            model_link: "link".to_string(),
            symbol_link: "zzzz".to_string(),
            symbol_offset: -2,
            ref_instance: None,
        }));

        a2ml.append(ifdata1, arena);
        a2ml.append(ifdata2, arena);
        a2ml.append(ifdata3, arena);
        ifdata2.append(blob, arena);
        // for it in a2ml.descendants(arena) {
        //     let x = arena[it].get();
        //     println!("{}",x.name());
        // }

        // for it in blob.ancestors(arena) {
        //     let x = arena[it].get();
        //     println!("{}",x.name());
        // }

        // for it in a2ml.children(arena) {
        //     let x = arena[it].get();
        //     println!("{}",x.name());
        // }
        assert!(false);
    }
}

pub struct A2LFile {
    /// child - parent hierichy stored in here.
    internal: Arena<Box<dyn Any>>,
    pub filename: String,
    pub included_files: Vec<String>,
    pub major_version: u32,
    pub minior_version: u32,
    pub project: NodeId,
}

#[add_node_field]
#[derive(Node)]
pub struct Header {
    pub comment: String,
    pub project_no: Option<String>,
    pub version: Option<String>,
}

impl A2LFile {
    fn get_node_by_name<T: 'static + NamedNode>(&self, name: &str) -> Option<&T> {
        self.project
            .descendants(&self.internal)
            .find(|x| {
                self.internal[*x]
                    .get()
                    .downcast_ref::<T>()
                    .map_or(None, |y| if y.name() == name { Some(1) } else { None })
                    .is_some()
            })
            .map_or(None, |z| self.internal[z].get().downcast_ref::<T>())
    }
    fn get_node<T: 'static + Node>(&self) -> Option<&T> {
        self.project
            .descendants(&self.internal)
            .find(|x| self.internal[*x].get().downcast_ref::<T>().is_some())
            .map_or(None, |z| self.internal[z].get().downcast_ref::<T>())
    }
    fn get_node_id<T: 'static + Node>(&self) -> Option<NodeId> {
        self.project
            .descendants(&self.internal)
            .find(|x| self.internal[*x].get().downcast_ref::<T>().is_some())
    }
    fn get_nodes<T: 'static + Node>(&self) -> Vec<&T> {
        self.project
            .descendants(&self.internal)
            .filter(|x| self.internal[*x].get().downcast_ref::<T>().is_some())
            .map(|y| self.internal[y].get().downcast_ref::<T>().unwrap())
            .collect()
    }

    fn get_mut_node<T: 'static + Node>(&mut self, id: NodeId) -> Option<&mut T> {
        self.internal[id].get_mut().downcast_mut::<T>()
    }

    /// NodeId is Copyable,
    fn get_node_ids<T: 'static + Node>(&mut self) -> Vec<NodeId> {
        self.project
            .descendants(&self.internal)
            .filter(|x| self.internal[*x].get().downcast_ref::<T>().is_some())
            .collect()
    }

    fn get_node_mut_by_name<T: 'static + NamedNode>(&mut self, name: &str) -> Option<&mut T> {
        self.project
            .descendants(&self.internal)
            .find(|x| {
                self.internal[*x]
                    .get()
                    .downcast_ref::<T>()
                    .map_or(None, |y| if y.name() == name { Some(1) } else { None })
                    .is_some()
            })
            .map_or(None, |z| self.internal[z].get_mut().downcast_mut::<T>())
    }

    fn remove_node(&mut self, id: NodeId, recursive: bool) {
        if recursive {
            id.remove_subtree(&mut self.internal);
        } else {
            id.remove(&mut self.internal);
        }
    }

    fn append_node(&mut self, parent_id: NodeId, child_id: NodeId) {
        parent_id.append(child_id, &mut self.internal);
    }
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct IfData {
    pub content: String,
}

#[add_node_field]
#[add_namenode_field]
#[add_addressnode_field]
#[derive(Node, NamedNode)]
pub struct Blob {
    pub addr_type: AddrType,
    pub size: u32,
}

/// TODO: remove this struct
pub struct ConversionRef {
    pub byte_order: Option<ByteOrderType>,
    pub conversion: String,
    pub decimal_count: i32,
    pub format: String,
    pub lower_limit: f64,
    pub upper_limit: f64,
    pub memory_segment_ref: String,
    pub ref_compu_method: Option<NodeId>,
    // reffucntions
    pub unit: String,
}

pub struct Measurement {
    pub accuracy: f64,
    pub addr_type: Option<AddrType>,
    pub bit_mask: u64,
    pub data_type: Option<DataType>,
    pub discrete: bool,
    pub error_mask: u64,
    pub layout: Option<IndexMode>,
    pub maxtrix_dim: Option<Vec<i32>>,
    pub read_write: bool,
    pub resolution: i32,
}

/// TODO: remove this struct
pub struct RecordLayoutRef {
    pub guard_rails: bool,
    pub lower_limit_ex: f64,
    pub upper_limit_ex: f64,
    pub max_diff: f64,
    pub read_only: bool,
    pub record_layout: String,
    pub ref_record_layout: Option<NodeId>,
    pub step_size: f64,
}

/// TODO: add an iterator over these layouts
#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct RecordLayout {
    /// map to indices, byte=0, word=1, long=2,float32ieee=3,float64ieee=4,int64=5
    pub alignments: Option<Vec<Vec<i32>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub axis_pts: Option<Vec<Vec<AxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub axis_rescale: Option<Vec<Vec<AxisRescaleLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub dist_op: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub fix_no_axis_pts: Option<Vec<Vec<i32>>>,
    pub fnc_values: Option<FncValuesLayoutDesc>,
    pub identification: Option<NoAxisPtsLayoutDesc>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub no_axis_pts: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub no_rescale: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub offset: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    pub reserved: Option<NoAxisPtsLayoutDesc>,
    /// Take care! map to indices, x=0,y=1,z=2,w=3,_4=4,_5=5
    pub rip_addr: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub shift_op: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    /// map to indices, x=0,y=1,z=2,_4=3,_5=4
    pub src_address: Option<Vec<Vec<NoAxisPtsLayoutDesc>>>,
    pub static_address_offsets: bool,
    pub static_record_layout: bool,
}

/// used by recordlayout
/// delete this node
pub struct ItemlayoutDesc {
    pub axis_idx: i32,
    pub data_type: DataType,
    pub name: String,
    pub position: i32,
}

/// parent is ItemLayoutDesc
pub struct NoAxisPtsLayoutDesc {
    pub datasize: DataSize,
}

/// parent is ItemLayoutDesc
pub struct FncValuesLayoutDesc {
    pub addr_type: AddrType,
    pub index_mode: IndexMode,
}

/// parent is ItemLayoutDesc
pub struct AxisPtsLayoutDesc {
    pub addr_type: AddrType,
    pub index_order: IndexOrder,
}

/// parent is ItemLayoutDesc
pub struct AxisRescaleLayoutDesc {
    pub addr_type: AddrType,
    pub index_order: IndexOrder,
    pub max_no_rescale_pairs: i32,
}

pub struct Characteristic {
    pub bit_mask: u64,
    pub char_type: CharacteristicType,
    pub comparision_quantity: String,
    pub discrete: bool,
    pub encoding: EncodingType,
    pub matrix_dim: Option<Vec<i32>>,
    pub number: i32,
    pub ref_comparison_quantity: Option<NodeId>,
}

pub struct AxisPts {
    pub deposit: DepositType,
    pub input_quantity: String,
    pub max_axis_points: i32,
    pub monotony: MonotonyType,
    pub ref_measurement: Option<NodeId>,
}

pub struct Instance {
    pub layout: IndexMode,
    pub matrix_dim: Option<Vec<i32>>,
    pub read_write: bool,
    /// use as cache
    pub ref_typedef: Option<NodeId>,
    pub typedef_name: String,
}

#[add_node_field]
#[derive(Node)]
pub struct Annotation {
    pub label: String,
    pub origin: String,
}

#[add_node_field]
#[derive(Node)]
pub struct AnnotationText {
    pub text: String,
}

#[add_node_field]
#[derive(Node)]
pub struct ArComponent {
    pub component_type: String,
    pub prototype_of: String,
}

#[add_node_field]
#[derive(Node)]
pub struct AxisDescr {
    pub axis_pts_ref: String,
    pub axis_type: AxisType,
    pub byte_order: ByteOrderType,
    pub conversion: String,
    pub curve_axis_ref: String,
    pub deposit: DepositType,
    pub fix_axis_par: Option<FixAxisPar>,
    pub fix_axis_par_dist: Option<FixAxisParDist>,
    pub format: String,
    pub input_quantity: String,
    pub lower_limit: f64,
    pub lower_limit_ex: f64,
    pub upper_limit: f64,
    pub upper_limit_ex: f64,
    pub max_axis_points: i32,
    pub max_grad: f64,
    pub monotony: MonotonyType,
    pub read_only: bool,
    pub ref_axis_pts_ref: Option<NodeId>,
    pub ref_compu_method: Option<NodeId>,
    pub ref_curve_axis_ref: Option<NodeId>,
    pub ref_measurement: Option<NodeId>,
    pub step_size: f64,
    pub unit: String,
}

pub struct FixAxisPar {
    pub number_apo: i32,
    pub offset: f64,
    pub shift: f64,
}

pub struct FixAxisParDist {
    pub distance: f64,
    pub number_apo: i32,
    pub offset: f64,
}
#[add_node_field]
#[derive(Node)]
pub struct BitOperation {
    pub bit_operation: BitOperationType,
    pub count: i32,
    pub sign_extend: bool,
}
#[add_node_field]
#[derive(Node)]
pub struct CalibrationHandle {
    pub text: String,
    pub handles: Option<Vec<u32>>,
}
#[add_node_field]
#[derive(Node)]
pub struct CalibrationMethod {
    pub version: i32,
    pub method: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct CompuMethod {
    pub coeffs: Option<RationCoeffs>,
    pub compu_tab_ref: String,
    pub description: String,
    pub format: String,
    pub is_string: String,
    pub ref_compu_tab: Option<NodeId>, // TODO: may need edit
    pub status_string_ref: String,
    pub unit: String,
}

pub struct RationCoeffs {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl RationCoeffs {
    fn to_phys(rawVal: f64) -> f64 {
        let mut result = f64::NAN;
        // TODO:
        todo!();
        result
    }
    fn to_raw(date_type: DataType, phys_value: f64) -> f64 {
        todo!()
    }
    fn factor(&self) -> f64 {
        1.0 / self.b
    }
    fn offset(&self) -> f64 {
        -self.factor() * self.c
    }
}

/// TODO: delete this struct
pub struct CompuTabBase {
    pub conversion_type: ConversionType,
    pub default_value: String,
    pub description: String,
}

#[add_node_field]
#[derive(Node)]
pub struct CompuTab {
    pub default_value_numeric: f64,
    pub values: Option<Vec<(f32, f64)>>,
}

#[add_node_field]
#[derive(Node)]
pub struct CompuVTab {
    pub verbs: Option<Vec<(u64, u64)>>,
}

#[add_node_field]
#[derive(Node)]
pub struct CompuVTabRange {
    pub verbs: Option<Vec<(u64, u64)>>,
}

#[add_node_field]
#[derive(Node)]
pub struct Formula {
    pub formula: String,
    pub foumula_inv: String,
}

#[add_node_field]
#[derive(Node)]
pub struct FixAxisParList {
    pub values: Option<Vec<f64>>,
}
#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct Frame {
    pub description: String,
    pub rate: u64,
    /// Used as Cache
    pub referenced_nodes: Option<Vec<NodeId>>,
    pub references: Option<Vec<String>>,
    pub scaling_unit: i32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
/// TODO: delete this node
pub struct RefBase {
    pub description: String,
    pub referenced_nodes: Option<Vec<NodeId>>,
}

/// parent is refbase
#[add_node_field]
#[derive(Node)]
pub struct Function {
    pub referenced_nodes: Option<Vec<NodeId>>,
    pub version: String,
}

pub struct FunctionList {}

/// parent is refbase
pub struct Group {
    pub referenced_nodes: Option<Vec<NodeId>>,
    pub root: bool,
}

/// TODO:
pub struct InMeasurement {}

pub struct OutMeasurement {}

pub struct LocMeasurement {}

// TODO:IMemory
#[add_node_field]
#[derive(Node)]
pub struct MemoryLayout {
    pub address: u32,
    pub offsets: Option<Vec<i32>>,
    pub prg_type: PRGType,
    pub size: u32,
}
// TODO:IMemory
#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct MemorySegment {
    pub address: u32,
    pub description: String,
    pub memory_attribute: MemoryAttribute,
    pub memory_type: MemoryType,
    pub prg_type: MemoryPRGType,
    pub offsets: Option<Vec<i32>>,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct Module {
    pub description: String,
}

#[add_node_field]
#[derive(Node)]
pub struct ModCommon {
    pub alignments: Option<Vec<i32>>,
    pub byte_order: ByteOrderType,
    pub comment: String,
    pub data_size: i32,
    pub deposit: DepositType,
}

#[add_node_field]
#[derive(Node)]
pub struct ModPar {
    pub cpu: String,
    pub customer: String,
    pub customer_no: String,
    pub description: String,
    pub ecu: String,
    pub ecu_calibration_offset: u32,
    pub epk: String,
    pub epk_address: u32,
    // display name
    pub no_of_interfaces: i32,
    pub phone_no: String,
    pub supplier: String,
    pub system_constants: Option<HashMap<String, String>>,
    pub user: String,
    pub version: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct Overwrite {
    pub axis_no: u16,
    pub conversion: String,
    pub format: String,
    pub input_quantity: String,
    pub lower_limit: f64,
    pub lower_limit_ex: f64,
    pub upper_limit: f64,
    pub upper_limit_ex: f64,
    pub monotony: MonotonyType,
    pub phys_unit: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct Project {
    pub description: String,
}

#[add_node_field]
#[derive(Node)]
/// TODO: delete this node
pub struct Reference {
    pub description: String,
    pub display_name: String,
    pub referenced_nodes: Option<Vec<NodeId>>,
    pub references: Option<Vec<String>>,
}

/// parent is reference
pub struct RefCharacteristic {}

/// parent is reference
pub struct RefMeasurement {}

/// parent is reference
pub struct RefGroup {}

/// parent is reference
pub struct SubFunction {}

/// parent is reference
pub struct SubGroup {}

/// parent is reference
pub struct TransformerInObjects {}

/// parent is reference
pub struct TransformerOutObjects {}

/// parent is reference
pub struct Virtual {}
/// parent is reference
pub struct DependentCharacteristic {
    pub formula: String,
}

pub struct VirtualCharacteristic {
    pub foumula: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct StructureComponent {
    pub address_offset: u32,
    pub layout: IndexMode,
    pub matrix_dim: Option<i32>,
    pub ref_typedef: Option<NodeId>,
    pub symbol_type_link: String,
    pub typedef_name: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct Transformer {
    pub executable32: String,
    pub executable64: String,
    pub inverse_transformer: String,
    pub ref_inverse_transformer: Option<NodeId>,
    pub timeout: u32,
    pub trigger: TriggerType,
    pub version: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// TODO: delete this node
pub struct TypeDef {
    pub description: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct TypedefBlob {
    pub addr_type: AddrType,
    pub size: u32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct TypedefStructure {
    pub addr_type: AddrType,
    pub size: u32,
    pub consistent_exchange: bool,
    pub symbol_type_link: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct TypedefMeasurement {
    pub accuracy: f64,
    pub addr_type: AddrType,
    pub bit_mask: u64,
    pub data_type: DataType,
    pub discrete: bool,
    pub error_mask: u64,
    pub layout: IndexMode,
    pub matrix_dim: Option<Vec<i32>>,
    pub resolution: i32,
}

pub struct TypedefConversionRef {
    pub byte_order: ByteOrderType,
    pub conversion: String,
    pub format: String,
    pub lower_limit: f64,
    pub upper_limit: f64,
    pub phys_unit: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// TODO: delele this node
pub struct TypedefRecordLayoutRef {
    pub lower_limit_ex: f64,
    pub upper_limit_ex: f64,
    pub max_diff: f64,
    pub record_layout: String,
    pub step_size: f64,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// Parent is TypedefRecordLayoutRef
pub struct TypedefAxis {
    pub deposit: DepositType,
    pub input_quantity: String,
    pub max_axis_points: i32,
    pub monotony: MonotonyType,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct TypedefCharacteristic {
    pub bitmask: u64,
    pub char_type: CharacteristicType,
    pub discrete: bool,
    pub encoding: EncodingType,
    pub matrix_dim: Option<Vec<i32>>,
    pub number: i32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct Unit {
    pub description: String,
    pub display: String,
    pub ref_unit: String,
    pub si_exponents: Option<Vec<i32>>,
    pub unit_conversion: Option<Vec<f64>>,
    pub unit_type: UnitType,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct UserRights {
    pub readonly: bool,
}

#[add_node_field]
#[derive(Node)]
pub struct VarAddress {
    pub addresses: Option<Vec<u32>>,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct VarCharacteristic {
    pub criterion_values: Option<Vec<String>>,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct VarCriterion {
    pub criterion_values: Option<Vec<String>>,
    pub description: String,
    pub ref_var_measurement: Option<NodeId>,
    pub ref_var_selection_characteristic: Option<NodeId>,
    pub var_measurement: String,
    pub var_selection_characteristic: String,
}

#[add_node_field]
#[derive(Node)]
pub struct VarForbiddenComb {
    pub forbidden_dict: Option<HashMap<String, String>>,
}

#[add_node_field]
#[derive(Node)]
pub struct VariantCoding {
    pub var_naming: VarNamingType,
    pub var_separator: String,
}

/// Another apporaoch to impl inheritance
pub struct NodeGeneric<T> {
    pub nodedata: T,
}

pub struct NodeData {
    pub start_line: u32,
    pub end_line: u32,
    pub childs: Vec<Box<dyn Any>>,
}

pub struct NamedNodeData<T> {
    pub name: u32,
    pub node_data: T,
}

/// Use proc macro to add More NodeTrait
pub struct A2MLData {
    pub content: String,
}

pub struct AddressNodeData<T> {
    pub address: u32,
    pub address_ext: i32,
    pub calib_access: CalibrationAccess,
    pub description: String,
    pub max_refresh_rate: i32,
    pub max_refresh_unit: ScalingUnits,
    pub model_link: String,
    pub symbol_link: String,
    pub symbol_offset: i32,
    pub namenode_data: T,
}

pub struct NewBlob<T> {
    pub addr_type: AddrType,
    pub size: u32,
    pub address_node_data: T,
}

/// No, this kind of NodeGenric is terriable
impl NodeGeneric<NewBlob<AddressNodeData<NamedNodeData<NodeData>>>> {
    fn fxxx(&self) {
        let x = TypeId::of::<String>();
    }
}
