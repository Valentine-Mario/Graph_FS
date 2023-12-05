use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::Future;

use crate::auth::jwt;
use crate::cli::Args;
use crate::user_setting::manage_config::get_user;

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
            if let Ok(user) = jwt::decode_token(&value.to_str().unwrap().to_string(), args) {
                if let Ok(_) = get_user(&user) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        true
    }
}
