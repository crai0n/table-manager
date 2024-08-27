use actix_web::{HttpResponse, post, Responder, web, web::Data, web::Json};
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

// #[get("/")]
// async fn list_all_tables() -> impl Responder {
//     let response = format!("Displaying your table: {:?}", id);
//     HttpResponse::Ok().body(response)
// }
//
// #[get("/tables")]
// async fn list_all_tables(id: u32) -> impl Responder {
//     let response = format!("Displaying your table: {:?}", id);
//     HttpResponse::Ok().body(response)
// }
//
// #[delete()]
// async fn delete(id: u32) -> impl Responder {
//     let response = format!("Removed your table: {:?}", id);
//     HttpResponse::Ok().body(response)
// }
//

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create)
    );
}


