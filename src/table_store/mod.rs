use async_trait::async_trait;
use diesel::result::Error as DieselError;

use crate::models::bridge_table::{BridgeTable, NewBridgeTable};

pub mod db_table_store;
#[allow(dead_code)]
pub mod in_memory_table_store;

#[async_trait]
pub trait TableStore {
    async fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, TableStoreError>;
    async fn get_bridge_tables(&self) -> Result<Vec<BridgeTable>, TableStoreError>;
    async fn get_bridge_table_by_id(&self, id: u32)
        -> Result<Option<BridgeTable>, TableStoreError>;
    async fn update_bridge_table_by_id(
        &self,
        id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Result<Option<BridgeTable>, TableStoreError>;
    async fn delete_bridge_table_by_id(
        &self,
        id: u32,
    ) -> Result<Option<BridgeTable>, TableStoreError>;
}

#[allow(dead_code)]
#[non_exhaustive]
pub enum TableStoreError {
    DieselError(DieselError),
}
