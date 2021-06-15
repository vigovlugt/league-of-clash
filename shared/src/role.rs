use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize_repr, Clone, Copy, Serialize_repr)]
#[repr(i64)]
pub enum Role {
    Jungle = 1,
    Supp = 2,
    Adc = 3,
    Top = 4,
    Mid = 5,
    None = 6,
}

impl Role {
    pub fn from_i64(value: i64) -> Self {
        match value {
            1 => Self::Jungle,
            2 => Self::Supp,
            3 => Self::Adc,
            4 => Self::Top,
            5 => Self::Mid,
            _ => Self::None,
        }
    }
}
