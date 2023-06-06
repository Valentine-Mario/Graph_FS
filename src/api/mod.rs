use std::io::Error;
use std::path::Path;

use crate::{schema, utils, Session};
use actix_web::HttpResponse;
use actix_web::{route, web, HttpRequest, Responder};
use async_stream::__private::AsyncStream;
use async_stream::try_stream;
use bytes::Bytes;
use std::io::Write;
use uuid::Uuid;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

#[route("/get_local_file", method = "GET")]
async fn read_file(
    req: HttpRequest,
    //read filee path
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
pub async fn read_remote_file(sess: web::Data<Session>) -> Result<HttpResponse, Error> {
    log::info!("session authenticated {}", sess.authenticated());
    sess.sftp().unwrap().mkdir(Path::new("ffff"), 1).unwrap();
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("update_succeeded"))
}

async fn save_local_file(mut payload: Multipart, file_path: &Path) -> Result<Option<bool>, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let uid = Uuid::new_v4().to_string();
        let new_name = format!("{}-{}", uid, filename);
        let filepath = file_path.join(new_name);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()?;
        }
    }

    Ok(Some(true))
}

//stream buffer to client
fn buffer_response(mut x: Vec<u8>) -> HttpResponse {
    //if buffer is less than 4kb, just return buffer
    if x.len() <= 4096 {
        HttpResponse::Ok().body(x)
    } else {
        let stream: AsyncStream<Result<Bytes, Error>, _> = try_stream! {
            //stream large buffer files
            loop{
                if x.len()>4096{
                    let u:Vec<u8>=x.drain(0..4096).collect();

                    yield Bytes::copy_from_slice(&u[..4096]);
                }else{
                    let u:Vec<u8>=x.drain(0..x.len()).collect();
                    if u.len()==0{
                        break
                    }
                    yield Bytes::copy_from_slice(&u[..u.len()]);

                }
            }
        };
        HttpResponse::Ok().streaming(stream)
    }
}
