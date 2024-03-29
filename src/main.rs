mod api;
pub mod auth;
mod cli;
pub mod db;
mod fs_module;
pub mod http_config;
pub mod schema;
pub mod user_setting;
pub mod utils;
use resolve_path::PathResolveExt;
use ssh2::Session;

// Main folder
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = cli::Args::new();

    if args.use_auth.is_some() && args.jwt_secret.is_none() {
        panic!("provide jwt secret when using auth mode")
    }

    if args.manage_users.is_none() {
        let a = args.clone();
        //it is safe to unwrap because the user has entered graphQL mode here
        log::info!("Starting HTTP server on port {}", a.port.unwrap());
        log::info!(
            "GraphiQL playground: http://{}:{}/graphiql",
            a.host.unwrap(),
            a.port.unwrap()
        );

        if a.authorized_path.is_none() {
            panic!("You must specify an auth path for the server")
        }

        //resolve path
        let auth_path = a.authorized_path.unwrap();
        let resolved = auth_path.resolve();
        if !resolved.exists() {
            panic!(
                "the auth path {:?} does not exist",
                resolved.to_str().unwrap()
            )
        }

        log::info!("authorized path set at {:?}", resolved.to_str());

        if args.key_path.is_some() && args.cert_path.is_some() {
            // Handle remote FS http server
            if args.remote.is_some() && args.remote.unwrap() {
                // Start remote HTTP server
                http_config::remote_server_ssl(args).await
            } else {
                // Start local FS HTTP server
                http_config::local_server_ssl(args).await
            }
        } else {
            // Handle remote FS http server
            if args.remote.is_some() && args.remote.unwrap() {
                // Start remote HTTP server
                http_config::remote_server(args).await
            } else {
                // Start local FS HTTP server
                http_config::local_server(args).await
            }
        }
    } else {
        //manage account here
        user_setting::manage_update(&args).await?;
        Ok(())
    }
}
