mod api;
mod cli;
mod fs_module;
pub mod schema;
pub mod utils;

use std::{
    io::{self},
    sync::Arc,
};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use ssh2::Session;

use crate::schema::{create_schema, GraphqlWebData};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
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

// Main folder
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = cli::Args::new();

    log::info!("Starting HTTP server on port {}", args.port);
    log::info!(
        "GraphiQL playground: http://{}:{}/graphiql",
        args.host,
        args.port
    );

    // Handle remote FS http server
    if args.remote.is_some() && args.remote.unwrap() {
        let arg = args.clone();
        // Start HTTP server
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
        .bind((arg.host, arg.port))?
        .run()
        .await
    } else {
        let arg = args.clone();
        // Start local FS HTTP server
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
        .bind((arg.host, arg.port))?
        .run()
        .await
    }
}
//./target/debug/graph_fs -p 8000 -h 127.0.0.1 --auth_path /home/dead/Documents
//remote
//./target/debug/graph_fs -p 8000 -h 127.0.0.1 --remote true --auth_option user_password --remote_host 127.0.0.1 --remote_port 22 --username <name> --password <pass>
