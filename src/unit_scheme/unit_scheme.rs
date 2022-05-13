use crate::structures::scheme_structure::UnitColumnType;

pub struct TableRecord<'a> {
    pub container_name: String,
    pub table_name: String,
    pub id: Option<String>,
    pub columns: Vec<TableRecordColumn<'a>>,
}

pub struct TableRecordColumn<'a> {
    pub name: String,
    pub col_type: UnitColumnType,
    pub string_value: Option<&'a str>,
    pub numeric_value: Option<&'a i64>,
}

pub struct SetEvent<'a> {
    pub cell: RecordCell,
    pub atomicity_id: u64,
    pub string_value: Option<&'a str>,
    pub numeric_value: Option<&'a i64>,
}

pub struct RecordCell {
    pub record_id: u128,
    pub col_id: u32,
}
