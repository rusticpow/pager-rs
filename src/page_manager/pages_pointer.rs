use super::file_io::{PagesPointer, StructurePages, PAGE_SIZE};

pub struct PagesPointerImpl {}

impl PagesPointer for PagesPointerImpl {
    fn get_identifiers(
        file_size: u64,
        content_chunks_len: usize,
        structure_pages: &StructurePages,
    ) -> Vec<u64> {
        let file_pages = (file_size as f64 / PAGE_SIZE as f64).ceil() as u64;

        let s_pages_len = structure_pages.value.len();
        let mut struct_local_pages: Vec<u64> = vec![0u64; s_pages_len];
        struct_local_pages.copy_from_slice(&structure_pages.value);
        struct_local_pages.sort();

        if s_pages_len > 0 {
            if struct_local_pages.last().copied().unwrap() > file_pages - 1 {
                return get_free_page_identifiers(file_pages, content_chunks_len);
            }

            if s_pages_len == content_chunks_len {
                return structure_pages.value.to_vec();
            }

            if s_pages_len < content_chunks_len {
                let mut pages: Vec<u64> = Vec::new();

                let mut free_pages = get_free_page_identifiers(
                    file_pages,
                    content_chunks_len - struct_local_pages.len(),
                );
                pages.append(&mut struct_local_pages.clone());
                pages.append(&mut free_pages);

                return pages;
            }
        }

        if struct_local_pages.len() > content_chunks_len {
            return struct_local_pages.to_vec();
        }

        get_free_page_identifiers(file_pages, content_chunks_len)
    }
}

fn get_free_page_identifiers(file_pages: u64, length: usize) -> Vec<u64> {
    let mut result = vec![];
    for id in file_pages..(length as u64 + file_pages) as u64 {
        result.push(id);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::PagesPointerImpl;
    use crate::page_manager::file_io::{PagesPointer, StructurePages, PAGE_SIZE};

    #[test]
    fn get_identifiers_when_file_size_zero_and_no_structure_pages_return_new_pages() {
        let ids = PagesPointerImpl::get_identifiers(0, 3, &StructurePages { value: vec![] });

        assert_eq![3, ids.len()];
        assert_eq![0, ids[0]];
        assert_eq![1, ids[1]];
        assert_eq![2, ids[2]];
    }

    #[test]
    fn get_identifiers_when_file_has_size_and_no_structure_pages_return_new_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64,
            3,
            &StructurePages { value: vec![] },
        );

        assert_eq![3, ids.len()];
        assert_eq![1, ids[0]];
        assert_eq![2, ids[1]];
        assert_eq![3, ids[2]];
    }

    #[test]
    fn get_identifiers_with_structure_pages_equals_content_size_pages_return_structure_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 6,
            3,
            &StructurePages {
                value: vec![8, 7, 6],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![6, ids[0]];
        assert_eq![7, ids[1]];
        assert_eq![8, ids[2]];
    }

    #[test]
    fn get_identifiers_with_structure_pages_less_than_content_size_pages_return_structure_pages_plus_free_pages(
    ) {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 6,
            4,
            &StructurePages { value: vec![3, 4] },
        );

        assert_eq![4, ids.len()];
        assert_eq![3, ids[0]];
        assert_eq![4, ids[1]];
        assert_eq![6, ids[2]];
        assert_eq![7, ids[3]];
    }

    #[test]
    fn get_identifiers_with_structure_page_more_than_content_size_pages_return_structure_pages() {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 7,
            2,
            &StructurePages {
                value: vec![3, 4, 5, 6],
            },
        );

        assert_eq![4, ids.len()];
        assert_eq![3, ids[0]];
        assert_eq![4, ids[1]];
        assert_eq![5, ids[2]];
        assert_eq![6, ids[3]];
    }

    #[test]
    fn get_identifiers_when_structure_page_id_more_than_file_size_pages_return_free_pages() {
        // this means that something went wrong that was not specified.
        // it can be panic for bug catching reasons, but more appropriate to be able to continue in working order
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 4,
            4,
            &StructurePages {
                value: vec![2, 4, 5, 6],
            },
        );

        assert_eq![4, ids.len()];
        assert_eq![4, ids[0]];
        assert_eq![5, ids[1]];
        assert_eq![6, ids[2]];
        assert_eq![7, ids[3]];
    }

    #[test]
    fn get_identifiers_when_content_chunks_len_zero_structure_pages_len_zero_return_empty() {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 4,
            0,
            &StructurePages { value: vec![] },
        );

        assert_eq![0, ids.len()];
    }

    #[test]
    fn get_identifiers_when_content_chunks_len_zero_but_has_structure_pages_len_return_structure_pages(
    ) {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 4,
            0,
            &StructurePages {
                value: vec![1, 2, 3],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![1, ids[0]];
        assert_eq![2, ids[1]];
        assert_eq![3, ids[2]];
    }
}
