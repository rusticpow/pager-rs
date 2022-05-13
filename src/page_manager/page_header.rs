use flexbuffers::{Reader, Builder};

use super::file_io::{HEADER_CAPACITY, PageType, PAGE_SIZE};

pub struct Header {
    pub page_type: PageType,
    pub next_page_id: u64,
    pub body_size: u16,
}

impl Header {
    pub fn new(page_buf: &[u8]) -> Self {
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

    pub fn write_into(&self, page_buf: &mut [u8]) {
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

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = vec![0; HEADER_CAPACITY];

        self.write_into(&mut vec);

        vec
    }
}
