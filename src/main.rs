use actix_web::{App, HttpServer};
use lazy_static::lazy_static;

mod conversion;
mod file_encoding;
mod routes;
mod utils;

lazy_static! {
    pub static ref BIND_ADDR: String = {
        use std::env::var;
        format!(
            "{}:{}",
            var("FD_BIND_ADDR").unwrap_or("127.0.0.1".to_owned()),
            var("FD_BIND_PORT").unwrap_or("18080".to_owned())
        )
    };
    pub static ref MAX_POOL_SIZE: u64 = {
        use std::env::var;
        var("FD_POOL_SIZE")
            .map(|x| x.parse::<u64>().unwrap_or(10_000_000u64))
            .unwrap_or(10_000_000u64)
    };
    pub static ref MAX_FILE_SIZE: u64 = {
        use std::env::var;
        var("FD_MAX_FILE_SIZE")
            .map(|x| x.parse::<u64>().unwrap_or(5_000_000u64))
            .unwrap_or(5_000_000u64)
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::thread::spawn(|| loop {
        if let Ok(mut pool) = dbg!(utils::get_pool()) {
            // Remove excess files
            while utils::get_pool_size(&pool) > *MAX_POOL_SIZE {
                if let Some(to_del) = pool.pop() {
                    std::fs::remove_file(to_del.0).ok();
                }
            }

            while pool.len() >= (file_encoding::MOD as f32 * 0.9) as usize {
                if let Some(to_del) = pool.pop() {
                    std::fs::remove_file(to_del.0).ok();
                }
            }

            let min_allowed_date = (std::time::SystemTime::now()
                - std::time::Duration::from_secs(24 * 3600))
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            loop {
                if let Some(file) = pool.pop() {
                    if file.2 < min_allowed_date {
                        std::fs::remove_file(file.0).ok();
                    } else {
                        break;
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(300));
    });
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
