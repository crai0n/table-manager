use actix_web::{delete, get, HttpResponse, post, put, Responder, web, web::Data, web::Json};

use models::table::NewTable;

use crate::models;
use crate::table_store::TableStore;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 201, description = "Returns the created resource", body = Table),
        (status = 400, description = "Bad Request", body = String)
    )

)]
#[post("/tables")]
async fn create_table(new_table: Json<NewTable>, db: Data<dyn TableStore>) -> impl Responder {
    match db.insert_table(new_table.into_inner()) {
        Ok(table) => HttpResponse::Created().json(table),
        Err(_) => HttpResponse::InternalServerError().body("Something strange happened"),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Lists all tables", body = Table)
    )
)]
#[get("/tables")]
pub async fn list_all_tables(db: web::Data<dyn TableStore>) -> impl Responder {
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
pub async fn get_table_by_id(id: web::Path<u32>, db: web::Data<dyn TableStore>) -> HttpResponse {
    let todo = db.get_table_by_id(id.into_inner());
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
pub async fn update_table_by_id(
    db: web::Data<dyn TableStore>,
    id: web::Path<u32>,
    updated_table: web::Json<NewTable>,
) -> HttpResponse {
    let table = db.update_table_by_id(id.into_inner(), updated_table.into_inner());
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
pub async fn delete_table_by_id(db: web::Data<dyn TableStore>, id: web::Path<u32>) -> HttpResponse {
    let table = db.delete_table_by_id(id.into_inner());
    match table {
        Some(table) => HttpResponse::Ok().json(table),
        None => HttpResponse::NotFound().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_table)
            .service(list_all_tables)
            .service(get_table_by_id)
            .service(update_table_by_id)
            .service(delete_table_by_id),
    );
}
