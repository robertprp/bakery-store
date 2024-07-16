use crate::schema::validators::timestamp::TimestampValidator;
use async_graphql::InputObject;
use entity::account::ReferrerProvider;
use serde::{Deserialize, Serialize};
use service::account::dto::Referrer;

#[derive(Clone, Debug, Serialize, InputObject)]
pub struct LoginSignatureInput {
    /// Timestamp used for the signature
    #[graphql(validator(custom = "TimestampValidator::new(30, 120)"))]
    pub timestamp: String,

    /// Signature for provided timestamp
    #[graphql(validator(min_length = 130, max_length = 130))]
    pub signature: String,

    /// Referrer information (Optional)
    pub referrer: Option<ReferrerInput>,
}

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
pub struct ReferrerInput {
    pub referrer_code: String,
    pub referrer_provider: ReferrerProvider,
}

impl From<ReferrerInput> for Referrer {
    fn from(input: ReferrerInput) -> Self {
        Referrer {
            referrer_code: input.referrer_code,
            referrer_provider: input.referrer_provider,
        }
    }
}
