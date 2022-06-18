use indextree::NodeId;

use crate::spec::CharacteristicType;

pub enum DataFileType {
    IntelHex,
    MotorolaS,
    Binary,
}

pub enum MotorolaFileType {
    /// Use original file format
    NotSet,
    /// S3 Records (4 byte addresses)
    _4Byte = 3,
    /// S2 Records (3 byte addresses)
    _3Byte = 2,
    /// S1 Records (2 byte addresses)
    _2Byte = 1,
}

pub enum ValueObjectFormat
{
    Physical,
    Raw,
    RawHex,
    RawBin
}

pub trait CharValue {
    fn axis_value(&self)->Option<&Vec<Vec<f64>>>;
    fn axis_value_mut(&mut self)->Option<&mut Vec<Vec<f64>>>;

    fn char_type(&self)->Option<&CharacteristicType>;
    fn char_type_mut(&mut self)->Option<&mut CharacteristicType>;

    fn characteristic(&self)->Option<NodeId>;

    fn decimal_count(&self)->Option<i32>;
    fn decimal_count_mut(&mut self)->Option<&mut i32>;

    fn decimal_count_axis(&self)->Option<&Vec<i32>>;
    fn decimal_count_axis_mut(&mut self)->Option<&mut Vec<i32>>;

    fn unit(&self)->Option<&String>;
    fn unit_mut(&mut self)->Option<&mut String>;

    fn unit_axis(&self)->Option<&Vec<String>>;
    fn unit_axis_mut(&mut self)->Option<&mut Vec<String>>;

    // fn value<T>(&self)->Option<&T>;
    // fn value_mut<T>(&mut self)->Option<&mut T>;

    fn value_format(&self)->Option<&ValueObjectFormat>;
    fn value_format_mut(&mut self)->Option<&mut ValueObjectFormat>;
}

pub struct CharValueData{}


/// DataConservation
pub trait DataExchangeObject {
    
}

pub struct DCMFile {
    pub values: Vec<Box<dyn CharValue>>,
    pub eol:String,
}

impl DCMFile {
    
}

pub struct CDFFile {

}

pub struct MatFile {

}

pub struct ParFile {

}

