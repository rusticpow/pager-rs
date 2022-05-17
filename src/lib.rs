#![allow(clippy::missing_safety_doc, clippy::not_unsafe_ptr_arg_deref)]

extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate libc;

use lazy_static::lazy_static;
use std::{
    array,
    fs::{remove_file, File},
    future::Future,
    io::Read,
    ops::Index,
};

use allo_isolate::Isolate;
use page_manager::unit_file_io::UnitFileIOImpl;
use structures::{
    event_log::EventLog,
    data_struct::{ColumnStruct, DataStruct},
    scheme_structure::{SetValue},
};
use tokio::{
    self, io,
    runtime::{Builder, Runtime},
    spawn,
    time::sleep,
};

use std::time::Duration;

use std::ffi::CStr;

use std::os::raw::c_char;

mod page_manager;
pub mod structures;

lazy_static! {
    static ref RUNTIME: io::Result<Runtime> = Builder::new_multi_thread()
        // .threaded_scheduler()
        .enable_all()
        // .core_threads(4)
        .thread_name("flutterust")
        .build();
}

// extern fn callback(a: i32) {
//     println!("I'm called from C with value {0}", a);
// }

// #[link(name = "extlib")]
// extern {
//    fn register_callback(cb: extern fn(i32)) -> i32;
//    fn trigger_callback();
// }

async fn sum_val(a: u32, b: i64) -> i64 {
    sleep(Duration::from_millis(3000)).await;

    return a as i64 + 10;
}

#[no_mangle]
pub extern "C" fn put_value(
    // un_pointer: *const c_char,
    col_id: u32,
    // record_id_pointer: *const u8,
    integer_value: i64,
) -> i64 {
    // cb((col_id as i64 + integer_value as i64) as i32)

    RUNTIME.as_ref().unwrap().spawn(async move {
        Isolate::new(integer_value)
            .task(sum_val(col_id, integer_value))
            .await
    });

    // let mut rt = RUNTIME.unwrap();
    // rt.block_on(async {
    //     spawn()
    // });

    //   посмотреть что произойдет на стороне dart

    // spawn(async move  {
    // });

    return 0;

    Api::put_value(
        "test_unit", //unit_name.to_str().unwrap(),
        &SetValue {
            col_id,
            record_id: 4, //record_id,
            integer_value,
        },
    );
}

pub struct Api {}

impl Api {
    pub fn put_value(unit_name: &str, value: &SetValue) {
        let mut unit_file_io = UnitFileIOImpl::new(unit_name);

        let mut data_struct = DataStruct::read_from_file(&mut unit_file_io.file);
        mutate_data_struct(&mut data_struct, value);

        let mut log_struct = EventLog::read_from_file(&mut unit_file_io.log_file);

        data_struct.write_to_file(&mut unit_file_io.file);
    }

    pub fn delete(unit_name: &str) {
        let mut unit_file_io = UnitFileIOImpl::new(unit_name);

        remove_file(unit_file_io.file_path).unwrap();
        remove_file(unit_file_io.log_file_path).unwrap();
    }

    pub fn read_value(unit_name: &str, col_id: u32, record_id: u128) {
        let mut unit_file_io = UnitFileIOImpl::new(unit_name);
        let mut data_struct = DataStruct::read_from_file(&mut unit_file_io.file);
    }
}

fn mutate_data_struct(data_struct: &mut DataStruct, value: &SetValue) {
    if !data_struct.col_ids.contains(&value.col_id) {
        data_struct.col_ids.push(value.col_id);
        data_struct.columns.push(ColumnStruct {
            record_ids: vec![],
            integer_values: vec![],
        });
    }

    let column_i = data_struct
        .col_ids
        .iter()
        .position(|&col_id| col_id == value.col_id)
        .unwrap();
    let column = &mut data_struct.columns[column_i];

    if column.record_ids.contains(&value.record_id) {
        let record_i = column
            .record_ids
            .iter()
            .position(|&record_id| record_id == value.record_id)
            .unwrap();
        column.integer_values[record_i] = value.integer_value;
    } else {
        column.record_ids.push(value.record_id);
        column.integer_values.push(value.integer_value);
    }
}
