use crate::{page_manager::{file_io::{FileIO, PageType, Structure, StructurePages}, pages_pointer::PagesPointerImpl}, structures::scheme_structure::{UnitColumnType, UnitScheme}};

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


pub struct TableRecord<'a> {
    pub container_name: String,
    pub table_name: String,
    pub id: Option<String>,
    pub columns: Vec<TableRecordColumn<'a>>,
}

pub struct TableRecordColumn<'a> {
    pub name: String,
    pub col_type: UnitColumnType,
    pub string_value: Option<&'a str>,
    pub numeric_value: Option<&'a i64>,
}