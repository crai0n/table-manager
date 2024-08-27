use actix_web::{get, HttpResponse, post, put, delete, Responder, web, web::Data, web::Json};
use chrono::Utc;

use models::table::Table;

use crate::models;
use crate::table_store::TableStore;

#[utoipa::path(
    context_path = "/api",
    request_body(content = Table, example = json!({
    "name": "table1",
    "owner": "owner1",
    "public": true
    })),
    responses(
        (status = 201, description = "Returns the created resource", body = Table),
        (status = 400, description = "Bad Request", body = String)
    )

)]
#[post("/tables")]
async fn create(table: Json<Table>, db: Data<TableStore>) -> impl Responder {
    let enriched_table = enrich_table(table);
    match db.insert_table(enriched_table) {
        Ok(table) => HttpResponse::Created().json(table),
        Err(_) => HttpResponse::InternalServerError().body("Something strange happened"),
    }
}

fn enrich_table(table: Json<Table>) -> Table {
    let table = table.into_inner();
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now();
    Table {
        id: Some(id),
        created_at: Some(created_at),
        updated_at: Some(created_at),
        ..table
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Lists all tables", body = Table)
    )
)]
#[get("/tables")]
pub async fn list_all_tables(db: web::Data<TableStore>) -> impl Responder {
    let tables = db.get_tables();
    HttpResponse::Ok().json(tables)
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Provides the requested Table", body = Table),
        (status = 404, description = "Table not found")
    )
)]
#[get("/tables/{id}")]
pub async fn get_table_by_id(id: web::Path<String>, db: web::Data<TableStore>) -> HttpResponse {
    let todo = db.get_table_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Requested Table has been updated", body = Table),
        (status = 404, description = "Table not found")
    )
)]
#[put("/tables/{id}")]
pub async fn update_table_by_id(db: web::Data<TableStore>, id: web::Path<String>, updated_table: web::Json<Table>) -> HttpResponse {
    let table = db.update_table_by_id(&id, updated_table.into_inner());
    match table {
        Some(table) => HttpResponse::Ok().json(table),
        None => HttpResponse::NotFound().finish(),
    }
}


#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Requested Table has been deleted", body = Table),
        (status = 404, description = "Table not found")
    )
)]
#[delete("/tables/{id}")]
pub async fn delete_table_by_id(db: web::Data<TableStore>, id: web::Path<String>) -> HttpResponse {
    let table = db.delete_table_by_id(&id);
    match table {
        Some(table) => HttpResponse::Ok().json(table),
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create)
            .service(list_all_tables)
            .service(get_table_by_id)
            .service(update_table_by_id)
            .service(delete_table_by_id)
    );
}


