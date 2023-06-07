use std::path::Path;

use juniper::FieldResult;

use crate::{
    schema::{Context, Message},
    utils::check_auth_path,
};
pub struct RemoteFsQuery;

#[juniper::graphql_object(context = Context)]
impl RemoteFsQuery {
    #[graphql(description = "create file")]
    fn create_file(context: &Context, path: String) -> FieldResult<Message> {
        let path = Path::new(&path);
        check_auth_path(&path)?;
        let sftp = context.sess.sftp()?;
        sftp.create(path)?;
        let return_msg = format!("{:?} created successfully", path);
        Ok(Message::new(String::from(return_msg)))
    }
}
