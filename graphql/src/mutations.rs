use async_graphql::Object;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_bakery(&self, name: String) -> &'static str {
        "Hello, world!"
    }
}