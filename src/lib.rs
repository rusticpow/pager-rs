use container_init::{ContainerInit, ContainerInitRepository};
use unit_scheme::UnitScheme;
use page_manager::container_init_repository_impl::{self, ContainerInitRepositoryImpl};

mod container_init;
mod unit_scheme;
mod page_manager;
mod page_structure;

struct Api {}

impl Api {
    fn init(name: &str, scheme: UnitScheme) {
        let repository = ContainerInitRepositoryImpl { file_io: todo!() };
        let container = ContainerInit::new(&mut repository, name);
        container.init(&scheme);
        // container.init();
    }
}
