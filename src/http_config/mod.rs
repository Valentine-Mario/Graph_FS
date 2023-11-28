use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use ssh2::Session;

use crate::{
    api::{self, graphql, graphql_playground},
    cli::Args,
    fs_module,
    schema::{create_schema, GraphqlWebData},
};

pub async fn local_server(args: Args) -> std::io::Result<()> {
    let arg = args.clone();
    HttpServer::new(move || {
        //contains only schema
        let local_data = Arc::new(GraphqlWebData {
            schema: create_schema(),
            sess: None,
            args: args.clone(),
        });
        App::new()
            .app_data(Data::from(local_data))
            .service(graphql)
            .service(graphql_playground)
            .service(api::read_file)
            .service(api::upload)
            // The GraphiQL UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(arg.worker.unwrap_or(2))
    .bind((arg.host.unwrap(), arg.port.unwrap()))?
    .run()
    .await
}

pub async fn remote_server(args: Args) -> std::io::Result<()> {
    let arg = args.clone();
    HttpServer::new(move || {
        let mut sess: Session = Session::new().expect("Failed to connect to SSH");
        // Create authenticated session
        sess = fs_module::utils::connection(&args, sess).expect("Error creating sessions");
        //contains schema and ssh auth session
        let remote_data = Arc::new(GraphqlWebData {
            schema: create_schema(),
            sess: Some(sess),
            args: args.clone(),
        });
        App::new()
            .app_data(Data::from(remote_data))
            .service(graphql)
            .service(graphql_playground)
            .service(api::read_remote_file)
            .service(api::upload_remote_file)
            .service(api::read_file)
            .service(api::upload)
            // The GraphiQL UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(arg.worker.unwrap_or(2))
    .bind((arg.host.unwrap(), arg.port.unwrap()))?
    .run()
    .await
}

pub async fn local_server_ssl(args: Args) -> std::io::Result<()> {
    let arg = args.clone();
    let ssl_builder = ssl_builder(&args).expect("error build ssl connection");

    let addr = format!("{}:{}", &arg.host.unwrap(), &arg.port.unwrap());

    HttpServer::new(move || {
        //contains only schema
        let local_data = Arc::new(GraphqlWebData {
            schema: create_schema(),
            sess: None,
            args: args.clone(),
        });
        App::new()
            .app_data(Data::from(local_data))
            .service(graphql)
            .service(graphql_playground)
            .service(api::read_file)
            .service(api::upload)
            // The GraphiQL UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(arg.worker.unwrap_or(2))
    .bind_openssl(addr, ssl_builder)?
    .run()
    .await
}

pub async fn remote_server_ssl(args: Args) -> std::io::Result<()> {
    let arg = args.clone();
    let ssl_builder = ssl_builder(&args).expect("error build ssl connection");

    let addr = format!("{}:{}", &arg.host.unwrap(), &arg.port.unwrap());

    HttpServer::new(move || {
        let mut sess: Session = Session::new().expect("Failed to connect to SSH");
        // Create authenticated session
        sess = fs_module::utils::connection(&args, sess).expect("Error creating sessions");
        let remote_data = Arc::new(GraphqlWebData {
            schema: create_schema(),
            sess: Some(sess),
            args: args.clone(),
        });
        App::new()
            .app_data(Data::from(remote_data))
            .service(graphql)
            .service(graphql_playground)
            .service(api::read_file)
            .service(api::upload)
            // The GraphiQL UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(arg.worker.unwrap_or(2))
    .bind_openssl(addr, ssl_builder)?
    .run()
    .await
}

fn ssl_builder(arg: &Args) -> std::io::Result<SslAcceptorBuilder> {
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    ssl_builder.set_private_key_file(&arg.clone().key_path.unwrap(), SslFiletype::PEM)?;
    ssl_builder.set_certificate_chain_file(&arg.clone().cert_path.unwrap())?;
    Ok(ssl_builder)
}