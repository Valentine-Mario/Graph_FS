use std::pin::Pin;

use juniper::futures::Stream;
use juniper::{EmptySubscription, FieldResult, RootNode, graphql_subscription, futures, FieldError};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use juniper::Context as JuniperContext;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

pub struct Context {
    pub counter: i32,
}

impl JuniperContext for Context {}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: i32,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn human(context: &Context, _id: String) -> FieldResult<Human> {
        Ok(Human {
            id: context.counter,
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Context)]
impl MutationRoot {
    fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: context.counter,
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

pub struct  Subscription;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream =
            futures::stream::iter(vec![Ok(String::from("Hello")), Ok(String::from("World!"))]);
        Box::pin(stream)
    }
}


pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, Subscription {})
}