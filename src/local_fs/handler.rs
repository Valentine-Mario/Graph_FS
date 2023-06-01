use juniper::FieldResult;

use crate::schema::{Context, Episode, Human, QueryRoot};

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    async fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: 3553,
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}
