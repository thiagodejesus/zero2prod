use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

static ADDRESS: &str = "127.0.0.1:8080";
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(ADDRESS)? // Needs to pay attention on this syntax
        .run();

    Ok(server)
}
