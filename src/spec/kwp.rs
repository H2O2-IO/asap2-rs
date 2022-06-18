use std::{any::Any, collections::HashMap, fs::OpenOptions};

use crate::spec::{NamedNode, Node, NodeType};
use asap2_derive::{add_namenode_field, add_node_field};
use indextree::NodeId;

use super::ByteOrderType;

pub enum AddressLocation {
    Intern,
    Extern,
}

pub enum KWPCopyMode {
    RAMInitByECU,
    RAMInitByTool,
}

pub enum KWPDataAccessFlags {
    ReadData = 1,
    VerifyCode = 2,
    ReadCode = 4,
    RWOnlyOnActivePage = 8,
}

pub enum KWPFlashMode {
    NoFlashBack,
    AutoFlashBack,
    ToolFlashBack,
}

pub enum KWPFlashResult {
    RequestRoutineResults,
    StartRoutine,
    CodedResult,
}

pub enum KWPMeasurementMode {
    AddressMode,
    BlockMode,
}

pub enum KWPPageSwitchMode {
    EscapeCode,
    LocalRoutine,
}

pub enum KWPPhysicalLayer {
    NotSet,
    KLine,
    CAN,
    KLineOnCAN,
    KLineAndCAN,
}

pub enum KWPStimulationMode {
    NotSet,
    _WuP,
    _5Baud,
}

pub enum KWPVersion {
    NotSet,
    VDA1996,
}

#[add_node_field]
#[derive(Node)]
pub struct Address {
    pub can_id_ecu:u32,
    pub can_id_tester:u32,
    pub filter_from_write:bool,
}

#[add_node_field]
#[derive(Node)]
pub struct CAN {
    pub baudrate:u32,
    pub btl_cycles:u8,
    pub filter_from_write:bool,
    pub network_limits:Option<NetworkLimits>,
    pub sample_count:u8,
    pub sjw_length:u8,
    pub start_stop_routine_no:u16,
    pub sync_edge:u8,
}

pub struct NetworkLimits {
    pub wft_max:u8,
    pub xdl_max:u16,
}

#[add_node_field]
#[derive(Node)]
pub struct CheckSum {
    pub checksum_type:u32,
    pub local_routine_no:u16,
    pub only_on_active_page:bool,
    pub result:KWPFlashResult,
    pub rnc_result:Option<Vec<u8>>,
}

#[add_node_field]
#[derive(Node)]
pub struct Copy {
    pub copy_para:Option<Vec<u8>>,
    pub flash_to_ram_diag_mode:u8,
    pub flash_to_ram_mode:KWPCopyMode,
}

#[add_node_field]
#[derive(Node)]
pub struct DiagBaud {
    pub baudrate:u32,
    pub bd_para:Option<Vec<u8>>,
    pub diag_mode:u16,
}

#[add_node_field]
#[derive(Node)]
pub struct Flash {
    pub copy_frame:Option<Vec<u8>>,
    pub copy_para:Option<Vec<u8>>,
    pub ram_to_flash_mode:KWPFlashMode,
    pub ram_to_flash_routine_no:u16,
    pub resutl:KWPFlashResult,
    pub rnc_result:Option<Vec<u8>>,
}

/// parent is Flash
#[add_node_field]
#[derive(Node)]
pub struct FlashCopy {
    pub copy_frame:Option<Vec<u8>>,
    pub copy_para:Option<Vec<u8>>,
    pub ram_to_flash_mode:KWPFlashMode,
    pub ram_to_flash_routine_no:u16,
    pub resutl:KWPFlashResult,
    pub rnc_result:Option<Vec<u8>>,
    pub flash_to_ram_diag_mode:u8,
    pub flash_to_ram_mode:KWPCopyMode,
}

#[add_node_field]
#[derive(Node)]
pub struct KLine {
    pub ecu_address:u16,
    pub stimulation_mode:KWPStimulationMode,
    pub tester_address:u16,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
pub struct AddressMapping {
    pub sub_type:NodeType,
    pub base_address:u32,
    pub length:u32,
    pub map_address:u32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
pub struct DPBlob {
    pub sub_type:NodeType,
    pub address:u32,
    pub length:u32,
}

#[add_node_field]
#[add_namenode_field]
#[derive(Node)]
pub struct KPBlob {
    pub sub_type:NodeType,
    pub address:u32,
    pub location:AddressLocation,
    pub size:u32,
}

#[add_node_field]
#[derive(Node)]
pub struct PageSwitch {
    pub escape_code_para_get:Option<Vec<u8>>,
    pub escape_code_para_set:Option<Vec<u8>>,
    pub mode:KWPPageSwitchMode,
    pub page_code:Option<Vec<u8>>,
}

#[add_node_field]
#[derive(Node)]
pub struct RoutinePara {
    pub local_routine_no:u16,
    pub result:KWPFlashMode,
    pub rnc_result:Option<Vec<u8>>,
}

#[add_node_field]
#[derive(Node)]
pub struct Source {
    pub cse_unit:u16,
    pub name_long:String,
    pub qp_blob:Option<QPBlob>,
    pub rate:i32,
}

pub struct QPBlob {
    pub block_model_id:u16,
    pub can_id:u32,
    pub discriminator:u16,
    pub max_no_of_bytes:u16,
    pub max_no_of_singnals:u16,
    pub max_sampling_rate:u16,
    pub measurement_mode:KWPMeasurementMode,
    pub no_of_samplings:u16,
    pub physical_layer:KWPPhysicalLayer,
}

#[add_node_field]
#[derive(Node)]
pub struct TimeDef {
    pub time_defs:Option<Vec<KWPTimeDef>>,
    pub usdtp_timings:Option<Vec<KWPUSDTPTiming>>,
}

pub struct KWPTimeDef {
    pub p1_max:u16,
    pub p2_max:u16,
    pub p2_min:u16,
    pub p3_max:u16,
    pub p3_min:u16,
    pub p4_min:u16,
}

pub struct KWPUSDTPTiming {
    pub para_as:u16,
    pub para_bs:u16,
    pub para_cd:u16,
}

#[add_node_field]
#[derive(Node)]
pub struct TPBlob {
    pub baud_defs:Option<Vec<BaudDef>>,
    pub byte_order:ByteOrderType,
    pub data_access:Option<KWPDataAccess>,
    pub ecu_access:u16,
    pub e_version:KWPVersion,
    pub kline:KWPKLine,
    pub project_base_address:u32,
    pub security_access: Option<Vec<SecurityAccess>>,
    pub seram:KWPSeram,
    pub start_diag_without_bd_switch:bool,
    pub stimulation_mode:KWPStimulationMode,
    pub tester_address:u16,
    pub time_defs:Option<Vec<KWPTimeDef>>,
    pub version:u16,
}

pub struct BaudDef {
    pub baud_rate:u32,
    pub diag_mode:u16,
    pub ik:u32,
}

pub struct KWPDataAccess {
    pub adr_flash:u32,
    pub adr_ram:u32,
    pub flags:KWPDataAccessFlags,
}

pub struct KWPKLine {
    pub ecu_address:u16,
    pub stimulation_mode:KWPStimulationMode,
    pub tester_address:u16,
}

pub struct KWPSeram {
    pub adr_flash:u32,
    pub adr_ram:u32,
    pub flags:KWPDataAccessFlags,
    pub u32_0:u32,
    pub u32_1:u32,
    pub u32_2:u32,
    pub u32_3:u32,
}

pub struct SecurityAccess {
    pub access_mode:u16,
    pub calc_mode:u16,
    pub delay_time:u16,
}