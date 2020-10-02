use iron::prelude::*;
use router::Router;

mod convertion;
mod file_encoding;
mod routes;

fn main() {
    std::thread::spawn(|| {});
    let mut router = Router::new();
    router.post("/", routes::save_file, "upload");
    router.get("/:fn", routes::get_file, "get");

    Iron::new(router).http("127.0.0.1:2195").unwrap();
}
