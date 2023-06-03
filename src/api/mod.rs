use std::io::Error;

use crate::{schema, utils};
use actix_web::{route, web, HttpRequest, Responder};

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
