pub struct UnitScheme {
    tables: Vec<UnitTable>,
}

pub struct UnitTable {
    name: String,
    columns: Vec<UnitColumn>,
}

pub enum UnitColumnType {
    integer,
    double,
    string
}

pub struct UnitColumn {
    id: u32,
    name: String,
    col_type: UnitColumnType
}