pub mod unit_scheme_apply;

pub struct UnitScheme {
    pub tables: Vec<UnitTable>,
}

pub struct UnitTable {
    pub name: String,
    pub columns: Vec<UnitColumn>,
}

pub enum UnitColumnType {
    Integer,
    Double,
    String
}

pub struct UnitColumn {
    pub id: u32,
    pub name: String,
    pub col_type: UnitColumnType
}

pub struct TableRecord<'a> {
    pub container_name: String,
    pub table_name: String,
    pub id: Option<String>,
    pub columns: Vec<TableRecordColumn<'a>>
}

pub struct TableRecordColumn<'a> {
    pub name: String,
    pub col_type: UnitColumnType,
    pub string_value: Option<&'a str>,
    pub numeric_value: Option<&'a i64>
}