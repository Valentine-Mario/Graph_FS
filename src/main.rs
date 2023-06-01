mod cli;
mod fs_utils;
mod local_fs;
mod remote_fs;
pub mod schema;

use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use ssh2::Session;

use crate::schema::{create_schema, Schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    //initialize context here
    let args = cli::Args::new();
    let sess = Session::new();

    match sess {
        Ok(mut sess) => {
            if args.remote.is_some() && args.remote.unwrap() {
                //create authenticated session
                sess = remote_fs::connection(&args, sess).unwrap();
            }
            println!("session {}", sess.authenticated());
            let ctx = schema::Context { sess };
            let user = data.execute(&st, &ctx).await;
            HttpResponse::Ok().json(user)
        }
        Err(_) => HttpResponse::ExpectationFailed().body("Error setting session"),
    }
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = cli::Args::new();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port {}", args.port);
    log::info!(
        "GraphiQL playground: http://{}:{}/graphiql",
        args.host,
        args.port
    );

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
        // .wrap(ctx.clone())
    })
    .workers(2)
    .bind((args.host, args.port))?
    .run()
    .await
}
//./target/debug/graph_fs -p 8000 -h 127.0.0.1
//remote
//./target/debug/graph_fs -p 8000 -h 127.0.0.1 --remote true --auth_option user_password --remote_host 127.0.0.1 --remote_port 22 --username <name> --password <pass>
