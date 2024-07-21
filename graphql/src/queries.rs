use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn get_home(&self) -> &'static str { env!("CARGO_PKG_VERSION") }
}