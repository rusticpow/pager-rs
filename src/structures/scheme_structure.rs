use serde::{Serialize, Deserialize};

pub struct SetValue {
    pub col_id: u32,
    pub record_id: u128,
    pub integer_value: i64
}