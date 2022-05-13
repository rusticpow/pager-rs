use crate::structures::data_structure::{DataLeaf, DataStructure, DataValue, NodeType};

use super::{file_rw::FileApi, page_builder::PageBuilder, page_header::Header, file_io::PageType};

struct DataPageInsertImpl {}

impl DataPageInsertImpl {
    fn insert(&self, file_api: &impl FileApi, value: DataValue) -> Result<(), &str> {
        let data_root = file_api.data_root(value.cell.col_id);
        if data_root.is_none() {
            let structure = DataStructure::from_leaf(DataLeaf {
                identifiers: vec![value.cell.record_id],
                integer_values: vec![value.integer_value],
            }).to_vec();

            let length = structure.len();
            // сначала оценить количество страниц
            PageBuilder::build(&Header {
                page_type: PageType::Data,
                next_page_id: 0,
                body_size: structure.len() as u16,
            }, structure.to_vec());

            let free_page_pid = file_api.free_pids();
            file_api.write_regular_page();

        }

        let scheme = data_root.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        page_manager::file_rw::FileApi,
        structures::{
            data_structure::{DataStructure, DataValue},
            root_structure::RootStructure,
            scheme_structure::UnitScheme,
        },
        unit_scheme::unit_scheme::RecordCell,
    };

    use super::DataPageInsertImpl;

    #[test]
    fn when_no_data_root_create_data_leaf_update_scheme() {
        let file_api = FileApiFakeImpl {
            data_root: None,
            free_pids: vec![2],
            regular_pages: HashMap::new(),
        };

        DataPageInsertImpl {}.insert(
            &file_api,
            DataValue {
                cell: RecordCell {
                    record_id: 101,
                    col_id: 102,
                },
                integer_value: 99,
            },
        );

        assert_eq!(, )
    }

    struct FileApiFakeImpl {
        free_pids: Vec<u32>,
        data_root: Option<DataStructure>,
        regular_pages: HashMap<u32, Vec<u8>>,
    }

    impl FileApi for FileApiFakeImpl {
        fn root_pid(&self) -> u8 {
            todo!()
        }

        fn write_root_pid(&self, _value: u8) -> Result<(), &str> {
            todo!()
        }

        fn write_root_page(&self, _pid: u8, _value: &[u8]) -> Result<(), &str> {
            todo!()
        }

        fn root_page_vec(&self, _pid: u8) -> Vec<u8> {
            todo!()
        }

        fn regular_page_vec(&self, pid: u32) -> Vec<u8> {
            todo!()
        }

        fn root(&self) -> Option<RootStructure> {
            todo!()
        }

        fn scheme(&self) -> Option<UnitScheme> {
            todo!()
        }

        fn data_root(&self, col_id: u32) -> Option<DataStructure> {
            self.data_root
        }

        fn write_regular_page(&self, pid: u32, value: &[u8]) -> Result<(), &str> {
            self.regular_pages.insert(pid, value.to_vec());

            Ok(())
        }

        fn free_pids(&self, chunks_length: usize) -> Vec<u32> {
            todo!()
        }
    }
}
