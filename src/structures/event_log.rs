use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventLog {
    pub log: Vec<EventRecord>,
}

#[derive(Serialize, Deserialize)]
pub struct EventRecord {
    pub timestamp: u64,
    pub col_id: u32,
    pub record_id: u128,
    pub integer_value: Option<i64>,
}

impl EventLog {
    pub fn write_to_file(&self, file: &mut File) {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut s).unwrap();

        file.write_all(s.view()).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> Self {
        if file.metadata().unwrap().len() == 0 {
            return Self {
                log: vec![],
            };
        }

        let mut vec: Vec<u8> = Vec::new();
        file.read_to_end(&mut vec).unwrap();
        let r = flexbuffers::Reader::get_root(vec.as_slice()).unwrap();

        Self::deserialize(r).unwrap()
    }
}
