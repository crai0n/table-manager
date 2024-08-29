use std::env;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;

use crate::models::bridge_table::{BridgeTable, NewBridgeTable};
use crate::schema::bridge_tables;
use crate::schema::bridge_tables::dsl::*;
use crate::table_store::{TableStore, TableStoreError};

#[derive(Clone)]
pub struct DbTableStore {
    connection: Arc<Mutex<MysqlConnection>>,
}

impl TableStore for DbTableStore {
    fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, TableStoreError> {
        self.insert_bridge_table(new_bridge_table)
            .map_err(|err| TableStoreError::DieselError(err))
    }

    fn get_bridge_tables(&self) -> Vec<BridgeTable> {
        self.get_bridge_tables()
    }

    fn get_bridge_table_by_id(&self, table_id: u32) -> Option<BridgeTable> {
        self.get_bridge_table_by_id(table_id)
    }

    fn update_bridge_table_by_id(
        &self,
        table_id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Option<BridgeTable> {
        self.update_bridge_table_by_id(table_id, new_bridge_table)
    }

    fn delete_bridge_table_by_id(&self, table_id: u32) -> Option<BridgeTable> {
        self.delete_bridge_table_by_id(table_id)
    }
}

impl DbTableStore {
    pub fn new() -> Self {
        let connection = Arc::new(Mutex::new(establish_connection()));
        DbTableStore { connection }
    }

    pub fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, Error> {
        let mut connection = self.connection.lock().unwrap();
        connection.transaction(|conn| {
            diesel::insert_into(bridge_tables::table)
                .values(&new_bridge_table)
                .execute(conn)?;

            bridge_tables::table
                .order(bridge_tables::id.desc())
                .select(BridgeTable::as_select())
                .first(conn)
        })
    }

    pub fn get_bridge_tables(&self) -> Vec<BridgeTable> {
        let mut connection = self.connection.lock().unwrap();
        bridge_tables
            .filter(public.eq(true))
            .limit(5)
            .select(BridgeTable::as_select())
            .load::<BridgeTable>(connection.deref_mut())
            .expect("Error loading tables")
    }

    pub fn get_bridge_table_by_id(&self, table_id: u32) -> Option<BridgeTable> {
        let mut connection = self.connection.lock().unwrap();
        bridge_tables
            .find(table_id)
            .select(BridgeTable::as_select())
            .first(connection.deref_mut())
            .optional()
            .expect("Error loading table")
    }

    pub fn update_bridge_table_by_id(
        &self,
        table_id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Option<BridgeTable> {
        let mut connection = self.connection.lock().unwrap();
        connection
            .transaction(|connection| {
                diesel::update(bridge_tables.find(table_id))
                    .set(new_bridge_table)
                    .execute(connection)?;

                let bridge_table = bridge_tables
                    .find(table_id)
                    .select(BridgeTable::as_select())
                    .first(connection)?;

                Ok(bridge_table)
            })
            .optional()
            .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post {}", table_id))
    }

    pub fn delete_bridge_table_by_id(&self, table_id: u32) -> Option<BridgeTable> {
        let mut connection = self.connection.lock().unwrap();
        connection
            .transaction(|connection| {
                let bridge_table = bridge_tables
                    .find(table_id)
                    .select(BridgeTable::as_select())
                    .first(connection)?;

                diesel::delete(bridge_tables.filter(id.eq(table_id))).execute(connection)?;

                Ok(bridge_table)
            })
            .optional()
            .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post {}", table_id))
    }
}

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
