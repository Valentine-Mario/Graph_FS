use crate::{
    auth::{bcrypt_util::compare_password, jwt::create_token},
    schema::{GraphqlWebData, JsonRes, LoginUser},
    user_setting::manage_config::get_user,
};
use actix_web::{web, HttpResponse};
use std::io::Error;

pub async fn login(
    st: web::Data<GraphqlWebData>,
    data: web::Json<LoginUser>,
) -> Result<HttpResponse, Error> {
    match get_user(&data.email, &st.db_conn.clone().unwrap()).await {
        Ok(mut user) => match user.pop() {
            Some(user) => match compare_password(&user.password, &data.password) {
                Ok(verify) => {
                    if verify {
                        let duration = st.args.jwt_duration.unwrap_or(30);
                        if let Ok(token) =
                            create_token(&user.email, duration, st.args.clone().jwt_secret)
                        {
                            Ok(HttpResponse::Ok().json(JsonRes { data: token }))
                        } else {
                            Ok(HttpResponse::BadGateway()
                                .content_type("text/plain")
                                .body("Error creating token"))
                        }
                    } else {
                        Ok(HttpResponse::Unauthorized()
                            .content_type("text/plain")
                            .body("Invalid password"))
                    }
                }
                Err(e) => {
                    let body = format!("{:?}", e);
                    Ok(HttpResponse::BadGateway()
                        .content_type("text/plain")
                        .body(body))
                }
            },
            None => Ok(HttpResponse::Unauthorized()
                .content_type("text/plain")
                .body("invalid password")),
        },
        Err(e) => {
            let body = format!("{:?}", e);
            Ok(HttpResponse::Forbidden()
                .content_type("text/plain")
                .body(body))
        }
    }
}
