use async_graphql::{Context, Result, Subscription};
use async_graphql::futures_util::Stream;
use service::message_broker::service::Event;
use crate::schema::GQLGlobalData;
use crate::schema::types::bakery::BakeryType;
use futures::StreamExt;
use std::iter::Iterator;
pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn new_bakery_product(&self, ctx: &Context<'_>, uid: Option<String>) -> Result<impl Stream<Item = BakeryType>> {
        let context = ctx.data_unchecked::<GQLGlobalData>();

        Ok(context
                .services
                .message_broker
                .subscribe()
                .await
                .filter_map(move |event| {
                    futures::future::ready(match event {
                        Event::BakeryCreated(event) => {
                            if let Some(ref uid) = uid {
                                if event.id.to_string().ne(uid) {
                                    return futures::future::ready(None);
                                }
                            }
                            Some(event.into())
                        }
                        _ => None,
                    })
                }),
        )
    }
}