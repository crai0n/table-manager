use std::sync::{Arc, Mutex};

use crate::models::bridge_table::{BridgeTable, NewBridgeTable};
use crate::table_store::{TableStore, TableStoreError};

#[derive(Default, Clone)]
pub(crate) struct InMemoryTableStore {
    pub tables: Arc<Mutex<Vec<BridgeTable>>>,
}

impl TableStore for InMemoryTableStore {
    fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, TableStoreError> {
        Ok(self.insert_bridge_table(new_bridge_table))
    }

    fn get_bridge_tables(&self) -> Vec<BridgeTable> {
        self.get_bridge_tables()
    }

    fn get_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable> {
        self.get_bridge_table_by_id(id)
    }

    fn update_bridge_table_by_id(
        &self,
        id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Option<BridgeTable> {
        self.update_bridge_table_by_id(id, new_bridge_table)
    }

    fn delete_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable> {
        self.delete_bridge_table_by_id(id)
    }
}

impl InMemoryTableStore {
    pub fn new() -> Self {
        let tables = Arc::new(Mutex::new(vec![]));
        InMemoryTableStore { tables }
    }

    pub fn insert_bridge_table(&self, new_bridge_table: NewBridgeTable) -> BridgeTable {
        let mut tables = self.tables.lock().unwrap();
        let id = rand::random();
        let table = BridgeTable {
            id,
            name: new_bridge_table.name,
            owner: new_bridge_table.owner,
            public: new_bridge_table.public,
        };
        tables.push(table.clone());
        table
    }

    pub fn get_bridge_tables(&self) -> Vec<BridgeTable> {
        let tables = self.tables.lock().unwrap();
        tables.clone()
    }

    pub fn get_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable> {
        let tables = self.tables.lock().unwrap();
        tables.iter().find(|table| table.id == id).cloned()
    }

    pub fn update_bridge_table_by_id(&self, id: u32, table: NewBridgeTable) -> Option<BridgeTable> {
        let mut tables = self.tables.lock().unwrap();
        let index = tables.iter().position(|table| table.id == id)?;
        let new_bridge_table = BridgeTable {
            id,
            name: table.name,
            owner: table.owner,
            public: table.public,
        };
        tables[index] = new_bridge_table.clone();
        Some(new_bridge_table)
    }

    pub fn delete_bridge_table_by_id(&self, id: u32) -> Option<BridgeTable> {
        let mut tables = self.tables.lock().unwrap();
        let index = tables.iter().position(|table| table.id == id)?;
        Some(tables.remove(index))
    }
}
