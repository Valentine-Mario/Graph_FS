use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "GraphFS ", about = "GraphFS tool")]
pub struct Args {
    // Select host
    #[structopt(short = "h", long = "host")]
    pub host: String,

    // Select port
    #[structopt(short = "p", long = "port")]
    pub port: u16,

    // Authorized path
    #[structopt(long = "auth_path")]
    pub authorized_path: String,

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
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
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

type ParseError = &'static str;

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
