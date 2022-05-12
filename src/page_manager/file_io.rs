use core::panic;
use read_write_at::{ReadAtMut, WriteAt};
use std::fs::{File, OpenOptions};

use super::page_header::Header;

const UNIT_EXTENSION: &str = ".unit";

pub const PAGE_SIZE: usize = 4096;
pub const HEADER_CAPACITY: usize = 23; // 1 + 23 bytes, where the first bytes is ubyte with size of header
pub const BODY_OFFSET: usize = HEADER_CAPACITY + 1;
pub const BODY_CAPACITY: usize = 4072;

pub trait FileIO {
    fn read(&mut self, start_page_id: u64) -> Result<Structure, &str>;
    fn write(
        &self,
        page_pointer: &impl PagesPointer,
        page_type: PageType,
        structure: &Structure,
    ) -> Result<Option<u64>, ()>;
}

pub trait PagesPointer {
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

pub struct FileIOImpl {
    file_path: String,
    file: File,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PageType {
    Data = 0u8,
    Scheme = 1u8,
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
    fn read(&mut self, start_page_id: u64) -> Result<Structure, &str> {
        let mut page_index = start_page_id;
        let mut total_buf: Vec<u8> = Vec::new();
        let mut pages: Vec<u64> = Vec::new();
        loop {
            pages.push(page_index);

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

        Ok(Structure {
            content: total_buf,
            pages: StructurePages { value: pages },
        })
    }

    fn write(
        &self,
        pages_pointer: &impl PagesPointer,
        page_type: PageType,
        structure: &Structure,
    ) -> Result<Option<u64>, ()> {
        let mut page_body_chunks: Vec<Vec<u8>> = Vec::new();

        fill_chunks(&structure.content, &mut page_body_chunks, BODY_CAPACITY);

        let page_identifiers = pages_pointer.get_identifiers(
            self.file.metadata().unwrap().len(),
            page_body_chunks.len(),
            &structure.pages,
        );

        for (index, page_body) in page_body_chunks.iter().enumerate() {
            let next_page_id = if page_identifiers.len() - 1 >= index + 1 {
                page_identifiers[index + 1]
            } else {
                0
            };

            let body_size = get_body_size(index, page_body_chunks.len(), structure.content.len());

            let page_buf: &mut [u8] = &mut [0; PAGE_SIZE];

            Header {
                page_type,
                next_page_id,
                body_size,
            }
            .write_into(page_buf);

            // header_size + header_capacity
            let page_offset = 1 + HEADER_CAPACITY;
            for i in page_offset..(BODY_CAPACITY + page_offset) {
                page_buf[i] = page_body[i - page_offset];
            }

            self.file
                .write_all_at(&page_buf, page_identifiers[index] * PAGE_SIZE as u64)
                .expect("Page writing failed");
        }

        Ok(if page_identifiers.len() > 0 {
            Some(page_identifiers[0])
        } else {
            None
        })
    }
}

fn get_body_size(
    current_index: usize,
    page_body_chunks_length: usize,
    content_body_length: usize,
) -> u16 {
    if current_index == page_body_chunks_length - 1 {
        let mut body_size = (content_body_length % BODY_CAPACITY) as u16;
        if body_size == 0 {
            body_size = BODY_CAPACITY as u16;
        }

        return body_size;
    }

    return BODY_CAPACITY as u16;
}

fn fill_chunks<'a>(content_body: &'a [u8], page_body_chunks: &mut Vec<Vec<u8>>, chunk_size: usize) {
    let content_chunks: Vec<&[u8]> = content_body.chunks(chunk_size).collect();

    for chunk in content_chunks {
        let mut page_body_chunk = [0u8; BODY_CAPACITY];
        for (index, item) in chunk.iter().enumerate() {
            page_body_chunk[index] = item.clone();
        }
        page_body_chunks.push(page_body_chunk.to_vec());
    }
}

fn get_file_path(unit_name: &str) -> String {
    let mut owned_string: String = unit_name.to_owned();
    owned_string.push_str(UNIT_EXTENSION);

    owned_string
}

#[cfg(test)]
mod tests {
    use std::fs;

    use ulid::Ulid;

    use crate::page_manager::file_io::{
        FileIOImpl, PageType, PagesPointer, Structure, StructurePages,
    };

    use super::{fill_chunks, FileIO, BODY_CAPACITY};

    struct FakePagePointer {
        identifiers: Vec<u64>,
    }

    impl PagesPointer for FakePagePointer {
        fn get_identifiers(
            &self,
            _file_size: u64,
            _pages_length: usize,
            _structure: &StructurePages,
        ) -> Vec<u64> {
            self.identifiers.to_vec()
        }
    }

    #[test]
    fn write_read_one_page_data_with_full_body() {
        let mut file_io = FileIOImpl::new(Ulid::new().to_string().as_str());

        let page_content: &mut [u8] = &mut [0; BODY_CAPACITY];
        page_content[0] = 5;
        page_content[4071] = 10;

        let result = file_io
            .write(
                &FakePagePointer {
                    identifiers: vec![0],
                },
                PageType::Scheme,
                &Structure {
                    content: page_content.to_vec(),
                    pages: StructurePages { value: vec![0u64] },
                },
            )
            .unwrap();

        let page = file_io.read(result.unwrap()).unwrap();

        assert_eq!(page.content[0], 5);
        assert_eq!(page.content[4071], 10);

        fs::remove_file(file_io.file_path).unwrap();
    }

    #[test]
    fn write_read_one_page_data_with_partial_data() {
        let mut file_io = FileIOImpl::new(Ulid::new().to_string().as_str());

        let page_content: &mut [u8] = &mut [0; 300];
        page_content[0] = 5;
        page_content[299] = 10;

        let result = file_io
            .write(
                &FakePagePointer {
                    identifiers: vec![0],
                },
                PageType::Scheme,
                &Structure {
                    content: page_content.to_vec(),
                    pages: StructurePages { value: vec![0u64] },
                },
            )
            .unwrap();

        let page = file_io.read(result.unwrap()).unwrap();

        assert_eq!(page.content.len(), 300);
        assert_eq!(page.content[0], 5);
        assert_eq!(page.content[299], 10);

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
            .write(
                &FakePagePointer {
                    identifiers: vec![0, 1],
                },
                PageType::Scheme,
                &Structure {
                    content: page_content.to_vec(),
                    pages: StructurePages { value: vec![0u64] },
                },
            )
            .unwrap();

        let page = file_io.read(result.unwrap()).unwrap();

        assert_eq!(page.content[0], 5);
        assert_eq!(page.content[4071], 10);
        assert_eq!(page.content[4171], 200);

        fs::remove_file(file_io.file_path).unwrap();
    }

    #[test]
    fn write_replace_pages_with_same_structure() {
        let mut file_io = FileIOImpl::new(Ulid::new().to_string().as_str());

        {
            let page_content: &mut [u8] = &mut [0; BODY_CAPACITY * 2 + 100];
            page_content[0] = 5;
            page_content[4071] = 10;
            page_content[4171] = 200;
            page_content[8243] = 255;

            let result = file_io
                .write(
                    &FakePagePointer {
                        identifiers: vec![0, 1, 2],
                    },
                    PageType::Scheme,
                    &Structure {
                        content: page_content.to_vec(),
                        pages: StructurePages { value: vec![0u64] },
                    },
                )
                .unwrap()
                .unwrap();

            let page = file_io.read(result).unwrap();

            assert_eq!(result, 0);
            assert_eq!(page.content[0], 5);
            assert_eq!(page.content[4071], 10);
            assert_eq!(page.content[4171], 200);
            assert_eq!(page.content[8243], 255);
        }

        {
            let page_content: &mut [u8] = &mut [0; BODY_CAPACITY * 2 + 300];
            page_content[0] = 5;
            page_content[4071] = 10;
            page_content[4171] = 100;
            page_content[8443] = 155;

            let result = file_io
                .write(
                    &FakePagePointer {
                        identifiers: vec![0, 1, 2],
                    },
                    PageType::Scheme,
                    &Structure {
                        content: page_content.to_vec(),
                        pages: StructurePages { value: vec![0u64] },
                    },
                )
                .unwrap()
                .unwrap();

            let page = file_io.read(result).unwrap();

            assert_eq!(result, 0);
            assert_eq!(8444, page.content.len());
            assert_eq!(page.content[0], 5);
            assert_eq!(page.content[4071], 10);
            assert_eq!(page.content[4171], 100);
            assert_eq!(page.content[8443], 155);
        }

        fs::remove_file(file_io.file_path).unwrap();
    }

    #[test]
    fn fill_chunks_multiple() {
        let mut chunks: Vec<Vec<u8>> = Vec::new();
        let chunk_size = BODY_CAPACITY;
        let content_body: &mut [u8] = &mut [0; BODY_CAPACITY * 2 + 100];

        fill_chunks(content_body, &mut chunks, chunk_size);

        assert_eq!(3, chunks.len());
        assert_eq!(4072, chunks[2].len());
    }
}
