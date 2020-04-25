pub type NodeId = u32;
pub type EdgeId = u32;
pub type Letter = char;
pub type StrLength = u32;
pub type NodeLength = i32;

pub const ROOT_ID: NodeId = 0;
pub const SOURCE_ID: NodeId = 1;
pub const NONE_SINK_ID: u32 = std::u32::MAX;