use actix_web::{App, HttpServer};

mod conversion;
mod file_encoding;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(actix_web::web::FormConfig::default().limit(1_000_000 * 10))
            .service(routes::get_file)
            .service(routes::save_file)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
