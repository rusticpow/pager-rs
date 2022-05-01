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

        if file_pages > 0
            && s_pages_len > 0
            && struct_local_pages.last().copied().unwrap() > file_pages - 1
        {
            panic!("structure_pages have id that can not be placed in file");
        }

        let mut pages: Vec<u64> = Vec::new();

        if s_pages_len > content_chunks_len {
            return struct_local_pages;
        }

        let mut free_pages = get_free_page_identifiers(
            file_pages,
            content_chunks_len - struct_local_pages.len(),
            struct_local_pages.clone(),
        );
        pages.append(&mut struct_local_pages.clone());
        pages.append(&mut free_pages);
        pages.sort();

        return pages;
    }
}

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `start_from` - inclusive
fn get_free_page_identifiers(start_from: u64, length: usize, existing_pages: Vec<u64>) -> Vec<u64> {
    let mut result = vec![];

    if length == 0 {
        return result;
    }

    let mut start = start_from;
    loop {
        if existing_pages.contains(&start) {
            start += 1;

            continue;
        }

        result.push(start);

        if result.len() == length {
            return result;
        }

        start += 1;
    }
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
                value: vec![3, 4, 2],
            },
        );

        assert_eq![3, ids.len()];
        assert_eq![2, ids[0]];
        assert_eq![3, ids[1]];
        assert_eq![4, ids[2]];
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

    #[test]
    fn get_identifiers_when_file_size_zero_return_structure_and_free_pages() {
        let ids = PagesPointerImpl::get_identifiers(0, 2, &StructurePages { value: vec![0, 1, 2] });

        assert_eq![3, ids.len()];
        assert_eq![0, ids[0]];
        assert_eq![1, ids[1]];
        assert_eq![2, ids[2]];
    }

    #[test]
    fn get_identifiers_when_structure_pages_has_some_of_existing_return_mix_of_structure_and_free_pages(
    ) {
        let ids = PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 4,
            5,
            &StructurePages { value: vec![1, 3] },
        );

        assert_eq![5, ids.len()];
        assert_eq![1, ids[0]];
        assert_eq![3, ids[1]];
        assert_eq![4, ids[2]];
        assert_eq![5, ids[3]];
        assert_eq![6, ids[4]];
    }

    #[test]
    #[should_panic]
    fn get_identifiers_when_structure_pages_contains_not_existing_no_zero_page_panic() {
        PagesPointerImpl::get_identifiers(
            PAGE_SIZE as u64 * 4,
            5,
            &StructurePages {
                value: vec![1, 3, 4],
            }, // id 4 cannot be exists, because
        );
    }
}
