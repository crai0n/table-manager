use async_trait::async_trait;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use dotenvy::dotenv;

use crate::models::bridge_table::{BridgeTable, NewBridgeTable};
use crate::schema::bridge_tables;
use crate::schema::bridge_tables::dsl::*;
use crate::table_store::{TableStore, TableStoreError};

#[derive(Clone)]
pub struct DbTableStore {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

#[async_trait]
impl TableStore for DbTableStore {
    async fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, TableStoreError> {
        self.insert_bridge_table(new_bridge_table)
            .await
            .map_err(TableStoreError::DieselError)
    }

    async fn get_bridge_tables(&self) -> Result<Vec<BridgeTable>, TableStoreError> {
        self.get_bridge_tables()
            .await
            .map_err(TableStoreError::DieselError)
    }

    async fn get_bridge_table_by_id(
        &self,
        table_id: u32,
    ) -> Result<Option<BridgeTable>, TableStoreError> {
        self.get_bridge_table_by_id(table_id)
            .await
            .map_err(TableStoreError::DieselError)
    }

    async fn update_bridge_table_by_id(
        &self,
        table_id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Result<Option<BridgeTable>, TableStoreError> {
        self.update_bridge_table_by_id(table_id, new_bridge_table)
            .await
            .map_err(TableStoreError::DieselError)
    }

    async fn delete_bridge_table_by_id(
        &self,
        table_id: u32,
    ) -> Result<Option<BridgeTable>, TableStoreError> {
        self.delete_bridge_table_by_id(table_id)
            .await
            .map_err(TableStoreError::DieselError)
    }
}

impl DbTableStore {
    pub async fn new() -> Self {
        let pool = get_connection_pool().await;
        DbTableStore { pool }
    }

    pub async fn insert_bridge_table(
        &self,
        new_bridge_table: NewBridgeTable,
    ) -> Result<BridgeTable, Error> {
        let connection = &mut self.pool.get().unwrap();

        connection.transaction(|conn| {
            diesel::insert_into(bridge_tables::table)
                .values(&new_bridge_table)
                .execute(conn)?;

            let bridge_table = bridge_tables::table
                .order(bridge_tables::id.desc())
                .select(BridgeTable::as_select())
                .first(conn)?;

            Ok(bridge_table)
        })
    }

    pub async fn get_bridge_tables(&self) -> Result<Vec<BridgeTable>, Error> {
        let connection = &mut self.pool.get().unwrap();
        bridge_tables
            .filter(public.eq(true))
            .limit(5)
            .select(BridgeTable::as_select())
            .load::<BridgeTable>(connection)
    }

    pub async fn get_bridge_table_by_id(
        &self,
        table_id: u32,
    ) -> Result<Option<BridgeTable>, Error> {
        let connection = &mut self.pool.get().unwrap();
        bridge_tables
            .find(table_id)
            .select(BridgeTable::as_select())
            .first(connection)
            .optional()
    }

    pub async fn update_bridge_table_by_id(
        &self,
        table_id: u32,
        new_bridge_table: NewBridgeTable,
    ) -> Result<Option<BridgeTable>, Error> {
        let connection = &mut self.pool.get().unwrap();
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
    }

    pub async fn delete_bridge_table_by_id(
        &self,
        table_id: u32,
    ) -> Result<Option<BridgeTable>, Error> {
        let connection = &mut self.pool.get().unwrap();
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
    }
}

async fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();

    let url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
