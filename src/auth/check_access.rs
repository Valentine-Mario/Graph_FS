use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::Future;

use crate::cli::Args;

pub struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            Box::pin(async move { Ok(Authorized) })
        } else {
            Box::pin(async move { Err(ErrorUnauthorized("not authorized"))? })
        }
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    let args = Args::new();

    if args.use_auth.is_some() && args.use_auth.unwrap() {
        if let Some(value) = req.headers().get("authorized") {
            println!("token {:?}", value);
            true
        } else {
            println!("unauthorized");
            false
        }
    } else {
        true
    }
}
