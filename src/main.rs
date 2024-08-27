use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use env_logger::{Env, Target};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod table_store;
mod tables;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default())
        .target(Target::Stdout)
        .init();

    let table_db = table_store::TableStore::new();
    let app_data = web::Data::new(table_db);

    #[derive(OpenApi)]
    #[openapi(
        paths(
            healthcheck,
            tables::create,
            tables::list_all_tables,
            tables::get_table_by_id,
            tables::update_table_by_id,
            tables::delete_table_by_id,
        ),
        components(schemas(models::table::Table, models::table::Table)),
        tags((name = "tables", description = "Table management endpoints"))
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_data.clone())
            .configure(tables::config)
            .service(healthcheck)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(Scalar::with_url("/scalar", openapi.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[utoipa::path(
    tag = "healthcheck",
    responses(
        (status = 200, description = "Everything is fine!")
    )
)]
#[get("/hc")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}
