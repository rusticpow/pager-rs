use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
    os::unix::prelude::FileExt,
    path::Path,
};

const PAGE_SIZE: usize = 4096;

fn main() {
    println!("Hello, world!");

    let file_name = "hello.db";
    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let page_size = 4096;

    let mut buf: &[u8] = &[0, 0, 0, 1, 1];
    file.write_at(buf, 0);
}

fn serialize_page(page: Page) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0; PAGE_SIZE];
    buf[0] = page.header.page_type as u8;

    let column_id_in_bytes = page.header.column_id.to_le_bytes();
    buf[1] = column_id_in_bytes[0];
    buf[2] = column_id_in_bytes[1];
    buf[3] = column_id_in_bytes[2];
    buf[4] = column_id_in_bytes[3];

    buf
}

fn deserialize_to_page(buffer: Vec<u8>) -> Page {
    let vec = &buffer[1..5];
    let column_id_bytes = <&[u8; 4]>::try_from(vec).unwrap();

    Page {
        header: PageHeader {
            page_type: PageType::from(buffer[0]),
            column_id: u32::from_le_bytes(*column_id_bytes),
        },
    }
}

struct Pointer {
    right: u32,
}

struct Page {
    header: PageHeader,
}

struct PageHeader {
    page_type: PageType,
    column_id: u32,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]

enum PageType {
    Index = 0,
    Leaf = 1,
}

impl PageType {
    fn from(b: u8) -> PageType {
        match b {
            0 => PageType::Index,
            1 => PageType::Leaf,
            _ => panic!("Invalid page type")
        }
    }
}

impl Page {}

#[cfg(test)]
mod tests {
    use crate::{deserialize_to_page, serialize_page, Page, PageHeader, PageType};

    #[test]
    fn serialization_desiralization_check() {
        let bytes = serialize_page(Page {
            header: PageHeader {
                page_type: PageType::Leaf,
                column_id: u32::MAX,
            },
        });

        let page = deserialize_to_page(bytes);

        assert_eq!(page.header.column_id, u32::MAX);
        matches!(page.header.page_type, PageType::Leaf);
    }
}
