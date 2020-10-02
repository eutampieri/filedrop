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
    pub fn get_ext(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
            Self::Png => "png",
            Self::Ico => "ico",
        }
    }
}

pub fn decode_image(img: &str) -> Result<(Vec<u8>, Format), &'static str> {
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
    Ok((
        image,
        Format::from(mime.mime.split("/").nth(1).unwrap().to_owned()),
    ))
}
