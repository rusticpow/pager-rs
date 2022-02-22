use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
    path::Path, os::unix::prelude::FileExt,
};

fn main() {
    println!("Hello, world!");

    let file_name = "hello.db";
    let path = Path::new(file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let page_size = 4096;

    let mut buf: &[u8] = &[0, 0, 0, 1, 1];
    file.write_at(buf, 0);
}

struct Pointer {
    right: u32,
}

struct Page {}

impl Page {}
