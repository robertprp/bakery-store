use buildstructor::buildstructor;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct CreateStockDTO {
    pub product_id: Uuid,
    pub quantity: Decimal,
}

#[derive(Default)]
pub struct UpdateStockDTO {
    pub product_id: Option<Uuid>,
    pub quantity: Option<Decimal>,
}

#[buildstructor]
impl CreateStockDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        product_id: Uuid,
        quantity: Decimal,
    ) -> Self {
        Self {
            product_id,
            quantity,
        }
    }
}

#[buildstructor]
impl UpdateStockDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        product_id: Option<Uuid>,
        quantity: Option<Decimal>,
    ) -> Self {
        Self {
            product_id,
            quantity,
        }
    }
}
