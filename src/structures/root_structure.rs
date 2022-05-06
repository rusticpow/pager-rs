use bincode::{config, Decode, Encode};

#[derive(Decode, Encode, PartialEq, Debug)]
pub struct RootStructure {
    scheme_pid: u32,
    events_pid: u32,
    data_pid: u32,
    free_pid: u32,
    reserve0: u8,
    reserve1: u8,
    reserve2: u16,
    reserve4: u32,
    reserve5: u32,
    reserve6: u32
}

pub struct RootStructureInit {
    scheme_pid: u32,
    events_pid: u32,
    data_pid: u32,
    free_pid: u32,
}

impl RootStructure {
    pub fn new(initial: &RootStructureInit) -> Self {
        RootStructure {
            scheme_pid: initial.scheme_pid,
            events_pid: initial.events_pid,
            data_pid: initial.data_pid,
            free_pid: initial.free_pid,
            reserve0: 0,
            reserve1: 0,
            reserve2: 0,
            reserve4: 0,
            reserve5: 0,
            reserve6: 0,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let config = config::standard().with_fixed_int_encoding();
        let encoded: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();

        encoded
    }

    pub fn from_slice(buffer: &[u8]) -> Self {
        let config = config::standard().with_fixed_int_encoding();
        let (decoded_struct, len): (RootStructure, usize) =
            bincode::decode_from_slice(&buffer, config).unwrap();

        decoded_struct
    }
}

#[cfg(test)]

mod tests {
    use crate::structures::root_structure::RootStructureInit;

    use super::RootStructure;

    #[test]
    fn test_serialize() {
        let root_struct = RootStructure::new(&RootStructureInit {
            scheme_pid: 1,
            events_pid: 2,
            data_pid: 3,
            free_pid: 4,
        });

        let encoded = root_struct.to_vec();
        let len = encoded.len();
        assert_eq!(len, 32);

        let decoded_struct = RootStructure::from_slice(encoded.as_slice());
        assert_eq!(root_struct, decoded_struct);
    }
}
