use crate::Page;
use bincode::{options};
pub use bincode::config::Options;

struct Api<'a> {
    manager: &'a dyn PageManager,
}

struct Column {
    name: String,
    col_type: ColumnType,
}

#[derive(serde::ser::Serialize, Debug)]
struct TableCreateCommand {
    name: String,
    columns: Vec<Column>,
    id: u16,
}

enum ColumnType {
    u32,
}

trait PageManager {
    fn find_scheme_page(&self) -> Option<(Page, u32)>;
    fn insert_new_scheme_page(&self, page: Page);
    fn update_scheme_page(&self, page: (Page, u32));
}

impl Api<'_> {
    fn create_table(&self, command: TableCreateCommand) {
        // 1. find page, that can be used to specify table and columns
        let page = self.manager.find_scheme_page();
        if page.is_some() {
            //
        }

        // 2. insert need data

        // 3. rewrite page back
    }
}

// fn get_command_size(command: &TableCreateCommand) -> usize {}

// fn serialize_table_create_command(command: &TableCreateCommand) -> Vec<u8> {

// }

// [length(u8),table_name_in_ascii(vary),table_id[u16]]
fn serialize_table_name(command: &TableCreateCommand) -> Vec<u8> {
    if !command.name.is_ascii() {
        panic!("ascii chars available only");
    }

    let config = options();
    
    let ser = bincode::serialize(command).unwrap();
    
    // let config = bincode::config::Options
    //  // pick one of:
    //  .with_little_endian()
    //  // pick one of:
    //  .with_variable_int_encoding()
    //  // pick one of:
    //  .skip_fixed_array_length()
    //  .write_fixed_array_length();

    let table_name_in_bytes = command.name.as_bytes();

    let length = 1 + table_name_in_bytes.len() + 2;
    let mut buf: Vec<u8> = vec![0; length];

    let mut offset = 0;
    buf[offset] = table_name_in_bytes.len() as u8;

    for i in 0..length {
        offset += 1;
        buf[offset] = table_name_in_bytes[i];
    }

    let id_in_bytes = command.id.to_le_bytes();
    offset += 1;
    buf[offset] = id_in_bytes[0];
    offset += 1;
    buf[offset] = id_in_bytes[1];

    buf
}
