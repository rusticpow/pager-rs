use crate::{
    container_init::ContainerInitRepository,
    page_structure::generated::{
        container_structure_generated::pager::{ContainerStructure, ContainerStructureArgs},
        header_generated::pager::root_as_header,
    },
};

use super::file_io::{FileIO, BODY_SIZE};

pub struct ContainerInitRepositoryImpl {}

impl<'a> ContainerInitRepository for ContainerInitRepositoryImpl {
    fn get_container_structure(&mut self, file_io: &mut impl FileIO) -> Result<Vec<u8>, &str> {
        let result = file_io.read(0);
        if result.is_err() {
            return Err("page is incorrect");
        }
        let page_buf = result.unwrap();

        let header_size = page_buf[0];
        let header_slice = &page_buf[1..(header_size as usize)];
        let header_result = root_as_header(header_slice);
        if header_result.is_err() {
            return Err("header is incorrect");
        }

        let header = header_result.unwrap();
        let body_slice = &page_buf[(header_size + 1) as usize..(header.body_size() as usize)];

        Ok(body_slice.to_vec())
    }

    fn set_container_structure(
        &self,
        scheme: &crate::unit_scheme::UnitScheme,
        file_io: &mut impl FileIO,
    ) -> Result<(), &str> {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(BODY_SIZE);
        let structure = ContainerStructure::create(
            &mut builder,
            &ContainerStructureArgs {
                scheme_from_page: 1,
                unit_from_page: 2,
            },
        );

        builder.finish(structure, None);
        let data = builder.finished_data();
        match file_io.write(data, 0) {
            Ok(_) => Ok(()),
            Err(_) => Err("writing is failed"),
        }
    }
}
