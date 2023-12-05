use std::str::FromStr;

use structopt::StructOpt;
type ParseError = &'static str;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "GraphFS ", about = "GraphFS tool")]
pub struct Args {
    // Select host
    #[structopt(short = "h", long = "host")]
    pub host: Option<String>,

    // Select port
    #[structopt(short = "p", long = "port")]
    pub port: Option<u16>,

    // Authorized path
    #[structopt(long = "auth_path")]
    pub authorized_path: Option<String>,

    // Define workers
    #[structopt[short = "w", long = "worker"]]
    pub worker: Option<usize>,

    // Simple deploy payload
    #[structopt(long = "remote")]
    pub remote: Option<bool>,

    // Auth option
    #[structopt(long = "auth_option")]
    pub auth_option: Option<AuthOption>,

    // Username
    #[structopt(long = "username", about = "The auth username")]
    pub username: Option<String>,

    // Password
    #[structopt(long = "password", about = "The auth password")]
    pub password: Option<String>,

    // Public key
    #[structopt(long = "public_key")]
    pub pub_key: Option<String>,

    // Private key
    #[structopt(long = "private_key")]
    pub private_key: Option<String>,

    // Passphrase
    #[structopt(long = "passphrase")]
    pub pass_phrase: Option<String>,

    // Remote host
    #[structopt(long = "remote_host")]
    pub remote_host: Option<String>,

    // Remote port
    #[structopt(long = "remote_port")]
    pub remote_port: Option<String>,

    //cert file
    #[structopt(long = "cert_path")]
    pub cert_path: Option<String>,

    // key file
    #[structopt(long = "key_path")]
    pub key_path: Option<String>,

    //use auth feature
    #[structopt(long = "use_auth")]
    pub use_auth: Option<bool>,

    // manage users
    #[structopt(long = "manage_users")]
    pub manage_users: Option<UserConfig>,

    //name for new user
    #[structopt(long = "acc_name")]
    pub account_name: Option<String>,

    //password for user
    #[structopt(long = "acc_password")]
    pub account_password: Option<String>,

    //account permission
    #[structopt(long = "acc_permission")]
    pub account_permission: Option<String>,

    //new name for when name is updated
    #[structopt(long = "new_acc_name")]
    pub new_account_name: Option<String>,

    //jwt secret
    #[structopt(long = "secret")]
    pub jwt_secret: Option<String>,

    //jwt duration
    #[structopt(long = "jwt_duration")]
    pub jwt_duration: Option<i64>,
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
}

#[derive(Debug, Clone)]
pub enum UserConfig {
    AddUser,
    UpdateUserName,
    DeleteUser,
    UpdateUserPermission,
    UpdateUserPassword,
}

#[derive(Debug, Clone)]
/// Auth options for ssh credentials
pub enum AuthOption {
    /// Attempt basic password authentication.
    UserauthPassword,
    /// Authenticate the current connection with the first public key found in an SSH agent
    UserauthAgent,
    /// Attempt public key authentication using a PEM encoded private key file stored on disk.
    UserauthPubkeyFile,
}

impl FromStr for AuthOption {
    type Err = ParseError;
    fn from_str(types: &str) -> Result<Self, Self::Err> {
        match types {
            "user_password" => Ok(Self::UserauthPassword),
            "user_agent" => Ok(Self::UserauthAgent),
            "user_pub_key" => Ok(Self::UserauthPubkeyFile),
            _ => Err("Could not parse auth type"),
        }
    }
}

impl FromStr for UserConfig {
    type Err = ParseError;
    fn from_str(types: &str) -> Result<Self, Self::Err> {
        match types {
            "add_user" => Ok(Self::AddUser),
            "update_username" => Ok(Self::UpdateUserName),
            "delete_user" => Ok(Self::DeleteUser),
            "update_user_password" => Ok(Self::UpdateUserPassword),
            "update_user_permission" => Ok(Self::UpdateUserPermission),
            _ => Err("could not parse user config"),
        }
    }
}
