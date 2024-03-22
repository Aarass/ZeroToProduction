use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
