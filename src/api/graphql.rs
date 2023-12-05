use crate::auth::check_access::Authorized;
use crate::schema;
use crate::schema::GraphqlWebData;
use actix_web::{route, HttpResponse};
use actix_web::{web, HttpRequest, Responder};
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

//graphql config
/// GraphiQL playground UI
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    st: web::Data<GraphqlWebData>,
    data: web::Json<GraphQLRequest>,
    req: HttpRequest,
    _: Authorized,
) -> impl Responder {
    let token = req.headers().get("authorized");

    match token {
        Some(token_value) => {
            //use ssh connection in context
            let ctx = schema::Context {
                sess: st.sess.clone(),
                //set auth token to context
                auth_token: Some(token_value.to_str().unwrap().to_string()),
                args: st.args.clone(),
            };
            let value = data.execute(&st.schema, &ctx).await;
            HttpResponse::Ok().json(value)
        }
        None => {
            //use ssh connection in context
            let ctx = schema::Context {
                sess: st.sess.clone(),
                auth_token: None,
                args: st.args.clone(),
            };
            let value = data.execute(&st.schema, &ctx).await;
            HttpResponse::Ok().json(value)
        }
    }
}
