use std::{any::Any, collections::HashMap};

use crate::spec::{NamedNode, Node, NodeType};
use asap2_derive::{add_namenode_field, add_node_field};
use indextree::NodeId;

use super::{xcp::XCPDAQListType, ByteOrderType, ChecksumType};

pub enum CCPAddressMode {
    NotSet,
    DAQ,
    ODT,
}

pub enum CCPChecksumCalcType {
    NotSet,
    ActivePage,
    BitOrWithOPTPage,
}

pub enum CCPDAQMode {
    NotSet,
    Alternating,
    Burst,
}

pub enum CCPMemoryPageType {
    NotSet = 0,
    RAM = 1,
    ROM = 2,
    Flash = 4,
    EEPROM = 8,
    RAMInitByECU = 16,
    RAMInitByTool = 32,
    AutoFlashBack = 64,
    FlashBack = 128,
    Default = 256,
}

pub enum CCPScalingUnit {
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
    NonDeterministic,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct CANParam {
    pub btr0: u8,
    pub btr1: u8,
    pub frequency: u16,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct CheckSum {
    pub dll: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct CheckSumParam {
    pub calc_type: CCPChecksumCalcType,
    pub checksum_type: ChecksumType,
    pub limit: u32,
    pub procedure: u16,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct DefinedPages {
    pub address: u32,
    pub address_ext: u8,
    pub length: u32,
    pub no: u16,
    pub page_name: String,
    pub page_type: CCPMemoryPageType,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct EventGroup {
    pub name_long: String,
    pub name_short: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct CCPNamedNode {
    pub sub_type: NodeType,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct AddressMapping {
    pub sub_type: NodeType,
    pub base_address: u32,
    pub length: u32,
    pub map_address: u32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct DPBlob {
    pub sub_type: NodeType,
    pub address: u32,
    pub address_ext: u16,
    pub size: u32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct KPBlob {
    pub sub_type: NodeType,
    pub address: u32,
    pub address_ext: u16,
    pub size: u32,
    pub rasters: Option<Vec<u16>>,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct QPBlob {
    pub can_id: u32,
    pub daq_list_type: XCPDAQListType,
    pub daq_no: u16,
    pub first_pid: u8,
    pub length: u16,
    pub raster: u8,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct Raster {
    pub evt_channel_no: u8,
    pub name_long: String,
    pub name_short: String,
    pub rate: u32,
    pub scaling_unit: CCPScalingUnit,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
/// ccp node base add name to struct
pub struct SeedKey {
    pub cal_dll: String,
    pub daq_dll: String,
    pub pgm_dll: String,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct Source {
    pub active: bool,
    pub name_long: String,
    pub rate: u32,
    pub scaling_unit: CCPScalingUnit,
}

#[add_node_field]
#[add_namenode_field]
#[derive(NamedNode, Node)]
pub struct TPBlob {
    pub address_ext: CCPAddressMode,
    pub baudrate: u32,
    pub blob_version: u16,
    pub byte_order: ByteOrderType,
    pub canid_cmd: u32,
    pub canid_resp: u32,
    pub consisteny: CCPAddressMode,
    pub daq_mode: CCPDAQMode,
    pub optional_cmds: Option<Vec<u8>>,
    pub station_address: u16,
    pub version: u16,
    pub source_raster_dict: Option<HashMap<u16, SourceRaster>>,
}

pub struct SourceRaster {
    pub source: Option<NodeId>,
    pub raster: Option<NodeId>,
}
