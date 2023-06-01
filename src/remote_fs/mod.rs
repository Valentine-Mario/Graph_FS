use std::{io::Error, io::ErrorKind, net::TcpStream, path::Path};

use ssh2::Session;

use crate::cli::Args;

pub fn connection(args: &Args, mut sess: Session) -> Result<Session, std::io::Error> {
    let args = args.clone();
    let url_host = format!(
        "{}:{}",
        args.remote_host.unwrap(),
        args.remote_port.unwrap()
    );
    let tcp = TcpStream::connect(url_host)?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    match args.auth_option {
        Some(auth_option) => {
            match auth_option {
                crate::cli::AuthOption::UserauthAgent => {
                    sess.userauth_agent(&args.username.unwrap()).unwrap();
                }
                crate::cli::AuthOption::UserauthPassword => {
                    sess.userauth_password(&args.username.unwrap(), &args.password.unwrap())
                        .unwrap();
                }
                crate::cli::AuthOption::UserauthPubkeyFile => {
                    sess.userauth_pubkey_file(
                        &args.username.unwrap(),
                        Some(Path::new(&args.pub_key.unwrap())),
                        Path::new(&args.private_key.unwrap()),
                        Some(&args.pass_phrase.unwrap()),
                    )
                    .unwrap();
                }
            }
            Ok(sess)
        }
        None => Err(Error::new(
            ErrorKind::InvalidData,
            "No auth option provided",
        )),
    }
}
