use std::fmt::Error;

use crate::models::table::{NewTable, Table};

pub mod in_memory_table_store;

pub trait TableStore {
    fn insert_table(&self, new_table: NewTable) -> Result<Table, Error>;
    fn get_tables(&self) -> Vec<Table>;
    fn get_table_by_id(&self, id: u32) -> Option<Table>;
    fn update_table_by_id(&self, id: u32, new_table: NewTable) -> Option<Table>;
    fn delete_table_by_id(&self, id: u32) -> Option<Table>;
}
