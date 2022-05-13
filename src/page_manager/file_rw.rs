use crate::{
    structures::{
        data_structure::{DataStructure, NodeType},
        root_structure::RootStructure,
        scheme_structure::UnitScheme,
    },
    unit_scheme::unit_scheme::{SetEvent, RecordCell},
};

pub struct FileRW {}

pub trait FileApi {
    fn root_pid(&self) -> u8;
    fn write_root_pid(&self, value: u8) -> Result<(), &str>;
    fn write_root_page(&self, pid: u8, value: &[u8]) -> Result<(), &str>;
    fn write_regular_page(&self, pid: u32, value: &[u8]) -> Result<(), &str>;
    fn root_page_vec(&self, pid: u8) -> Vec<u8>;
    fn regular_page_vec(&self, pid: u32) -> Vec<u8>;
    fn root(&self) -> Option<RootStructure>;
    fn scheme(&self) -> Option<UnitScheme>;
    fn data_root(&self, col_id: u32) -> Option<DataStructure>;
    fn free_pids(&self, chunks_length: usize) -> Vec<u32>;
}

pub trait DataPageFinder {
    fn find_page(file_api: &impl FileApi, cell: &RecordCell) -> Option<(Vec<u8>, u32)>;
}

impl FileRW {
    pub fn write_root<'a>(file_api: &'a impl FileApi, root: &RootStructure) -> Result<(), &'a str> {
        let pointer = file_api.root_pid();
        assert!(pointer < 2);

        let new_pointer = 1 - pointer;
        let root_page_result = file_api.write_root_page(new_pointer, &root.to_vec());

        if root_page_result.is_ok() {
            file_api.write_root_pid(1 - pointer)
        } else {
            root_page_result
        }
    }

    pub fn root<'a>(file_api: &'a impl FileApi) -> RootStructure {
        current_root(file_api)
    }

    pub fn append_event<'a>(file_api: &impl FileApi, event: &SetEvent) -> Result<(), &'a str> {
        let col_id = event.cell.col_id;
        let record_id = event.cell.record_id;

        let pages = find_pages(file_api, event);

        Ok(())
    }
}

fn find_pages(file_api: &impl FileApi, event: &SetEvent) -> Vec<u32> {
    let root = current_root(file_api);

    let scheme_page = UnitScheme::read_from(&file_api.regular_page_vec(root.scheme_pid));
    let col_pid = match scheme_page.col_root_pid(event.cell.col_id) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let page_data = file_api.regular_page_vec(col_pid);
    let from = DataStructure::read_from(&page_data);

    let record_id = event.cell.record_id;
    match from.node_type {
        NodeType::Root => {
            for id in from.root.unwrap().identifiers.iter() { // 3
            }
        }
        NodeType::Intermediate => todo!(),
        NodeType::Leaf => todo!(),
    }

    vec![3]
}

fn current_root(file_api: &impl FileApi) -> RootStructure {
    let pointer = file_api.root_pid();
    assert!(pointer < 2);

    let buffer = file_api.root_page_vec(pointer);
    RootStructure::from_slice(&buffer)
}
