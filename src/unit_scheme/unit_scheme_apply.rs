use crate::page_manager::{file_io::{FileIO, PageType, Structure, StructurePages}, pages_pointer::PagesPointerImpl};

use super::UnitScheme;

pub struct UnitSchemeReadWrite {}

impl UnitSchemeReadWrite {
    pub fn apply(file_io: &mut impl FileIO, scheme: &UnitScheme) {
        let start_page = file_io
            .write(
                &PagesPointerImpl{},
                PageType::Scheme,
                &Structure {
                    content: scheme.to_vec(),
                    pages: StructurePages { value: vec![0] },
                },
            )
            .unwrap()
            .unwrap();

        println!("Start_page is: {}", start_page);
    }

    pub fn read(file_io: &mut impl FileIO) -> UnitScheme {
        let structure = file_io.read(0).unwrap();

        UnitScheme::read_from(&structure.content)
    }
}
