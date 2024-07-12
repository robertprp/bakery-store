use rust_decimal::Decimal;
use uuid;

pub struct CreateOrderProductDTO {
    pub order_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub quantity: Decimal,
    pub total_price: Decimal,
}

#[derive(Default)]
pub struct UpdateOrderProductDTO {
    pub order_id: Option<uuid::Uuid>,
    pub product_id: Option<uuid::Uuid>,
    pub quantity: Option<Decimal>,
    pub total_price: Option<Decimal>,
}

#[buildstructor::buildstructor]
impl CreateOrderProductDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        order_id: uuid::Uuid,
        product_id: uuid::Uuid,
        quantity: Decimal,
        total_price: Decimal,
    ) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
            total_price,
        }
    }
}

#[buildstructor::buildstructor]
impl UpdateOrderProductDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        order_id: Option<uuid::Uuid>,
        product_id: Option<uuid::Uuid>,
        quantity: Option<Decimal>,
        total_price: Option<Decimal>,
    ) -> Self {
        Self {
            order_id,
            product_id,
            quantity,
            total_price,
        }
    }
}