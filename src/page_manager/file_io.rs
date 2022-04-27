use core::panic;
use std::{
    fs::{File, OpenOptions},
};

use flexbuffers::{Builder, Reader};
use read_write_at::{ReadAtMut, WriteAt};

const UNIT_EXTENSION: &str = ".unit";

pub const PAGE_SIZE: usize = 4096;
pub const HEADER_CAPACITY: usize = 23; // 1 + 23 bytes, where the first bytes is ubyte with size of header
pub const BODY_CAPACITY: usize = 4072;

pub trait FileIO {
    fn read(&mut self, page_i: u64) -> Result<Vec<u8>, &str>;
    fn write(
        &self,
        page_type: PageType,
        content_body: &[u8],
        page_i: Option<u64>,
    ) -> Result<u64, ()>;
}

pub struct FileIOImpl {
    file_path: String,
    file: File,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PageType {
    Data = 0u8,
    Scheme = 1u8,
}

struct Header {
    page_type: PageType,
    next_page_id: u64,
    body_size: u16,
}

impl Header {
    fn new(page_buf: &[u8]) -> Self {
        let header_size = page_buf[0];
        let header_slice = &page_buf[1..(header_size as usize) + 1];
        let header_vector = Reader::get_root(header_slice).unwrap().as_vector();

        Header {
            page_type: match header_vector.idx(0).as_u8() {
                0 => PageType::Data,
                1 => PageType::Scheme,
                _ => panic!(),
            },
            next_page_id: header_vector.idx(1).as_u64(),
            body_size: header_vector.idx(2).as_u16(),
        }
    }

    fn write_into(&self, page_buf: &mut [u8]) {
        let mut builder = Builder::default();
        let mut header_structure = builder.start_vector();

        // Use `push` to add elements to a vector or map. Note that it up to the programmer to ensure
        // duplicate keys are avoided and the key has no null bytes.
        header_structure.push(self.page_type as u8);
        header_structure.push(self.next_page_id as u64);
        header_structure.push(self.body_size as u16);

        header_structure.end_vector();

        let page_header = builder.view();

        let page_header_len = page_header.len();
        if page_header_len > HEADER_CAPACITY {
            panic!("page_header_len is more than HEADER_CAPACITY");
        }

        // header size
        let mut page_offset = 0;
        page_buf[page_offset] = page_header_len as u8;

        page_offset = 1;
        for i in page_offset..(page_header_len + page_offset) {
            page_buf[i] = page_header[i - page_offset];
        }
    }
}

impl FileIOImpl {
    pub fn new(unit_name: &str) -> FileIOImpl {
        let file_path = get_file_path(unit_name);

        let result = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(file_path.as_str());

        let file = match result {
            Ok(file) => file,
            Err(_) => panic!(), // TODO: temporary
        };

        FileIOImpl {
            file_path: file_path,
            file: file,
        }
    }
}

impl<'a> FileIO for FileIOImpl {
    fn read(&mut self, start_page_id: u64) -> Result<Vec<u8>, &str> {
        let mut page_index = start_page_id;
        let mut total_buf: Vec<u8> = Vec::new();

        loop {
            let page_buf: &mut [u8] = &mut [0; PAGE_SIZE];
            self.file
                .read_exact_at(page_buf, page_index * PAGE_SIZE as u64)
                .expect("Page reading failed");

            let header = Header::new(page_buf);

            let body_buf = &page_buf
                [(1 + HEADER_CAPACITY as usize)..(1 + HEADER_CAPACITY + header.body_size as usize)];

            total_buf.append(&mut body_buf.to_vec());

            page_index = header.next_page_id;
            if page_index == 0 {
                break;
            }
        }

        Ok(total_buf)
    }

    fn write(
        &self,
        page_type: PageType,
        content_body: &[u8],
        start_page_id: Option<u64>,
    ) -> Result<u64, ()> {
        let mut page_body_chunks: Vec<Vec<u8>> = Vec::new();

        let mut content_chunks: Vec<&[u8]> = Vec::new();
        fill_chunks(content_body, &mut content_chunks, BODY_CAPACITY);

        for chunk in content_chunks {
            let mut page_body_chunk = [0u8; BODY_CAPACITY];
            for (index, item) in chunk.iter().enumerate() {
                page_body_chunk[index] = item.clone();
            }
            page_body_chunks.push(page_body_chunk.to_vec());
        }

        let mut next_page_id: u64 = 0;
        let mut body_size = BODY_CAPACITY as u16;

        let free_page_identifiers = get_free_page_identifiers(&self.file, page_body_chunks.len());
        for (index, page_body) in page_body_chunks.iter().enumerate() {
            if page_body_chunks.len() == 1 {
                next_page_id = 0;
            } else {
                if index == page_body_chunks.len() - 1 {
                    next_page_id = 0;
                    body_size = (content_body.len() % BODY_CAPACITY) as u16;
                    if body_size == 0 {
                        body_size = BODY_CAPACITY as u16;
                    }
                } else {
                    next_page_id = free_page_identifiers[index + 1];
                    body_size = BODY_CAPACITY as u16;
                }
            }

            let header = Header {
                page_type,
                next_page_id,
                body_size,
            };

            let page_buf: &mut [u8] = &mut [0; PAGE_SIZE];

            header.write_into(page_buf);

            // header_size + header_capacity
            let page_offset = 1 + HEADER_CAPACITY;
            for i in page_offset..(BODY_CAPACITY + page_offset) {
                page_buf[i] = page_body[i - page_offset];
            }

            self.file
                .write_all_at(&page_buf, free_page_identifiers[index] * PAGE_SIZE as u64)
                .expect("Page writing failed");
        }

        Ok(free_page_identifiers[0])
    }
}

fn fill_chunks<'a>(content_body: &'a [u8], chunks: &mut Vec<&'a [u8]>, chunk_size: usize) {
    (*chunks) = content_body.chunks(chunk_size).collect();
}

fn get_file_path(unit_name: &str) -> String {
    let mut owned_string: String = unit_name.to_owned();
    owned_string.push_str(UNIT_EXTENSION);

    owned_string
}

fn get_free_page_identifiers(file: &File, length: usize) -> Vec<u64> {
    let size = file.metadata().unwrap().len();
    let pages = (size as f64 / PAGE_SIZE as f64).ceil() as u64;

    let mut result = vec![];
    for id in pages..length as u64 {
        result.push(id);
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use ulid::Ulid;

    use crate::page_manager::file_io::{FileIOImpl, PageType};

    use super::{fill_chunks, FileIO, BODY_CAPACITY};
    
    #[test]
    fn write_read_one_page_data() {
        let mut file_io = FileIOImpl::new(Ulid::new().to_string().as_str());

        let page_content: &mut [u8] = &mut [0; BODY_CAPACITY];
        page_content[0] = 5;
        page_content[4071] = 10;

        let result = file_io
            .write(PageType::Scheme, page_content, Some(0))
            .unwrap();

        let page = file_io.read(result).unwrap();

        assert_eq!(page[0], 5);
        assert_eq!(page[4071], 10);

        fs::remove_file(file_io.file_path).unwrap();
    }

    #[test]
    fn write_read_multiple_page_data() {
        let mut file_io = FileIOImpl::new(Ulid::new().to_string().as_str());

        let page_content: &mut [u8] = &mut [0; BODY_CAPACITY + 100];
        page_content[0] = 5;
        page_content[4071] = 10;
        page_content[4171] = 200;

        let result = file_io
            .write(PageType::Scheme, page_content, Some(0))
            .unwrap();

        let page = file_io.read(result).unwrap();

        assert_eq!(page[0], 5);
        assert_eq!(page[4071], 10);
        assert_eq!(page[4171], 200);

        fs::remove_file(file_io.file_path).unwrap();
    }

    #[test]
    fn fill_chunks_multiple() {
        let mut chunks: Vec<&[u8]> = Vec::new();
        let chunk_size = BODY_CAPACITY;
        let content_body: &mut [u8] = &mut [0; BODY_CAPACITY * 2 + 100];

        fill_chunks(content_body, &mut chunks, chunk_size);

        assert_eq!(3, chunks.len());
        assert_eq!(100, chunks[2].len());
    }
}
