use iron::prelude::*;
use iron::status;
use router::Router;

mod file_encoding;
mod routes;

fn main() {
    std::thread::spawn(|| {});
    let mut router = Router::new();
    router.post("/", routes::save_file, "turni");

    Iron::new(router).http("127.0.0.1:2195").unwrap();
}
