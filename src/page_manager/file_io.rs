use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
};

use read_write_at::{ReadAtMut, WriteAt};

const CONTAINER_EXTENSION: &str = ".container";
pub const PAGE_SIZE: usize = 4096;

pub struct FileIO {
    file_path: String,
    file: File,
}

impl<'a> FileIO {
    pub fn new(container_name: &str) -> FileIO {
        let file_path = get_file_path(container_name);

        let result = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(file_path.as_str());

        let file = match result {
            Ok(file) => file,
            Err(_) => panic!(), // TODO: temporary
        };

        FileIO {
            file_path: file_path,
            file: file,
        }
    }

    pub fn read(&mut self, page_i: u64) -> Result<Vec<u8>, ()> {
        // .. need to support reading
        let chunk: &mut [u8] = &mut [0; PAGE_SIZE];
        let result = self.file.read_exact_at(chunk, page_i * PAGE_SIZE as u64);
        // TODO: need to handle bad readings (corrupted file, etc.)

        match result {
            Ok(_) => Ok(chunk.to_vec()),
            Err(_) => Err(()),
        }
    }

    pub fn write(&self, page_content: &[u8], page_i: u64) -> Result<(), ()> {
        let result = self
            .file
            .write_all_at(&page_content, page_i * PAGE_SIZE as u64);
        // TODO: need to handle bad writings (full disk, corrupted writing..)

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

fn get_file_path(container_name: &str) -> String {
    let mut owned_string: String = container_name.to_owned();
    owned_string.push_str(CONTAINER_EXTENSION);

    owned_string
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{FileIO, PAGE_SIZE};

    #[test]
    fn write_read_common() {
        let mut file_io = FileIO::new("testable");

        let page_content: &mut [u8] = &mut [0; PAGE_SIZE];
        page_content[1] = 5;
        page_content[4095] = 10;

        file_io.write(page_content, 0).unwrap();

        let page = file_io.read(0).unwrap();

        assert_eq!(page[1], 5);
        assert_eq!(page[4095], 10);

        fs::remove_file(file_io.file_path).unwrap();
    }
}
