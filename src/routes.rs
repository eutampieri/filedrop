use iron::prelude::*;
use iron::status;
use router::Router;

pub fn get_file(r: &mut Request) -> IronResult<Response> {
    if let Some(x) = r.extensions.get::<Router>().unwrap().find("fn") {
        Ok(Response::with((
            //iron::modifiers::Header(),
            status::Ok,
            "Requested file wasn't found!",
        )))
    } else {
        Ok(Response::with((
            status::NotFound,
            "Requested file wasn't found!",
        )))
    }
}
pub fn save_file(r: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "session.as_str()")))
}
