use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    responses(
        (status = 200, description = "Successful response", body = String)
    ),
    tag = "table"
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(hello))]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();


    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
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