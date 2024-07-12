use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct CreateOrderDTO {
    pub price: Decimal,
    pub bakery_id: Uuid,
}

#[buildstructor::buildstructor]
impl CreateOrderDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        bakery_id: Uuid,
        price: Decimal,
    ) -> Self {
        Self {
            bakery_id,
            price
        }
    }
}

pub struct UpdateOrderDTO {
    pub bakery_id: Option<Uuid>,
    pub price: Option<Decimal>,
    pub deleted_at: Option<DateTime<Utc>>
}

#[buildstructor::buildstructor]
impl UpdateOrderDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        bakery_id: Option<Uuid>,
        price: Option<Decimal>,
        deleted_at: Option<DateTime<Utc>>
    ) -> Self {
        Self {
            bakery_id,
            price,
            deleted_at
        }
    }
}