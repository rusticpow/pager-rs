use flexbuffers::to_vec;

use crate::page_manager::file_io::{BODY_OFFSET, HEADER_CAPACITY};

use super::{file_io::PAGE_SIZE, page_header::Header};

struct PageBuilder {}

impl PageBuilder {
    /// Build page in bytes with header and body (which length can be vary and must equals Header.body_size) and returns PAGE_SIZE length vector
    fn build(header: &Header, body: Vec<u8>) -> Vec<u8> {
        assert_eq!(body.len() as u16, header.body_size);

        let mut page: Vec<u8> = vec![0; PAGE_SIZE];

        let header_vec = header.to_vec();
        assert_eq!(HEADER_CAPACITY, header_vec.len());

        for (i, v) in header_vec.iter().enumerate() {
            page[i] = *v;
        }

        for (i, v) in body.iter().enumerate() {
            page[i + BODY_OFFSET] = *v;
        }

        page
    }

    /// Read the whole page and return Header and body vector (that will equal the length set on Header.body_size)
    fn separate(page: Vec<u8>) -> (Header, Vec<u8>) {
        let header = Header::new(&page);
        let mut body: Vec<u8> = vec![0; header.body_size.into()];

        for (i, v) in page[BODY_OFFSET..(BODY_OFFSET + header.body_size as usize)]
            .iter()
            .enumerate()
        {
            body[i] = *v;
        }

        (header, body)
    }
}

#[cfg(test)]
mod tests {
    use crate::page_manager::{
        file_io::{PageType, BODY_OFFSET, PAGE_SIZE},
        page_builder::PageBuilder,
        page_header::Header,
    };

    #[test]
    fn build_when_header_and_body_less_than_page_size_fill_zeros_return_page_size() {
        let page_vec = PageBuilder::build(
            &Header {
                page_type: PageType::Data,
                next_page_id: 2,
                body_size: 3000,
            },
            vec![1; 3000],
        );

        assert_eq!(PAGE_SIZE, page_vec.len());
        assert_eq!(1, page_vec[BODY_OFFSET + 2999]);
        assert_eq!(0, page_vec[BODY_OFFSET + 3000]);

        let (header, body) = PageBuilder::separate(page_vec);

        assert_eq!(3000, body.len());
        assert_eq!(1, body[0]);
        assert_eq!(1, body[2999]);
        assert_eq!(PageType::Data, header.page_type);
        assert_eq!(2, header.next_page_id);
        assert_eq!(3000, header.body_size);
    }
}
