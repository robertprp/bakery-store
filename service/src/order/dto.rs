use rust_decimal::Decimal;
use uuid::Uuid;

pub struct CreateOrderDTO {
    pub name: String,
    pub product_id: Uuid,
    pub bakery_id: Uuid,
    pub quantity: u32,
    pub price: Decimal,
}

#[buildstructor::buildstructor]
impl CreateOrderDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: String,
        product_id: Uuid,
        bakery_id: Uuid,
        quantity: u32,
        price: Decimal,
    ) -> Self {
        Self {
            name,
            product_id,
            bakery_id,
            quantity,
            price,
        }
    }
}

pub struct UpdateOrderDTO {
    pub name: Option<String>,
    pub product_id: Option<Uuid>,
    pub bakery_id: Option<Uuid>,
    pub quantity: Option<u32>,
    pub price: Option<Decimal>,
}

#[buildstructor::buildstructor]
impl UpdateOrderDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: Option<String>,
        product_id: Option<Uuid>,
        bakery_id: Option<Uuid>,
        quantity: Option<u32>,
        price: Option<Decimal>,
    ) -> Self {
        Self {
            name,
            product_id,
            bakery_id,
            quantity,
            price,
        }
    }
}