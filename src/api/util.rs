use std::io::Error;
use std::path::Path;

use crate::Session;
use actix_web::web;
use actix_web::HttpResponse;
use async_stream::__private::AsyncStream;
use async_stream::try_stream;
use bytes::Bytes;
use std::io::Write;
use uuid::Uuid;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
// Local util
pub async fn save_local_file(
    mut payload: Multipart,
    file_path: &Path,
) -> Result<Option<bool>, Error> {
    // Iterate over multipart stream
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
            // Filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()?;
        }
    }

    Ok(Some(true))
}

pub async fn save_remote_file(
    mut payload: Multipart,
    sessiopn: web::Data<Session>,
    file_path: &Path,
) -> Result<Option<bool>, Error> {
    let mut return_write: Vec<Vec<u8>> = vec![];
    let mut file_name = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();
        let uid = Uuid::new_v4().to_string();
        let new_name = format!("{}-{}", uid, filename);
        file_name = file_path.join(new_name).to_str().unwrap().to_string();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            return_write.push(chunk.unwrap()[..].to_vec());
        }
    }
    let flattened = return_write.into_iter().flatten().collect::<Vec<u8>>();
    let mut remote_file =
        sessiopn.scp_send(Path::new(&file_name), 0o644, flattened.len() as u64, None)?;

    // Write buffer to remote FS
    remote_file.write_all(&flattened)?;
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;
    Ok(Some(true))
}

// Stream buffer to client
pub fn buffer_response(mut x: Vec<u8>) -> HttpResponse {
    // If buffer is less than 4kb, just return buffer
    if x.len() <= 4096 {
        HttpResponse::Ok().body(x)
    } else {
        let stream: AsyncStream<Result<Bytes, Error>, _> = try_stream! {
            // Stream large buffer files
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
