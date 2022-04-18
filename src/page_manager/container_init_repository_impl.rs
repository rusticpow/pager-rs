use crate::{
    container_init::ContainerInitRepository,
    page_structure::generated::header_generated::pager::root_as_header,
};

use super::file_io::FileIO;

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

    fn set_container_structure(&self, scheme: &crate::unit_scheme::UnitScheme) -> Result<(), &str> {
        todo!();
    }
}
