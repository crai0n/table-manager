use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::models::table::{NewTable, Table};
use crate::table_store::TableStore;

#[derive(Default, Clone)]
pub(crate) struct InMemoryTableStore {
    pub tables: Arc<Mutex<Vec<Table>>>,
}

impl TableStore for InMemoryTableStore {
    fn insert_table(&self, new_table: NewTable) -> Result<Table, Error> {
        self.insert_table(new_table)
    }

    fn get_tables(&self) -> Vec<Table> {
        self.get_tables()
    }

    fn get_table_by_id(&self, id: u32) -> Option<Table> {
        self.get_table_by_id(id)
    }

    fn update_table_by_id(&self, id: u32, new_table: NewTable) -> Option<Table> {
        self.update_table_by_id(id, new_table)
    }

    fn delete_table_by_id(&self, id: u32) -> Option<Table> {
        self.delete_table_by_id(id)
    }
}

impl InMemoryTableStore {
    pub fn new() -> Self {
        let tables = Arc::new(Mutex::new(vec![]));
        InMemoryTableStore { tables }
    }

    pub fn insert_table(&self, new_table: NewTable) -> Result<Table, Error> {
        let mut tables = self.tables.lock().unwrap();
        let id = rand::random();
        let table = Table {
            id,
            name: new_table.name,
            owner: new_table.owner,
            public: new_table.public,
        };
        tables.push(table.clone());
        Ok(table)
    }

    pub fn get_tables(&self) -> Vec<Table> {
        let tables = self.tables.lock().unwrap();
        tables.clone()
    }

    pub fn get_table_by_id(&self, id: u32) -> Option<Table> {
        let tables = self.tables.lock().unwrap();
        tables.iter().find(|table| table.id == id).cloned()
    }

    pub fn update_table_by_id(&self, id: u32, table: NewTable) -> Option<Table> {
        let mut tables = self.tables.lock().unwrap();
        let index = tables.iter().position(|table| table.id == id)?;
        let new_table = Table {
            id,
            name: table.name,
            owner: table.owner,
            public: table.public,
        };
        tables[index] = new_table.clone();
        Some(new_table)
    }

    pub fn delete_table_by_id(&self, id: u32) -> Option<Table> {
        let mut tables = self.tables.lock().unwrap();
        let index = tables.iter().position(|table| table.id == id)?;
        Some(tables.remove(index))
    }
}
