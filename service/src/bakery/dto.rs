use chrono::{DateTime, Utc};

pub struct CreateBakeryDTO {
    pub name: String,
}

#[buildstructor::buildstructor]
impl CreateBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: String,
    ) -> Self {
        Self {
            name,
        }
    }
}

#[derive(Default)]
pub struct UpdateBakeryDTO {
    pub name: Option<String>,
    pub active_at: Option<DateTime<Utc>>,
}

#[buildstructor::buildstructor]
impl UpdateBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: Option<String>,
        active_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name,
            active_at,
        }
    }
}

#[derive(Debug)]
pub struct DeleteBakeryDTO {
    pub id: uuid::Uuid,
    pub deleted_at: DateTime<Utc>,
}

#[buildstructor::buildstructor]
impl DeleteBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        id: uuid::Uuid,
        deleted_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            deleted_at,
        }
    }
}