use juniper::FieldResult;

use crate::{
    schema::{Context, File, Folder, Message},
    utils::check_auth_path,
};
pub struct RemoteFsQuery;

#[juniper::graphql_object(context = Context)]
impl RemoteFsQuery {
    #[graphql(description = "Returns a list of all files in directory")]
    fn read_file_in_dir(context: &Context, path: String) -> FieldResult<Message> {
        log::info!("authenticated {}", context.sess.authenticated());
        Ok(Message::new(String::from("Item renamed successfully")))
    }
}
