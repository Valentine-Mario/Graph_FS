use std::pin::Pin;

use juniper::futures::Stream;
use juniper::Context as JuniperContext;
use juniper::GraphQLObject;
use juniper::{futures, graphql_subscription, RootNode};
use serde::{Deserialize, Serialize};
use ssh2::Session;

use crate::cli::Args;

#[derive(Debug, Clone, Copy)]
pub enum MySshFileType {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
    Other,
}

impl JuniperContext for Context {}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A simple representation of a file struct")]
pub struct File {
    pub name: String,
    pub size: f64,
    pub file_type: String,
    pub parent_folder: String,
    pub last_modified: f64,
}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A simple folder representation")]
pub struct Folder {
    pub name: String,
    pub content_length: i32,
    pub parent_folder: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRes {
    pub data: String,
}

#[derive(Deserialize)]
pub struct PathQuery {
    pub path: String,
}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "Message on completion of action")]
pub struct Message {
    pub msg: String,
}
#[derive(Debug)]
pub struct QueryRoot;

#[derive(Debug)]
pub struct MutationRoot;

#[derive(Debug)]
pub struct Subscription;

impl Message {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl File {
    pub fn new(
        name: String,
        size: f64,
        file_type: String,
        parent_folder: String,
        last_modified: f64,
    ) -> Self {
        Self {
            name,
            size,
            file_type,
            parent_folder,
            last_modified,
        }
    }
}

impl Folder {
    pub fn new(name: String, content_length: i32, parent_folder: String) -> Self {
        Self {
            name,
            content_length,
            parent_folder,
        }
    }
}

type BufferStream = Pin<Box<dyn Stream<Item = i32> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn hello_world() -> BufferStream {
        let stream = futures::stream::iter(vec![4, 10, 11]);
        Box::pin(stream)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, Subscription {})
}

pub struct GraphqlWebData {
    pub sess: Option<Session>,
    pub schema: Schema,
    pub args: Args,
}

#[derive(Clone)]
pub struct Context {
    pub sess: Option<Session>,
    pub auth_token: Option<String>,
    pub args: Args,
}
