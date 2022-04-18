use crate::page_structure::generated::container_structure_generated::pager::{
    root_as_container_structure, ContainerStructure,
};

use super::unit_scheme::UnitScheme;

pub struct ContainerInit<'a> {
    repository: &'a mut dyn ContainerInitRepository<'a>,
    name: &'a str,
}

pub struct ContainerInitRepo {}

pub trait ContainerInitRepository<'a> {
    fn get_container_structure(&mut self) -> Result<Vec<u8>, &str>;
    fn set_container_structure(&self, scheme: &UnitScheme) -> Result<(), &str>;
}

impl<'a> ContainerInit<'a> {
    pub fn new(
        repository: &'a mut impl ContainerInitRepository<'a>,
        name: &'a str,
    ) -> ContainerInit<'a> {
        ContainerInit {
            repository: repository,
            name: name,
        }
    }

    pub fn init(&'a mut self, scheme: &UnitScheme) {
        let structure_result = self.repository.get_container_structure();
        match structure_result {
            Ok(buf) => {
            }
            Err(_) => {
                self.repository.set_container_structure(scheme);
            }
        }
    }
}
