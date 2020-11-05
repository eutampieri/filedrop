use super::{conversion, file_encoding};
use actix_web::{get, post, web, HttpResponse, Result};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Upload {
    img: String,
}

lazy_static! {
    static ref HASHER: std::sync::Mutex<file_encoding::FileHasher> = std::sync::Mutex::default();
}

#[get("/{name}.{ext}")]
pub async fn get_file(web::Path((name, ext)): web::Path<(String, String)>) -> Result<HttpResponse> {
    let fmt: conversion::Format = ext.into();
    if !std::path::Path::new(&format!("pool/{}.webp", name)).exists() {
        Ok(HttpResponse::NotFound().finish())
    } else {
        let webp =
            std::fs::read(&format!("pool/{}.webp", name)).map_err(|_| HttpResponse::NotFound())?;
        let output = conversion::encode_img(&webp, &fmt)
            .map_err(|x| HttpResponse::InternalServerError().body(x))?;
        Ok(HttpResponse::Ok()
            .content_type(fmt.get_content_type())
            .body(output))
    }
}

#[post("/")]
pub async fn save_file(form: web::Form<Upload>) -> Result<String> {
    if super::utils::get_unsorted_pool().unwrap_or(vec![]).len() >= super::file_encoding::MOD {
        return Ok(HttpResponse::InsufficientStorage().finish());
    }
    let image =
        conversion::decode_image(&form.img).map_err(|x| HttpResponse::BadRequest().body(x))?;
    let hash = HASHER
        .lock()
        .map_err(|_| HttpResponse::InternalServerError())?
        .hash_file(&image);
    if !std::path::Path::new(&format!("pool/{}.webp", hash)).exists() {
        // Save file
        std::fs::write(format!("pool/{}.webp", hash), image)
            .map_err(|_| HttpResponse::InsufficientStorage())?;
    }
    Ok(format!("https://i.shlnk.eu/{}", hash))
}
