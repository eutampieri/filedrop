use image::{GenericImageView, ImageDecoder};
use iron::prelude::*;
use iron::status;
use router::Router;

fn decode_image(img: &str) -> Result<Vec<u8>, &'static str> {
    let image = base64::decode(img).map_err(|_| "Cannot decode image from b64")?;
    if image.len() > 5_000_000 {
        return Err("File is too big");
    }
    let mime = infer::Infer::new()
        .get(&image)
        .ok_or("Cannot detect image type")?;
    if mime.mime.split("/").nth(0).unwrap() != "image" {
        return Err("The file is not an image");
    }
    let decoded = image::load_from_memory(&image).map_err(|_| "Cannot decode the image")?;
    let mut output = Vec::<u8>::new();
    let farbfeld_encoder = image::farbfeld::FarbfeldEncoder::new(&mut output);
    let dim = decoded.dimensions();
    farbfeld_encoder
        .encode(decoded.as_rgba8().unwrap(), dim.0, dim.1)
        .map_err(|_| "Cannot encode in farbfeld")?;
    Ok(output)
}

fn encode_img(farbfeld: &[u8], ext: &str) -> Result<Vec<u8>, &'static str> {
    let decoder =
        image::farbfeld::FarbfeldDecoder::new(farbfeld).map_err(|_| "Cannot decode farbfeld")?;
    Ok(vec![])
}

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
