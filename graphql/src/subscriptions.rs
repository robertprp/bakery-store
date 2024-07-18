use async_graphql::{Context, Result, Subscription};
use async_graphql::futures_util::Stream;
use crate::schema::GQLGlobalData;
use crate::schema::types::bakery::BakeryType;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_bakery_product(&self, ctx: Context<'_>, uid: Option<String>) -> Result<impl Stream<Item = BakeryType>> {
        let context = ctx.data_unchecked::<GQLGlobalData>();

        Ok(context.services.message_broker.subscribe().await.filter_map(move |event| {
            async move {
                if let Some(uid) = &uid {
                    if event.uid.eq(uid) {
                        Some(BakeryType::from(event))
                    } else {
                        None
                    }
                } else {
                    Some(BakeryType::from(event))
                }
            }
        }))
    }
}