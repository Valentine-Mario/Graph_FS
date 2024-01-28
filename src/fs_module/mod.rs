use crate::{auth::check_access::check_write_access, schema::Context};

pub mod handler;
pub mod local_fs;
pub mod remote_fs;
pub mod utils;

pub fn graphql_write_access(context: &Context) -> bool {
    if context.args.use_auth.is_some() && context.args.use_auth.unwrap() {
        let token = context.clone().auth_token.unwrap_or("".to_string());
        return check_write_access(context.args.clone(), &token);
    }
    true
}
