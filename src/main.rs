pub mod page_io;

use std::{
    mem,
    os::unix::prelude::FileExt,
    path::Path
};

pub const HEADER_SIZE: usize = 12;
pub const BODY_SIZE: usize = 4084;
pub const PAGE_SIZE: usize = 4096;

fn main() {
    println!("Hello, world!");

    let file_name = "hello.db";
    let path = Path::new(file_name);
    let display = path.display();

    

  
}

struct Pointer {
    right: u32,
}

struct PageU32Body {
    items: Vec<u32>,
}

impl PageU32Body {
    fn from(body: &[u8; BODY_SIZE]) -> PageU32Body {
        let u32_size = mem::size_of::<u32>();
        let mut body_elements = vec![0; BODY_SIZE / u32_size];
        let max_elements = BODY_SIZE / u32_size;

        let mut b_index: usize = 0;
        for i in 0..(max_elements) {
            let start: usize = b_index;
            let end = start + 4;
            let a: &[u8] = &body[start..end];
            let vec = <&[u8; 4]>::try_from(a).unwrap();

            let value = u32::from_le_bytes(*vec);
            body_elements[i] = value;
            b_index += 4;
        }

        PageU32Body {
            items: body_elements,
        }
    }
}

struct Page {
    header: PageHeader,
    body: PageU32Body,
}

impl Page {
    fn u32from(buffer: Vec<u8>) -> Page {
        let vec = &buffer[1..5];
        let column_id_bytes = <&[u8; 4]>::try_from(vec).unwrap();
        let body = <&[u8; BODY_SIZE]>::try_from(&buffer[12..PAGE_SIZE]).unwrap();

        Page {
            header: PageHeader {
                page_type: PageType::from(buffer[0]),
                column_id: u32::from_le_bytes(*column_id_bytes),
            },
            body: PageU32Body::from(body),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![0; PAGE_SIZE];
        buf[0] = self.header.page_type as u8;

        let column_id_in_bytes = self.header.column_id.to_le_bytes();
        buf[1] = column_id_in_bytes[0];
        buf[2] = column_id_in_bytes[1];
        buf[3] = column_id_in_bytes[2];
        buf[4] = column_id_in_bytes[3];

        
        let mut index = 12;
        for i in &self.body.items {
            let value_bytes = i.to_le_bytes();

            buf[index] = value_bytes[0];
            buf[index + 1] = value_bytes[1];
            buf[index + 2] = value_bytes[2];
            buf[index + 3] = value_bytes[3];
             
            index += 4;
        }

        buf
    }
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
            _ => panic!("Invalid page type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::{Page, PageHeader, PageType, PageU32Body, BODY_SIZE};

    #[test]
    fn serialization_desiralization_check() {
        let mut items = vec![0; BODY_SIZE / mem::size_of::<u32>()];
        items[0] = 3; // 3, 4, 99876, 4
        items[1] = 4;
        items[2] = 99876;
        items[3] = 4;
        items[1020] = 21;

        let bytes = Page {
            header: PageHeader {
                page_type: PageType::Leaf,
                column_id: u32::MAX,
            },
            body: PageU32Body {
                items: items,
            },
        }
        .serialize();

        let page = Page::u32from(bytes);

        assert_eq!(page.header.column_id, u32::MAX);
        matches!(page.header.page_type, PageType::Leaf);

        assert_eq!(page.body.items.len(), BODY_SIZE / mem::size_of::<u32>());
        assert_eq!(page.body.items[0], 3);
        assert_eq!(page.body.items[1], 4);
        assert_eq!(page.body.items[2], 99876);
        assert_eq!(page.body.items[3], 4);
        assert_eq!(page.body.items[1020], 21);
    }
}
