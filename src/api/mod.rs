use std::io::{BufReader, Error};
use std::path::PathBuf;

use crate::{schema, utils};
use actix_web::HttpResponse;
use actix_web::{route, web, HttpRequest, Responder};
use async_stream::__private::AsyncStream;
use async_stream::try_stream;
use bytes::Bytes;

#[route("/get_file", method = "GET")]
async fn read_file(
    req: HttpRequest,
    info: web::Query<schema::ReadFileQuery>,
) -> Result<impl Responder, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let file = actix_files::NamedFile::open_async(file_path).await?;
    Ok(file.into_response(&req))
}

//stream buffer to client
fn buffer_response(mut x: Vec<u8>) -> HttpResponse {
    //if buffer is less han 4kb, just return buffer
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
