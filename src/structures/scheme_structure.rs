use serde::{Serialize, Deserialize};


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

