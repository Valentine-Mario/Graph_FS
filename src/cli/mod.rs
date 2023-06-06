use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "GraphFS ", about = "GraphFS tool")]
pub struct Args {
    //select host
    #[structopt(short = "h", long = "host")]
    pub host: String,

    //select port
    #[structopt(short = "p", long = "port")]
    pub port: u16,

    //authorized path
    #[structopt(long = "auth_path")]
    pub authorized_path: String,

    //define workers
    #[structopt[short = "w", long = "worker"]]
    pub worker: Option<usize>,

    //simple deploy payload
    #[structopt(long = "remote")]
    pub remote: Option<bool>,

    //auth option
    #[structopt(long = "auth_option")]
    pub auth_option: Option<AuthOption>,

    //username
    #[structopt(long = "username", about = "The auth usename")]
    pub username: Option<String>,

    //password
    #[structopt(long = "password", about = "The auth password")]
    pub password: Option<String>,

    //publick key
    #[structopt(long = "public_key")]
    pub pub_key: Option<String>,

    //private key
    #[structopt(long = "private_key")]
    pub private_key: Option<String>,

    //passphrase
    #[structopt(long = "passphrase")]
    pub pass_phrase: Option<String>,

    //remote host
    #[structopt(long = "remote_host")]
    pub remote_host: Option<String>,

    //remote port
    #[structopt(long = "remote_port")]
    pub remote_port: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        Args::from_args()
    }
}

#[derive(Debug, Clone)]
///Auth options for ssh credentials
pub enum AuthOption {
    ///Attempt basic password authentication.
    UserauthPassword,
    ///authenticate the current connection with the first public key found in an SSH agent
    UserauthAgent,
    ///Attempt public key authentication using a PEM encoded private key file stored on disk.
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
