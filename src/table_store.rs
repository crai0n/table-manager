use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::models::table::Table;

#[derive(Default)]
pub(crate) struct TableStore {
    pub tables: Arc<Mutex<Vec<Table>>>,
}

impl TableStore {
    pub fn new() -> Self {
        let tables = Arc::new(Mutex::new(vec![]));
        TableStore { tables }
    }

    pub fn insert_table(&self, table: Table) -> Result<Table, Error> {
        let mut tables = self.tables.lock().unwrap();
        tables.push(table.clone());
        Ok(table)
    }
}

