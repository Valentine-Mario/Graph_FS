use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::{self, Data};
use actix_web::{Error, FromRequest, HttpRequest};
use futures::Future;

use crate::auth::jwt;
use crate::cli::Args;
use crate::schema::GraphqlWebData;
use crate::user_setting::manage_config::get_user;

pub struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req
            .app_data::<Data<GraphqlWebData>>()
            .expect("error getting app state")
            .to_owned();
        let req = req.to_owned();
        Box::pin(async move {
            if is_authorized(req, state).await {
                Ok(Authorized)
            } else {
                Err(ErrorUnauthorized("not authorized"))?
            }
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}

async fn is_authorized(req: HttpRequest, state: web::Data<GraphqlWebData>) -> bool {
    let args = Args::new();

    if args.use_auth.is_some() && args.use_auth.unwrap() {
        if let Some(value) = req.headers().get("authorization") {
            if let Ok(user) = jwt::decode_token(value.to_str().unwrap_or(""), args.jwt_secret) {
                state
                    .db_conn
                    .clone()
                    .expect("error fetching db pool")
                    .get_user_by_email(&user)
                    .await
                    .is_ok()
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

pub fn check_write_access(args: Args, token: &str) -> bool {
    if args.use_auth.is_some() && args.use_auth.unwrap() {
        if let Ok(user) = jwt::decode_token(token, args.jwt_secret) {
            if let Ok(usr_details) = get_user(&user) {
                match usr_details.get("permission") {
                    Some(permission) => return permission.as_str().unwrap_or("").trim() != "read",
                    None => return true,
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
