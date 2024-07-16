use async_graphql::{Context, Enum, Object};
use chrono::{TimeZone, Utc};
use entity::bakery;
use crate::schema::GQLGlobalData;


pub struct BakeryType(bakery::Model);

impl From<bakery::Model> for BakeryType {
    fn from(item: bakery::Model) -> Self {
        BakeryType(item)
    }
}

#[Object]
impl BakeryType {
    async fn name(&self) -> Option<&String> {
        self.0.name.as_ref()
    }


}
