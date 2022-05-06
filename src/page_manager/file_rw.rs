use crate::structures::root_structure::RootStructure;

pub struct FileRW {}

pub trait FileApi {
    fn root_pid(&self) -> u8;
    fn write_root_pid(&self, value: u8) -> Result<(), &str>;
    fn write_root_page(&self, pid:u8, value: &[u8]) -> Result<(), &str>;
    fn root_page(&self, pid:u8) -> Vec<u8>;
}

impl FileRW {
    pub fn write_root<'a>(file_api: &'a impl FileApi, root: &RootStructure) -> Result<(), &'a str> {
        let pointer = file_api.root_pid();
        assert!(pointer < 2); 

        let new_pointer = 1 - pointer;
        file_api.write_root_page(new_pointer, &root.to_vec())
    }

    pub fn root<'a>(file_api: &'a impl FileApi) -> RootStructure {
        let pointer = file_api.root_pid();
        assert!(pointer < 2); 

        let buffer = file_api.root_page(pointer);
        RootStructure::from_slice(&buffer)
    }
 
    pub fn switch_root(file_api: &impl FileApi) {
        let pointer = file_api.root_pid();
        assert!(pointer < 2);

        file_api.write_root_pid(1 - pointer).unwrap();
    }

    pub fn append_event(file_api: &impl FileApi) -> Result<(), &str> {
        


        Ok(())
    }
}


