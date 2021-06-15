use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Rank {
    Challenger = 1,
    Master = 2,
    Diamond = 3,
    Platinum = 4,
    Gold = 5,
    Silver = 6,
    Bronze = 7,
    Overall = 8,
    Unknown = 9,
    PlatinumPlus = 10,
    DiamondPlus = 11,
    Iron = 12,
    Grandmaster = 13,
    MasterPlus = 14,
    Diamond2Plus = 15,
}
