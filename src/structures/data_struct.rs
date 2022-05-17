use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataStruct {
    pub col_ids: Vec<u32>,
    pub columns: Vec<ColumnStruct>,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnStruct {
    pub record_ids: Vec<u128>,
    pub integer_values: Vec<i64>,
}

impl DataStruct {
    pub fn write_to_file(&self, file: &mut File) {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut s).unwrap();

        file.write_all(s.view()).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> DataStruct {
        if file.metadata().unwrap().len() == 0 {
            return Self {
                col_ids: vec![],
                columns: vec![],
            };
        }

        let mut vec: Vec<u8> = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        let r = flexbuffers::Reader::get_root(vec.as_slice()).unwrap();

        Self::deserialize(r).unwrap()
    }
}
