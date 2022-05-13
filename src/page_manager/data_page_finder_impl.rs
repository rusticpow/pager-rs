use std::option::Option;

use crate::{
    structures::{root_structure::RootStructure, scheme_structure::UnitScheme},
    unit_scheme::unit_scheme::{RecordCell, SetEvent},
};

use super::file_rw::{DataPageFinder, FileApi};

struct DataPageFinderImpl {}

impl DataPageFinder for DataPageFinderImpl {
    fn find_page(file_api: &impl FileApi, cell: &RecordCell) -> Option<(Vec<u8>, u32)> {
        let root_pid = file_api.root_pid();
        let root = RootStructure::from_slice(&file_api.root_page_vec(root_pid));

        let scheme = UnitScheme::read_from(&file_api.regular_page_vec(root.scheme_pid));
        scheme.tables.iter();

        Some((vec![], 0))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        page_manager::{
            file_rw::{DataPageFinder, FileApi},
            page_header::Header,
        },
        structures::{
            root_structure::{RootStructure, RootStructureInit},
            scheme_structure::UnitScheme, data_structure::DataStructure,
        },
        unit_scheme::unit_scheme::RecordCell,
    };

    use super::DataPageFinderImpl;

    fn find_page_when_record_is_in_alone_leaf() {
        let header = Header {
            page_type: todo!(),
            next_page_id: todo!(),
            body_size: todo!(),
        };

        let file_api = FileApiFakeImpl {
            root_pid: 0,
            root_page: RootStructure::new(&RootStructureInit {
                scheme_pid: 0,
                events_pid: 0,
                events_last_pid: 0,
                free_pid: 0,
                vpm_pid: 0,
            }),
            regular_pages: HashMap::from([(2, UnitScheme { tables: todo!() }.to_vec())]),
        };

        DataPageFinderImpl::find_page(
            &file_api,
            &RecordCell {
                record_id: todo!(),
                col_id: todo!(),
            },
        );
    }

    struct FileApiFakeImpl {
        root_pid: u8,
        root_page: RootStructure,
        regular_pages: HashMap<u32, Vec<u8>>,
    }

    impl FileApi for FileApiFakeImpl {
        fn root_pid(&self) -> u8 {
            self.root_pid
        }

        fn write_root_pid(&self, value: u8) -> Result<(), &str> {
            todo!()
        }

        fn write_root_page(&self, pid: u8, value: &[u8]) -> Result<(), &str> {
            todo!()
        }

        fn write_regular_page(&self, pid: u32, value: &[u8]) -> Result<(), &str> {
        todo!()
    }

        fn root_page_vec(&self, pid: u8) -> Vec<u8> {
            self.root_page.to_vec()
        }

        fn regular_page_vec(&self, pid: u32) -> Vec<u8> {
            self.regular_pages[&pid].to_owned()
        }

        fn root(&self) -> Option<RootStructure> {
            todo!()
        }

        fn scheme(&self) -> Option<UnitScheme> {
            todo!()
        }

        fn data_root(
            &self,
            col_id: u32,
        ) -> Option<DataStructure> {
            todo!()
        }

        fn free_pids(&self, chunks_length: usize) -> Vec<u32> {
        todo!()
    }
    }
}
