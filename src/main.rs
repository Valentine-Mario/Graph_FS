mod api;
pub mod auth;
mod cli;
mod fs_module;
pub mod http_config;
pub mod schema;
pub mod user_setting;
pub mod utils;
use ssh2::Session;

// Main folder
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = cli::Args::new();

    if args.manage_users.is_none() {
        let a = args.clone();
        //it is safe to unwrap because the user has entered graphQL mode here
        log::info!("Starting HTTP server on port {}", a.port.unwrap());
        log::info!(
            "GraphiQL playground: http://{}:{}/graphiql",
            a.host.unwrap(),
            a.port.unwrap()
        );

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
        user_setting::manage_update(&args).unwrap();
        Ok(())
    }
}
//./target/debug/graph_fs -p 8000 -h 127.0.0.1 --auth_path /home/dead/Documents
//remote
//./target/debug/graph_fs -p 8000 -h 127.0.0.1 --remote true --auth_option user_password --remote_host 127.0.0.1 --remote_port 22 --username <name> --password <pass>
