pub enum DataFileType
{
    IntelHex,
    MotorolaS,
    Binary
}

pub enum MotorolaFileType
{
    /// Use original file format
    NotSet,
    /// S3 Records (4 byte addresses)
    _4Byte = 3,
    /// S2 Records (3 byte addresses)
    _3Byte = 2,
    /// S1 Records (2 byte addresses)
    _2Byte = 1
}