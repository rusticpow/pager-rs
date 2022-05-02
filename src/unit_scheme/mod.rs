use flexbuffers::Builder;
use serde::{Deserialize, Serialize};

pub mod unit_scheme_apply;

#[derive(Serialize, Deserialize)]
pub struct UnitScheme {
    pub tables: Vec<UnitTable>,
}

impl UnitScheme {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut s).unwrap();

        s.view().to_vec()
    }

    pub fn read_from(buffer: &[u8]) -> UnitScheme {
        let r = flexbuffers::Reader::get_root(buffer).unwrap();
        UnitScheme::deserialize(r).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnitTable {
    pub name: String,
    pub columns: Vec<UnitColumn>,
}

#[derive(Serialize, Deserialize)]
pub enum UnitColumnType {
    Integer,
    Double,
    String,
}

#[derive(Serialize, Deserialize)]
pub struct UnitColumn {
    pub id: u32,
    pub name: String,
    pub col_type: UnitColumnType,
}

pub struct TableRecord<'a> {
    pub container_name: String,
    pub table_name: String,
    pub id: Option<String>,
    pub columns: Vec<TableRecordColumn<'a>>,
}

pub struct TableRecordColumn<'a> {
    pub name: String,
    pub col_type: UnitColumnType,
    pub string_value: Option<&'a str>,
    pub numeric_value: Option<&'a i64>,
}
