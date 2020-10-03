use actix_web::{App, HttpServer};
use lazy_static::lazy_static;

mod conversion;
mod file_encoding;
mod routes;

lazy_static! {
    pub static ref BIND_ADDR: String = {
        use std::env::var;
        format!(
            "{}:{}",
            var("FD_BIND_ADDR").unwrap_or("127.0.0.1".to_owned()),
            var("FD_BIND_PORT").unwrap_or("18080".to_owned())
        )
    };
    pub static ref MAX_POOL_SIZE: usize = {
        use std::env::var;
        var("FD_POOL_SIZE")
            .map(|x| x.parse::<usize>().unwrap_or(10_000_000usize))
            .unwrap_or(10_000_000usize)
    };
    pub static ref MAX_FILE_SIZE: usize = {
        use std::env::var;
        var("FD_MAX_FILE_SIZE")
            .map(|x| x.parse::<usize>().unwrap_or(5_000_000usize))
            .unwrap_or(5_000_000usize)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(actix_web::web::FormConfig::default().limit(1_000_000 * 10))
            .service(routes::get_file)
            .service(routes::save_file)
    })
    .bind(&*BIND_ADDR)?
    .run()
    .await
}
