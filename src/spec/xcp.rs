use std::{any::Any, collections::HashMap};

use crate::spec::{NamedNode, Node, NodeType};
use asap2_derive::{add_namenode_field, add_node_field};
use indextree::NodeId;

use super::{ByteOrderType, ChecksumType};

pub enum ChecksumSxI {
    NotSet = -1,
    NoChecksum,
    ChecksumByte,
    ChecksumWord,
}

pub enum CommModeBasic {
    NotSet = -1, // may modify
    BigEndian = 1,
    AddressGranularityBYTE = 0,
    AddressGranularityWORD = 2,
    AddressGranularityDWORD = 4,
    SlaveBlockMode = 64,
    Optional = 128,
}

pub enum DtoCtrDaqMode {
    NotSet,
    InsertCounter,
    InsertSTIMCounterCopy,
}

pub enum DtoCtrStimMode {
    NotSet,
    DoNotCheckCounter,
    CheckCounter,
}

pub enum PAGProperties {
    NotSet = 0,
    FreezeSupported = 1,
}
pub enum ParityType {
    None,
    Odd,
    Even,
}

pub enum StopBitsType {
    OneStopBit = 1,
    TwoStopBits,
}

pub enum SyncModeSize {
    NotSet,
    Byte,
    Word,
    DWord = 4,
}

pub enum XCPAddressEXT {
    Free,
    ODT,
    DAQ,
}

pub enum XCPAddressMode {
    NotSet,
    DAQ,
    Event,
    ODT,
    None,
}

pub enum XCPAlignment {
    _8BIT = 1,
    _16BIT = 2,
    _32BIT = 4,
    _64BIT = 8,
}

pub enum XCPBlockMode {
    Standard = 0,
    Slave = 1,
    Master = 2,
    Interleaved = 4,
}

pub enum XCPClockEpoch {
    AtomicTime,
    UniversalCoordinatedTime,
    Arbitrary,
}

pub enum XCPClockMode {
    XCPSlaveClock,
    ECUClock,
    XCPSlaveGrandmasterClock,
    ECUGrandmasterClock,
}

pub enum XCPClockReadAbility {
    RandomlyReadable,
    LimitedReadable,
    NotReadable,
}

pub enum XCPClockSyncFeature {
    SynUnsupported,
    SynChronizationOnly,
    SynTonizationOnly,
    SynAll,
}

pub enum XCPDAQListCANSampleRate {
    NotSet,
    Single,
    Triple,
}

pub enum XCPDAQListCANType {
    NotSet,
    Variable,
    Fixed,
}

pub enum XCPDAQListType {
    NotSet,
    DAQ,
    STIM,
    DAQSTIM,
}

pub enum XCPDAQMode {
    Static,
    Dynamic,
}

pub enum XCPECUAccess {
    NotAllowed = 1,
    WithoutXCPOnly = 2,
    WithXCPOnly = 4,
    DontCare = 8,
}

pub enum XCPEndpoint {
    NotSet,
    In,
    Out,
}

pub enum XCPGroupMode {
    ElementGrouped,
    EventGrouped,
}

pub enum XCPHeaderLen {
    NotSet,
    Byte,
    CTRByte,
    FillByte,
    Word,
    CTRWord,
    FillWord,
}

pub enum XCPIDFieldType {
    Absolute,
    Byte,
    Word,
    Aligned,
}

pub enum XCPMemoryAccess {
    NotAllowed,
    Allowed,
}

pub enum XCPMessagePacking {
    Single,
    Multiple,
    Streaming,
}

pub enum XCPNativeTimestampSize {
    FourByte,
    EightByte,
}

pub enum XCPODTEntrySize {
    Byte = 1,
    Word = 2,
    DWord = 4,
    DLong = 8,
}

pub enum XCPOptimisationType {
    DEFAULT = 0,
    ODTType16 = 1,
    ODTType32 = 2,
    ODTType64 = 3,
    ODTTypeAlignment = 4,
    MaxEntrySize = 5,
}

pub enum XCPOverloadIND {
    NotSet,
    Indication,
    PID,
    Event,
}

pub enum XCPPacketAligment {
    _8,
    _16,
    _32,
    NotSet,
}

pub enum XCPPackMode {
    Optional,
    Mandatory,
}

pub enum XCPPGMMode {
    Absolute = 1,
    Functional = 2,
}

pub enum XCPReadWriteAccess {
    NotAllowed = 1,
    WithoutECUOnly = 2,
    WithECUOnly = 4,
    DontCare = 8,
}

pub enum XCPResourceState {
    NotActive,
    Active,
}

pub enum XCPSTSMode {
    Last,
    First,
}

pub enum XCPSyncEdge {
    NotSet,
    Single,
    Dual,
}

pub enum XCPTimestampResolution {
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
    _100PS,
}

pub enum XCPTimestampSize {
    NotSet,
    BYTE,
    WORD,
    DWORD = 4,
}

pub enum XCPTransceiverDelayCompensation {
    NotSet,
    On,
    Off,
}

pub enum XCPTransfer {
    BulkTransfer,
    InterruptTransfer,
}

pub enum XCPTSRelation {
    XCPSlaveClock,
    ECUClock,
}

#[add_node_field]
#[derive(Node)]
pub struct AddressMapping {
    pub dst_address: u32,
    pub length: u16,
    pub src_address: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct BufferReserve {
    pub odt_daq: u8,
    pub odt_stim: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct CANFD {
    pub btl_cycles: u8,
    pub data_transfer_baudrate: u32,
    pub max_dlc: u16,
    pub max_dlc_required: bool,
    pub sample_point: u8,
    pub secondary_sample_point: u8,
    pub sjw: u8,
    pub sync_edge: XCPSyncEdge,
    pub transceiver_delay_compensation: XCPTransceiverDelayCompensation,
}

#[add_node_field]
#[derive(Node)]
pub struct ChekcSum {
    pub check_sum: ChecksumType,
    pub external_function: String,
    pub max_block_size: u32,
    pub mta_block_size_align: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct Clock {
    pub epoch: XCPClockEpoch,
    pub feature: XCPClockSyncFeature,
    pub max_timestamp_value_before_wrap: u64,
    pub mode: XCPClockMode,
    pub quality: u8,
    pub readability: XCPClockReadAbility,
    pub uuid: Option<Vec<u8>>,
}

#[add_node_field]
#[derive(Node)]
pub struct CoreLoad {
    pub core_load: f32,
    pub core_nr: u32,
}

#[add_node_field]
#[derive(Node)]
pub struct DAQ {
    pub addr_ext: XCPAddressEXT,
    pub core_load_max_total: f32,
    pub cpu_load_max_total: f32,
    pub daq_alternating_supported: u16,
    pub daq_evt_dict: Option<HashMap<u16, DAQEvt>>,
    pub dto_ctr_field_supported: bool,
    pub id_field: XCPIDFieldType,
    pub max_daq: u16,
    pub max_daq_total: u16,
    pub max_evt_channel: u16,
    pub max_odt_daq_total: u16,
    pub max_odt_entries_daq_total: u16,
    pub max_odt_entries_stim_total: u16,
    pub max_odt_entries_total: u16,
    pub max_odt_entry_size: u8,
    pub max_odt_stim_total: u16,
    pub max_odt_total: u16,
    pub min_daq: u8,
    pub mode: XCPDAQMode,
    pub odt_entry_size: XCPODTEntrySize,
    pub opt_type: XCPOptimisationType,
    pub overload_ind: XCPOverloadIND,
    pub pid_off_supported: bool,
    pub prescaler_supported: bool,
    pub resume_supported: bool,
    pub store_daq_supported: bool,
}

pub struct DAQEvt {
    pub daq_list: Option<NodeId>,
    pub evt: Option<NodeId>,
}

#[add_node_field]
#[derive(Node)]
pub struct DAQList {
    pub active: bool,
    pub daq_list_type: XCPDAQListType,
    pub daq_no: u16,
    pub event_fixed: u16,
    pub first_pid: u8,
    pub max_odt: u8,
    pub max_odt_entries: u8,
    pub packed_mode_supported: bool,
}

#[add_node_field]
#[derive(Node)]
pub struct DAQListCANID {
    pub can_id: u32,
    pub daq_list_type: XCPDAQListCANType,
    pub daq_no: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct DAQListUSBEndpoint {
    pub daq_no: u16,
    pub ep_no: u8,
    pub ep_type: XCPEndpoint,
}

#[add_node_field]
#[derive(Node)]
pub struct DAQMemoryConsumption {
    pub daq_memory_limit: u32,
    pub daq_size: u16,
    pub odt_daq_buffer_element_size: u16,
    pub odt_entry_size: u16,
    pub odt_size: u16,
    pub odt_stim_buffer_element_size: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct ECUStatesMemoryAccess {
    pub read_access: XCPMemoryAccess,
    pub write_access: XCPMemoryAccess,
}

#[add_node_field]
#[derive(Node)]
pub struct ECUStatesState {
    pub cal_pag: XCPResourceState,
    pub daq: XCPResourceState,
    pub pgm: XCPResourceState,
    pub state_name: String,
    pub state_number: u8,
    pub stim: XCPResourceState,
}

/// TODO: delete this struct
pub struct EndPoint {
    pub alignment: XCPAlignment,
    pub ep_no: u8,
    pub ep_poll_internal: u8,
    pub host_buffer_size: u16,
    pub max_packet_size: u16,
    pub packing: XCPMessagePacking,
    pub transfer_type: XCPTransfer,
}

#[add_node_field]
#[derive(Node)]
pub struct InEPOnlyDAQ {}

#[add_node_field]
#[derive(Node)]
pub struct InEPOnlyEvServ {}

#[add_node_field]
#[derive(Node)]
pub struct InEPResErrDAQEvServ {}

#[add_node_field]
#[derive(Node)]
pub struct OutEPCmdStim {}

#[add_node_field]
#[derive(Node)]
pub struct OutEPCOnlyStim {}

#[add_node_field]
#[derive(Node)]
pub struct Event {
    pub complementary_bypass_event_channel_no: u16,
    pub consistency: XCPAddressMode,
    pub cpu_load_max: f32,
    pub daqlist_type: XCPDAQListType,
    pub dto_ctr_daq_mode: DtoCtrDaqMode,
    pub dto_ctr_daq_mode_fixed: bool,
    pub dto_ctr_stim_mode: DtoCtrStimMode,
    pub dto_ctr_stim_mode_fixed: bool,
    pub event_counter_present: bool,
    pub id: u16,
    pub max_daq_list: u8,
    pub name_long: String,
    pub name_short: String,
    pub priority: u8,
    pub realted_event_channel_number: u16,
    pub related_event_channel_number_fixed: bool,
    pub stim_dto_ctr_copy_present: bool,
    pub time_cycle: u8,
    pub time_unit: XCPTimestampResolution,
}

#[add_node_field]
#[derive(Node)]
pub struct EventCANIDList {
    pub evt_no: u16,
    pub fixed_can_ids: Option<Vec<u32>>,
}

#[add_node_field]
#[derive(Node)]
pub struct EventCPULoadConsumption {
    pub daq_factor: f32,
    pub odt_entry_factor: f32,
    pub odt_factor: f32,
}

#[add_node_field]
#[derive(Node)]
pub struct EventCPULoadConsumptionQueue {
    pub odt_element_load: f32,
    pub odt_factor: f32,
}

#[add_node_field]
#[derive(Node)]
pub struct EventDAQPackedMode {
    pub alt_sample_counts: Option<Vec<u16>>,
    pub group_mode: XCPGroupMode,
    pub packmode: XCPPackMode,
    pub sample_count: u16,
    pub sts_mode: XCPSTSMode,
}

#[add_node_field]
#[derive(Node)]
pub struct EventList {
    pub events: Option<Vec<u16>>,
}

#[add_node_field]
#[derive(Node)]
pub struct EventMinCycleTime {
    pub time_cycle: u8,
    pub time_unit: XCPTimestampResolution,
}

#[add_node_field]
#[derive(Node)]
pub struct EventODTEntrySizeFactorTable {
    pub size: u32,
    pub size_factor: f32,
}

#[add_node_field]
#[derive(Node)]
pub struct Framing {
    pub esc: u8,
    pub sync: u8,
}

#[add_node_field]
#[derive(Node)]
/// TODO: delete this node
pub struct Media {
    pub max_bus_load: u32,
    pub optional_tl_cmds: Option<Vec<String>>,
    pub transport_layer_instance: String,
}

#[add_node_field]
#[derive(Node)]
pub struct XCPonCAN {
    pub baudrate: u32,
    pub btl_cycles: u8,
    pub canid_broadcast: u32,
    pub canid_cmd: u32,
    pub canid_daq_clock_multicast: u32,
    pub canid_master_incremental: u32,
    pub canid_resp: u32,
    pub max_dlc_required: bool,
    pub measurement_split_allowed: bool,
    pub sample_point: u8,
    pub sample_rate: XCPDAQListCANSampleRate,
    pub sjw: u8,
    pub sync_edge: XCPSyncEdge,
}

/// TODO: delete this node
pub struct XCPOnEthernet {
    pub address: String,
    pub host_name: String,
    pub ipv6: String,
    pub max_bit_rate: u32,
    pub packet_alignment: XCPPacketAligment,
    pub port: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct XCPOnTCPIP {}
#[add_node_field]
#[derive(Node)]
pub struct XCPOnUDPIP {}
#[add_node_field]
#[derive(Node)]
pub struct XCPOnSxI {
    pub baudrate: u32,
    pub check_sum: ChecksumSxI,
    pub duplex_mode: Option<AsyncFullDuplexMode>,
    pub header_len: XCPHeaderLen,
    pub sync_full_duplex_mode: SyncModeSize,
    pub sync_master_slave_mode: SyncModeSize,
}

pub struct AsyncFullDuplexMode {
    pub parity: ParityType,
    pub stop_bits: StopBitsType,
}

#[add_node_field]
#[derive(Node)]
pub struct XCPOnUSB {
    pub alternate_setting_no: u8,
    pub header_len: XCPHeaderLen,
    pub interface_descriptor: String,
    pub number_of_if: u8,
    pub product_id: u16,
    pub vendor_id: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct ODT {
    pub odt_entries: Option<Vec<NodeId>>,
    pub odt_no: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct PAG {
    pub max_segments: u8,
    pub properties: PAGProperties,
}

#[add_node_field]
#[derive(Node)]
pub struct PAGE {
    pub access_ecu: XCPECUAccess,
    pub access_read: XCPReadWriteAccess,
    pub access_write: XCPReadWriteAccess,
    pub page_no: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct PGM {
    pub comm_nmodes_supported: Option<XCPCommModes>,
    pub max_cto: u8,
    pub max_sectors: u8,
    pub mode: XCPPGMMode,
}

pub struct XCPCommModes {
    pub comm_mode: XCPBlockMode,
    pub max_bs: u8,
    pub min_st: u8,
    pub queue_size: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct ProtocolLayer {
    pub byte_order: ByteOrderType,
    pub comm_mode_basic: CommModeBasic,
    pub comm_modes_supported: Option<XCPCommModes>,
    pub max_cto: u8,
    pub max_dto: u16,
    pub max_dto_stim: u16,
    pub oprional_cmds: Option<Vec<String>>,
    pub seed_key_extrnal_function: String,
    pub timings: Option<Vec<u16>>,
    pub version: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct Sector {
    pub address: u32,
    pub clear_seq_no: u8,
    pub length: u32,
    pub name_long: String,
    pub number: u8,
    pub pgm_method: u8,
    pub pgm_seq_no: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct Segment {
    pub address_extension: u8,
    pub compression_method: u8,
    pub default_page_number: u8,
    pub encryption_method: u8,
    pub no_of_pages: u8,
    pub pgm_verify: u32,
    pub segment_no: u8,
}

#[add_node_field]
#[derive(Node)]
pub struct Stim {
    pub bit_stim_supported: bool,
    pub max_odt_entry_size: u8,
    pub min_st_stim: u8,
    pub odt_entry_size: XCPODTEntrySize,
}

#[add_node_field]
#[derive(Node)]
pub struct TimestampCharacterization {
    pub resolution: XCPTimestampResolution,
    pub size: XCPNativeTimestampSize,
    pub timestamp_ticks: u32,
}

#[add_node_field]
#[derive(Node)]
pub struct TimestampSupported {
    pub is_fixed: bool,
    pub resolution: XCPTimestampResolution,
    pub size: XCPTimestampSize,
    pub ticks: u16,
}

#[add_node_field]
#[derive(Node)]
pub struct TimeCorrelation {
    pub timestamp_relateto: XCPTSRelation,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node, NamedNode)]
pub struct DAQEvent {
    pub events: Option<Vec<u16>>,
}
