use crate::schema::GraphqlWebData;
use crate::{schema, utils};
use actix_multipart::Multipart;
use actix_web::HttpResponse;
use actix_web::{web, HttpRequest, Responder};
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::io::{Error, Read};
use std::path::Path;

mod util;
use util::*;

//graphql config
/// GraphiQL playground UI
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
pub async fn graphql(
    st: web::Data<GraphqlWebData>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
) -> impl Responder {
    let token = req.headers().get("authorization");

    match token {
        Some(token_value) => {
            //use ssh connection in context
            let ctx = schema::Context {
                sess: st.sess.clone(),
                //set auth token to context
                auth_token: Some(token_value.to_str().unwrap().to_string()),
                args: st.args.clone(),
            };
            let value = data.execute(&st.schema, &ctx).await;
            HttpResponse::Ok().json(value)
        }
        None => {
            //use ssh connection in context
            let ctx = schema::Context {
                sess: st.sess.clone(),
                auth_token: None,
                args: st.args.clone(),
            };
            let value = data.execute(&st.schema, &ctx).await;
            HttpResponse::Ok().json(value)
        }
    }
}

pub async fn read_file(
    req: HttpRequest,
    // Read file path
    info: web::Query<schema::PathQuery>,
) -> Result<impl Responder, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let file = actix_files::NamedFile::open_async(file_path).await?;
    Ok(file.into_response(&req))
}

pub async fn upload(
    payload: Multipart,
    // Directory you want to add file
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

pub async fn read_remote_file(
    sess: web::Data<GraphqlWebData>,
    info: web::Query<schema::PathQuery>,
) -> Result<HttpResponse, Error> {
    let (mut remote_file, _) = sess
        .sess
        .as_ref()
        .unwrap()
        .scp_recv(&Path::new(&info.path))?;

    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents)?;
    Ok(buffer_response(contents))
}

pub async fn upload_remote_file(
    sess: web::Data<GraphqlWebData>,
    payload: Multipart,
    // Directory you want to add file
    info: web::Query<schema::PathQuery>,
) -> Result<HttpResponse, Error> {
    let file_path = std::path::Path::new(&info.path);
    utils::check_auth_path(file_path)?;
    let upload_status = save_remote_file(payload, sess.sess.as_ref().unwrap(), &file_path).await;
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
