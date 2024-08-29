use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema::*;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, ToSchema, Clone)]
#[diesel(table_name = bridge_tables)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct BridgeTable {
    pub id: u32,
    pub name: String,
    pub owner: String,
    pub public: bool,
}

#[derive(Insertable, Debug, Serialize, Deserialize, ToSchema, Clone)]
#[diesel(table_name = bridge_tables)]
#[schema(example = json!({
    "name": "table1",
    "owner": "owner1",
    "public": true
    }))]
pub struct NewBridgeTable {
    pub name: String,
    pub owner: String,
    pub public: bool,
}
