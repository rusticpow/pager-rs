use super::file_io::{PagesPointer, Structure, BODY_CAPACITY, PAGE_SIZE};

pub struct PagesPointerImpl {}

impl PagesPointer for PagesPointerImpl {
    fn get_identifiers(file_size: u64, pages_len: usize, structure: &Structure) -> Vec<u64> {
        if structure.pages.len() != 0
            && structure.pages.len()
                == f32::ceil(structure.content.len() as f32 / BODY_CAPACITY as f32) as usize
        {
            return structure.pages.to_vec();
        }

        let free_pages = get_free_page_identifiers(file_size, pages_len);
        free_pages
    }
}

fn get_free_page_identifiers(file_size: u64, length: usize) -> Vec<u64> {
    let pages = (file_size as f64 / PAGE_SIZE as f64).ceil() as u64;

    let mut result = vec![];
    for id in pages..(length as u64 + pages) as u64 {
        result.push(id);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::PagesPointerImpl;
    use crate::page_manager::file_io::{PagesPointer, Structure};

    #[test]
    fn get_identifiers_when_file_size_zero_and_no_structure_pages_return_new_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            0,
            3,
            &Structure {
                content: vec![],
                pages: vec![],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![0, ids[0]];
        assert_eq![1, ids[1]];
        assert_eq![2, ids[2]];
    }

    #[test]
    fn get_identifiers_when_file_has_size_and_no_structure_pages_return_new_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            4096,
            3,
            &Structure {
                content: vec![],
                pages: vec![],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![1, ids[0]];
        assert_eq![2, ids[1]];
        assert_eq![3, ids[2]];
    }

    #[test]
    fn get_identifiers_with_structure_pages_equals_content_size_pages_return_structure_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            4096 * 6,
            3,
            &Structure {
                content: vec![0; 4072 * 3],
                pages: vec![8, 7, 6],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![8, ids[0]];
        assert_eq![7, ids[1]];
        assert_eq![6, ids[2]];
    }
}
