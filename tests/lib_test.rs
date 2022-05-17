use std::fs::{self};

use ulid::Ulid;

#[test]
fn test_put_read_value() {
    let unit_name = Ulid::new().to_string();



    fs::remove_file(unit_name + ".unit").unwrap();
}
