use super::{conversion, file_encoding};
use actix_web::{get, post, web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Upload {
    img: String,
}

#[get("/{name}.{ext}")]
pub async fn get_file(web::Path((name, ext)): web::Path<(String, String)>) -> Result<HttpResponse> {
    let fmt: conversion::Format = ext.clone().into();
    if !std::path::Path::new(&format!("pool/{}.{}", name, ext)).exists() {
        Ok(HttpResponse::NotFound().finish())
    } else {
        let img = std::fs::read(&format!("pool/{}.{}", name, ext))
            .map_err(|_| HttpResponse::NotFound())?;
        Ok(HttpResponse::Ok()
            .content_type(fmt.get_content_type())
            .body(img))
    }
}

#[post("/")]
pub async fn save_file(form: web::Form<Upload>) -> Result<String> {
    let image =
        conversion::decode_image(&form.img).map_err(|x| HttpResponse::BadRequest().body(x))?;
    let hash = file_encoding::hash_file(&image.0);
    if !std::path::Path::new(&format!("pool/{}", hash)).exists() {
        // Save file
        std::fs::write(format!("pool/{}.{}", hash, image.1.get_ext()), image.0)
            .map_err(|_| HttpResponse::InsufficientStorage())?;
    }
    Ok(format!("https://i.shlnk.eu/{}.{}", hash, image.1.get_ext()))
}
