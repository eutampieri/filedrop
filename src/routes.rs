use iron::prelude::*;
use iron::status;

pub fn get_file(r: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "session.as_str()")))
}
pub fn save_file(r: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "session.as_str()")))
}
