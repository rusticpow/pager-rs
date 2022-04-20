use pager_rs::{
    unit_scheme::{UnitColumn, UnitColumnType, UnitScheme, UnitTable},
    Api,
};

#[test]
fn test_schema_init() {
    Api::init(
        "test_container",
        UnitScheme {
            tables: vec![UnitTable {
                name: String::from("tasks"),
                columns: vec![UnitColumn {
                    id: 7,
                    name: String::from("So cool string"),
                    col_type: UnitColumnType::string,
                }],
            }],
        },
    )
}
