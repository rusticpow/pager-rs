use crate::unit_scheme::TableRecord;

use super::next_id::NextId;

pub struct TableWriter {}

impl TableWriter {
    pub fn set_record<'a>(record: &'a TableRecord) -> Result<&'a str, &'a str> {
        let id = match &record.id {
            Some(v) => v.to_owned(),
            None => NextId::get_id(),
        };

        Ok("")
    }
}
