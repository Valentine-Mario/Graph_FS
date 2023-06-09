use crate::{schema, utils, Session};
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use actix_web::{route, web, HttpRequest, Responder};
use std::io::{Error, Read};
use std::path::Path;

mod util;
use util::*;

#[route("/get_local_file", method = "GET")]
async fn read_file(
    req: HttpRequest,
    //read file path
    info: web::Query<schema::PathQuery>,
) -> Result<impl Responder, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let file = actix_files::NamedFile::open_async(file_path).await?;
    Ok(file.into_response(&req))
}

#[route("/add_local_file", method = "POST")]
pub async fn upload(
    payload: Multipart,
    //directory you want to add file
    info: web::Query<schema::PathQuery>,
) -> Result<HttpResponse, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let upload_status = save_local_file(payload, &file_path).await;
    match upload_status {
        Ok(val) => match val {
            Some(true) => Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("update_succeeded")),
            _ => Ok(HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("update_failed")),
        },
        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
}

#[route("/get_remote_file", method = "GET")]
pub async fn read_remote_file(
    sess: web::Data<Session>,
    info: web::Query<schema::PathQuery>,
) -> Result<HttpResponse, Error> {
    let (mut remote_file, _) = sess.scp_recv(&Path::new(&info.path))?;

    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents)?;
    Ok(buffer_response(contents))
}

#[route("/add_remote_file", method = "POST")]
pub async fn upload_remote_file(
    sess: web::Data<Session>,
    payload: Multipart,
    //directory you want to add file
    info: web::Query<schema::PathQuery>,
) -> Result<HttpResponse, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let upload_status = save_remote_file(payload, sess, &file_path).await;
    match upload_status {
        Ok(val) => match val {
            Some(true) => Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("update_succeeded")),
            _ => Ok(HttpResponse::BadRequest()
                .content_type("text/plain")
                .body("update_failed")),
        },
        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
}
