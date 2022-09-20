use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub struct Query;

#[Object]
impl Query {
    async fn greeting(&self) -> String {
        "Hello".to_string()
    }
}

pub type ExampleSchema = Schema<Query, EmptyMutation, EmptySubscription>;
