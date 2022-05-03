#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use unit_scheme::{UnitScheme, unit_scheme_apply::UnitSchemeReadWrite};
use page_manager::file_io::{FileIOImpl};

mod page_manager;
mod page_structure;
pub mod unit_scheme;
mod table_manager;

pub struct Api {
}

impl Api {
    pub fn init(unit_name: &str, scheme: &UnitScheme) {
        let mut file_io = FileIOImpl::new(unit_name);
        UnitSchemeReadWrite::apply(&mut file_io, scheme);
    }

    pub fn get_scheme(unit_name: &str) -> UnitScheme {
        let mut file_io = FileIOImpl::new(unit_name);
        UnitSchemeReadWrite::read(&mut file_io)
    }
}
