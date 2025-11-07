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
    Heic,
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
            "heic" => Self::Heic,
            "heif" => Self::Heic,
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
            Self::Heic => "image/heic",
        }
    }
}

pub fn decode_image(img: &str) -> Result<Vec<u8>, &'static str> {
    let image = base64::decode(img).map_err(|_| "Cannot decode image from b64")?;
    if image.len() as u64 > *super::MAX_FILE_SIZE {
        return Err("File is too big");
    }
    let mime = infer::get(&image)
        .map(|x| x.mime_type())
        .ok_or("Cannot detect image type")?;
    let lossy =  if mime.split("/").nth(0).unwrap() != "image" {
        Err("The file is not an image")
    } else if mime.split("/").nth(0).unwrap() == "jpeg" {
        Ok(true)
    } else {
        Ok(false)
    }?;
    let decoded = image::load_from_memory(&image).map_err(|_| "Cannot decode the image")?;
    let dim = decoded.dimensions();
    if lossy {
        libwebp::WebPEncodeRGBA(&decoded.into_rgba8(), dim.0, dim.1, 4 * dim.0, 90f32)
            .map_err(|_| "Cannot encode to WebP")
            .map(|x| Vec::from(&*x))
    } else {
        libwebp::WebPEncodeLosslessRGBA(&decoded.into_rgba8(), dim.0, dim.1, 4 * dim.0)
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
            let mut original_image =
                image::RgbaImage::from_raw(width, height, img_data.into_iter().cloned().collect())
                    .ok_or("Cannot parse image")?;

            let (nw, nh);
            if width > 256 || height > 256 {
                if width > height {
                    nw = 256;
                    nh = 256 * height / width;
                } else {
                    nh = 256;
                    nw = 256 * width / height;
                }
                original_image = image::imageops::resize(
                    &original_image,
                    nw,
                    nh,
                    image::imageops::FilterType::Lanczos3,
                );
            } else {
                nw = width;
                nh = height;
            }

            let mut output = Vec::<u8>::new();
            let encoder = image::ico::IcoEncoder::new(&mut output);
            encoder
                .encode(&original_image, nw, nh, img_color_type)
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
        Format::Heic => {
            let mut image = libheif_rs::Image::new(
                width,
                height,
                libheif_rs::ColorSpace::Rgb(libheif_rs::RgbChroma::C444),
            )
            .map_err(|_| "Cannot encode image")?;

            image
                .create_plane(libheif_rs::Channel::R, width, height, 8)
                .map_err(|_| "Cannot encode image")?;
            image
                .create_plane(libheif_rs::Channel::G, width, height, 8)
                .map_err(|_| "Cannot encode image")?;
            image
                .create_plane(libheif_rs::Channel::B, width, height, 8)
                .map_err(|_| "Cannot encode image")?;
            image
                .create_plane(libheif_rs::Channel::Alpha, width, height, 8)
                .map_err(|_| "Cannot encode image")?;

            let planes = image.planes_mut();
            let plane_r = planes.r.unwrap();
            let stride = dbg!(plane_r.stride);

            let data_r = plane_r.data;
            let data_g = planes.g.unwrap().data;
            let data_b = planes.b.unwrap().data;
            let data_a = planes.a.unwrap().data;

            // Fill data of planes by some "pixels"
            for y in 0..height {
                let mut row_start = stride * y as usize;
                let mut row_start_webp = (4 * width * y) as usize;
                for _ in 0..width {
                    data_r[row_start] = img_data[row_start_webp];
                    data_g[row_start] = img_data[row_start_webp + 1];
                    data_b[row_start] = img_data[row_start_webp + 2];
                    data_a[row_start] = img_data[row_start_webp + 3];
                    row_start += 1;
                    row_start_webp += 4;
                }
            }

            // Encode image and save it into file.
            let mut context = libheif_rs::HeifContext::new().map_err(|_| "Cannot encode image")?;
            let mut encoder = context
                .encoder_for_format(libheif_rs::CompressionFormat::Hevc)
                .map_err(|_| "Cannot encode image")?;
            encoder
                .set_quality(libheif_rs::EncoderQuality::LossLess)
                .map_err(|_| "Cannot encode image")?;
            context
                .encode_image(&image, &mut encoder, None)
                .map_err(|_| "Cannot encode image")?;
            Ok(context
                .write_to_bytes()
                .map_err(|_| "Cannot encode image")?)
        }
    }
}
