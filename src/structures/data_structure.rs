use serde::{Deserialize, Serialize};

use crate::unit_scheme::unit_scheme::RecordCell;

#[derive(Serialize, Deserialize)]
pub struct DataStructure {
    pub node_type: NodeType,
    pub root: Option<DataRoot>,
    pub intermediate: Option<DataIntermediate>,
    pub leaf: Option<DataLeaf>,
}

impl DataStructure {
    pub fn from_leaf(leaf: DataLeaf) -> Self {
        DataStructure {
            node_type: NodeType::Leaf,
            root: None,
            intermediate: None,
            leaf: Some(leaf),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut s).unwrap();

        s.view().to_vec()
    }

    pub fn read_from(buffer: &[u8]) -> DataStructure {
        let r = flexbuffers::Reader::get_root(buffer).unwrap();
        DataStructure::deserialize(r).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataRoot {
    pub identifiers: Vec<u128>,
    pub pids: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct DataIntermediate {
    pub identifiers: Vec<u128>,
    pub pids: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct DataLeaf {
    pub identifiers: Vec<u128>,
    pub integer_values: Vec<i64>,
}

pub struct DataValue {
    pub cell: RecordCell,
    pub integer_value: i64,
}

#[derive(Serialize, Deserialize)]
pub enum NodeType {
    Root = 0,
    Intermediate = 1,
    Leaf = 2,
}
