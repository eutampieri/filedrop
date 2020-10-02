use image::{GenericImageView, ImageDecoder};
use std::io::Cursor;

pub enum Format {
    Jpeg,
    Webp,
    Gif,
    Bmp,
    Tiff,
    Png,
    Ico,
}

pub fn decode_image(img: &str) -> Result<Vec<u8>, &'static str> {
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

pub fn farbfeld_to_webp(farbfeld: &[u8]) -> Result<Vec<u8>, &'static str> {
    let decoder =
        image::farbfeld::FarbfeldDecoder::new(farbfeld).map_err(|_| "Cannot decode farbfeld")?;
    let img_color_type = decoder.color_type();
    let dim = decoder.dimensions();
    let mut img_data = Vec::<u8>::new();
    decoder
        .read_image(&mut img_data)
        .map_err(|_| "Could not read farbfeld")?;
    libwebp::WebPEncodeLosslessRGBA(&img_data, dim.0, dim.1, 8)
        .map_err(|_| "Cannot encode to WebP")
        .map(|x| Vec::from(&*x)) // TODO Fix this alloc
}

pub fn encode_img(webp: &[u8], format: Format) -> Result<Vec<u8>, &'static str> {
    let (width, height, img_data) =
        libwebp::WebPDecodeRGBA(webp).map_err(|_| "Cannot decode WebP")?;
    let img_color_type = image::ColorType::Rgba8;
    let dim = (width, height);
    match format {
        Format::Bmp => {
            let mut output = Vec::<u8>::new();
            let mut encoder = image::bmp::BmpEncoder::new(&mut output);
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            Ok(output)
        }
        Format::Gif => {
            let mut output = Vec::<u8>::new();
            let mut encoder = image::gif::GifEncoder::new(&mut output);
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            drop(encoder);
            Ok(output)
        }
        Format::Ico => {
            let mut output = Vec::<u8>::new();
            let encoder = image::ico::IcoEncoder::new(&mut output);
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            Ok(output)
        }
        Format::Jpeg => {
            let mut output = Vec::<u8>::new();
            let mut encoder = image::jpeg::JpegEncoder::new(&mut output);
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            Ok(output)
        }
        Format::Png => {
            let mut output = Vec::<u8>::new();
            let encoder = image::png::PngEncoder::new(&mut output);
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            Ok(output)
        }
        Format::Tiff => {
            let mut output = Vec::<u8>::new();
            let encoder = image::tiff::TiffEncoder::new(Cursor::new(&mut output));
            encoder
                .encode(&img_data, dim.0, dim.1, img_color_type)
                .map_err(|_| "Cannot encode image")?;
            Ok(output)
        }
        Format::Webp => Ok(Vec::from(webp)),
    }
}
