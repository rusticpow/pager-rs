use std::fs::{File, OpenOptions};

const UNIT_EXTENSION: &str = ".unit";
const UNIT_LOG_EXTENSION: &str = ".unit_log";

pub struct UnitFileIOImpl {
    pub file_path: String,
    pub file: File,
    pub log_file: File,
    pub log_file_path: String,
}

impl UnitFileIOImpl {
    pub fn new(unit_name: &str) -> UnitFileIOImpl {
        let file_path = file_path(unit_name);
        let log_file_path = log_file_path(unit_name);

        UnitFileIOImpl {
            file: OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(file_path.as_str())
                .unwrap(),
            file_path: String::from(file_path),
            log_file: OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(log_file_path.as_str())
                .unwrap(),
            log_file_path: String::from(log_file_path),
        }
    }
}

fn file_path(unit_name: &str) -> String {
    let mut unit_file_name: String = unit_name.to_owned();
    unit_file_name.push_str(UNIT_EXTENSION);

    unit_file_name
}

fn log_file_path(unit_name: &str) -> String {
    let mut log_file_name: String = unit_name.to_owned();
    log_file_name.push_str(UNIT_LOG_EXTENSION);

    log_file_name
}
