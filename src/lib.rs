extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use structures::scheme_structure::UnitScheme;

mod page_manager;
pub mod unit_scheme;
pub mod structures;

pub struct Api {
}

impl Api {
    pub fn init(unit_name: &str, scheme: &UnitScheme) {
        todo!()
    }

    pub fn get_scheme(unit_name: &str) -> UnitScheme {
       todo!()
    }
}
