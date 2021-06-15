use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Region {
    Na1 = 1,
    Euw1 = 2,
    Kr = 3,
    Eun1 = 4,
    Br1 = 5,
    La1 = 6,
    La2 = 7,
    Oc1 = 8,
    Ru = 9,
    Tr1 = 10,
    Jp1 = 11,
    World = 12,
}
