use ulid::Ulid;

pub struct NextId {}

impl NextId {
    pub fn get_id() -> String {
        Ulid::new().to_string()
    }
}
