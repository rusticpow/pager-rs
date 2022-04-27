#[macro_use]
extern crate lazy_static;

use unit_scheme::{UnitScheme, unit_scheme_apply::UnitSchemeApply};
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
        UnitSchemeApply::apply(&mut file_io, scheme);
    }
}
