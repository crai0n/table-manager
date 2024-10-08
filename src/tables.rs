use actix_web::{delete, get, post, put, web, web::Data, web::Json, HttpResponse, Responder};

use models::bridge_table::NewBridgeTable;

use crate::models;
use crate::table_store::TableStore;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 201, description = "Returns the created resource", body = BridgeTable),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )

)]
#[post("/tables")]
async fn create_bridge_table(
    new_bridge_table: Json<NewBridgeTable>,
    db: Data<dyn TableStore + Send + Sync>,
) -> impl Responder {
    let table = db.insert_bridge_table(new_bridge_table.into_inner()).await;
    match table {
        Ok(table) => HttpResponse::Created().json(table),
        Err(_) => HttpResponse::InternalServerError().body("Error in TableStore"),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Lists all tables", body = BridgeTable),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
#[get("/tables")]
pub async fn list_all_bridge_tables(db: web::Data<dyn TableStore + Send + Sync>) -> impl Responder {
    let tables = db.get_bridge_tables().await;
    match tables {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(_) => HttpResponse::InternalServerError().body("Error in TableStore"),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Provides the requested Table", body = BridgeTable),
        (status = 404, description = "Table not found"),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
#[get("/tables/{id}")]
pub async fn get_bridge_table_by_id(
    id: web::Path<u32>,
    db: web::Data<dyn TableStore + Send + Sync>,
) -> HttpResponse {
    let table = db.get_bridge_table_by_id(id.into_inner()).await;
    match table {
        Ok(Some(table)) => HttpResponse::Ok().json(table),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error in TableStore"),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Requested Table has been updated", body = BridgeTable),
        (status = 404, description = "Table not found")
    )
)]
#[put("/tables/{id}")]
pub async fn update_bridge_table_by_id(
    db: web::Data<dyn TableStore + Send + Sync>,
    id: web::Path<u32>,
    updated_bridge_table: web::Json<NewBridgeTable>,
) -> HttpResponse {
    let table = db
        .update_bridge_table_by_id(id.into_inner(), updated_bridge_table.into_inner())
        .await;
    match table {
        Ok(Some(table)) => HttpResponse::Ok().json(table),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error in TableStore"),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Requested Table has been deleted", body = BridgeTable),
        (status = 404, description = "Table not found")
    )
)]
#[delete("/tables/{id}")]
pub async fn delete_bridge_table_by_id(
    db: web::Data<dyn TableStore + Send + Sync>,
    id: web::Path<u32>,
) -> HttpResponse {
    let table = db.delete_bridge_table_by_id(id.into_inner()).await;
    match table {
        Ok(Some(table)) => HttpResponse::Ok().json(table),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error in TableStore"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_bridge_table)
            .service(list_all_bridge_tables)
            .service(get_bridge_table_by_id)
            .service(update_bridge_table_by_id)
            .service(delete_bridge_table_by_id),
    );
}
