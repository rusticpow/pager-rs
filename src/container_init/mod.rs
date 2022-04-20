use crate::{
    page_manager::file_io::FileIO,
    page_structure::generated::container_structure_generated::pager::{
        root_as_container_structure, ContainerStructure,
    },
};

use super::unit_scheme::UnitScheme;

pub struct ContainerInit<'a> {
    name: &'a str,
}

pub struct ContainerInitRepo {}

pub trait ContainerInitRepository {
    fn get_container_structure(&mut self, file_io: &mut impl FileIO) -> Result<Vec<u8>, &str>;
    fn set_container_structure(
        &self,
        scheme: &UnitScheme,
        file_io: &mut impl FileIO
    ) -> Result<(), &str>;
}

impl<'a> ContainerInit<'a> {
    pub fn new(name: &'a str) -> ContainerInit<'a> {
        ContainerInit { name: name }
    }

    pub fn init(
        &'a mut self,
        scheme: &UnitScheme,
        file_io: &'a mut impl FileIO,
        repository: &'a mut impl ContainerInitRepository,
    ) {
        let structure_result = repository.get_container_structure(file_io);
        match structure_result {
            Ok(buf) => {}
            Err(_) => {
                repository.set_container_structure(scheme, file_io);
            }
        }
    }
}
