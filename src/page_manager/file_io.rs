use core::panic;
use read_write_at::{ReadAtMut, WriteAt};
use std::fs::{File, OpenOptions};

use super::page_header::Header;

const UNIT_EXTENSION: &str = ".unit";

pub const PAGE_SIZE: usize = 4096;
pub const HEADER_CAPACITY: usize = 23; // 1 + 23 bytes, where the first bytes is ubyte with size of header
pub const BODY_OFFSET: usize = HEADER_CAPACITY + 1;
pub const BODY_CAPACITY: usize = 4072;

pub trait FreePages {
    fn get_identifiers(
        &self,
        file_size: u64,
        pages_length: usize,
        structure: &StructurePages,
    ) -> Vec<u64>;
}

pub struct StructurePages {
    pub value: Vec<u64>,
}

pub struct Structure {
    pub content: Vec<u8>,
    pub pages: StructurePages,
}


#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PageType {
    Data = 0u8,
    Scheme = 1u8,
}