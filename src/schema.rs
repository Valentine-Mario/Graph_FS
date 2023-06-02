use std::pin::Pin;

use juniper::futures::Stream;
use juniper::Context as JuniperContext;
use juniper::{futures, graphql_subscription, FieldResult, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use ssh2::Session;

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

pub struct Context {
    pub sess: Session,
}

impl JuniperContext for Context {}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A simple representation of a file struct")]
pub struct File {
    pub name: String,
    pub size: f64,
    pub file_type: String,
    pub parent_folder: String,
}

#[derive(GraphQLObject, Debug)]
#[graphql(description = "A simple folder representation")]
pub struct Folder {
    pub name: String,
    pub content_length: i32,
    pub parent_folder: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: i32,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

pub struct MutationRoot;
pub struct Subscription;

impl File {
    pub fn new(name: String, size: f64, file_type: String, parent_folder: String) -> Self {
        File {
            name,
            size,
            file_type,
            parent_folder,
        }
    }
}

impl Folder {
    pub fn new(name: String, content_length: i32, parent_folder: String) -> Self {
        Folder {
            name,
            content_length,
            parent_folder,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: 235,
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

type StringStream = Pin<Box<dyn Stream<Item = String> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream = futures::stream::iter(vec![String::from("hello world")]);
        Box::pin(stream)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, Subscription {})
}
