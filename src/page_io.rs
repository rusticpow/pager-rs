use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
};

use read_write_at::{ReadAt, WriteAt};

struct PageIO {
    db_name: String,
}

impl PageIO {
    pub fn new(db_name: &str) -> PageIO {
        PageIO { db_name: String::from(db_name) }
    }

    pub fn write(&self, page: &[u8], position: u32) {
        if page.len() != crate::PAGE_SIZE {
            panic!(
                "PAGE size must be: {}, given:{}",
                crate::PAGE_SIZE,
                page.len()
            );
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("{}.db", self.db_name))
            .unwrap();

        file.write_all_at(page, PageIO::get_offset(position))
            .unwrap();
    }

    pub fn read(&self, buffer: &mut [u8], position: u32) {
        let file = OpenOptions::new()
            .read(true)
            .open(format!("{}.db", self.db_name))
            .unwrap();

        let offset = PageIO::get_offset(position);
        // let mut buf: [u8; crate::PAGE_SIZE] = [0; crate::PAGE_SIZE];
        file.read_at(buffer, offset).unwrap();
    }

    fn get_offset(position: u32) -> u64 {
        u64::from(position) * (crate::PAGE_SIZE as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::{Page, PageHeader, PageType, PageU32Body, BODY_SIZE};

    use super::PageIO;

    #[test]
    fn read_write() {
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
            body: PageU32Body { items: items },
        }
        .serialize();

        let page_io = PageIO::new("some_db");
        page_io.write(&bytes, 0);
        page_io.write(&bytes, 1);
        page_io.write(&bytes, 2);
        page_io.write(&bytes, 3);

        let mut buffer = [0; crate::PAGE_SIZE];

        page_io.read(&mut buffer, 3);

        assert_eq!(bytes, buffer.to_vec());

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
