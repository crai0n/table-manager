use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Table {
    pub id: Option<String>,
    #[schema(value_type = Option < String >)]
    pub created_at: Option<DateTime<Utc>>,
    #[schema(value_type = Option < String >)]
    pub updated_at: Option<DateTime<Utc>>,
    pub name: String,
    pub owner: Option<String>,
    pub public: bool,

}

