use image::GenericImageView;
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

impl From<String> for Format {
    fn from(s: String) -> Self {
        match s.as_str() {
            "jpeg" => Self::Jpeg,
            "jpg" => Self::Jpeg,
            "webp" => Self::Webp,
            "gif" => Self::Gif,
            "bmp" => Self::Bmp,
            "tif" => Self::Tiff,
            "tiff" => Self::Tiff,
            "png" => Self::Png,
            "ico" => Self::Ico,
            _ => Self::Jpeg,
        }
    }
}

impl Format {
    pub fn get_content_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bitmap",
            Self::Tiff => "image/tiff",
            Self::Png => "image/png",
            Self::Ico => "image/vnd.microsoft.icon",
        }
    }
}

pub fn decode_image(img: &str) -> Result<Vec<u8>, &'static str> {
    let image = base64::decode(img).map_err(|_| "Cannot decode image from b64")?;
    if image.len() > 5_000_000 {
        return Err("File is too big");
    }
    let mime = infer::Infer::new()
        .get(&image)
        .ok_or("Cannot detect image type")?;
    let lossy;
    if mime.mime.split("/").nth(0).unwrap() != "image" {
        return Err("The file is not an image");
    } else if mime.mime.split("/").nth(0).unwrap() == "jpeg" {
        lossy = true
    } else {
        lossy = false;
    }
    let decoded = image::load_from_memory(&image).map_err(|_| "Cannot decode the image")?;
    let dim = decoded.dimensions();
    if lossy {
        libwebp::WebPEncodeRGBA(&decoded.into_rgba(), dim.0, dim.1, 4 * dim.0, 90)
            .map_err(|_| "Cannot encode to WebP")
            .map(|x| Vec::from(&*x))
    } else {
        libwebp::WebPEncodeLosslessRGBA(&decoded.into_rgba(), dim.0, dim.1, 4 * dim.0)
            .map_err(|_| "Cannot encode to WebP")
            .map(|x| Vec::from(&*x)) // TODO Fix this alloc
    }
}

pub fn encode_img(webp: &[u8], format: &Format) -> Result<Vec<u8>, &'static str> {
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
