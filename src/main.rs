use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(healthcheck)
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .service(healthcheck)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
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


