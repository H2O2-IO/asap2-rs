pub mod ccp;
pub mod xcp;

pub enum ConnectBehaviourType
{
    Automatic,
    Manual
}

pub enum EPKCheckResult
{
    Equal,
    NotEqual,
    Failed,
    NotApplicable
}

pub enum StartStopMode
{
    Stop,
    Start,
    Select
}

