use buildstructor::buildstructor;
use rust_decimal::Decimal;
use uuid::Uuid;

pub struct CreateProductDTO {
    pub name: String,
    pub price: Decimal,
}

#[derive(Default)]
pub struct UpdateProductDTO {
    pub name: Option<String>,
    pub price: Option<Decimal>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub active_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[buildstructor::buildstructor]
impl CreateProductDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: String,
        price: Decimal,
    ) -> Self {
        Self {
            name,
            price,
        }
    }
}

#[buildstructor::buildstructor]
impl UpdateProductDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: Option<String>,
        price: Option<Decimal>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
        active_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        Self {
            name,
            price,
            deleted_at,
            active_at,
        }
    }
}

