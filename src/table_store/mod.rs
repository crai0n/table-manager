use diesel::result::Error as DieselError;

use crate::models::bridge_table::{BridgeTable, NewBridgeTable};

mod db_table_store;
pub mod in_memory_table_store;

pub trait TableStore {
    fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, TableStoreError>;
    fn get_bridge_tables(&self) -> Vec<BridgeTable>;
    fn get_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable>;
    fn update_bridge_table_by_id(
        &self,
        id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Option<BridgeTable>;
    fn delete_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable>;
}

#[non_exhaustive]
pub enum TableStoreError {
    DieselError(DieselError),
}
