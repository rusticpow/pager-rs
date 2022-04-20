pub struct UnitScheme {
    pub tables: Vec<UnitTable>,
}

pub struct UnitTable {
    pub name: String,
    pub columns: Vec<UnitColumn>,
}

pub enum UnitColumnType {
    integer,
    double,
    string
}

pub struct UnitColumn {
    pub id: u32,
    pub name: String,
    pub col_type: UnitColumnType
}