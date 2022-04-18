use container_init::{ContainerInit, ContainerInitRepository};
use unit_scheme::UnitScheme;
use page_manager::container_init_repository_impl::{self, ContainerInitRepositoryImpl};
use page_manager::file_io::{FileIOImpl};

mod container_init;
mod unit_scheme;
mod page_manager;
mod page_structure;

struct Api {}

impl Api {
    fn init(container_name: &str, scheme: UnitScheme) {
        let mut file_io = FileIOImpl::new(container_name);
        let mut repository = ContainerInitRepositoryImpl { };
        let mut container = ContainerInit::new(container_name);
        container.init(&scheme, &mut file_io, &mut repository);
        // container.init();
    }
}
