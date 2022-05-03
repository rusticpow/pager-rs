use std::fs::{File, self};

use pager_rs::{
    unit_scheme::{UnitColumn, UnitColumnType, UnitScheme, UnitTable},
    Api,
};
use ulid::Ulid;

#[test]
fn test_schema_init() {
    let unit_name = Ulid::new().to_string();

    Api::init(
        unit_name.as_str(),
        &UnitScheme {
            tables: vec![UnitTable {
                name: String::from("tasks"),
                columns: vec![UnitColumn {
                    id: 7,
                    name: String::from("So cool string"),
                    col_type: UnitColumnType::String,
                }],
            }],
        },
    );

    let scheme = Api::get_scheme(unit_name.as_str());

    assert_eq!("tasks", scheme.tables[0].name.as_str());

    fs::remove_file(unit_name + ".unit").unwrap();
}
