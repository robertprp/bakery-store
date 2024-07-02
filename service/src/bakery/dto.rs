use chrono::{DateTime, Utc};

pub struct CreateBakeryDTO {
    pub name: String,
    pub active_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[buildstructor::buildstructor]
impl CreateBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        active_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name,
            created_at,
            updated_at,
            active_at
        }
    }
}

#[derive(Default)]
pub struct UpdateBakeryDTO {
    pub name: Option<String>,
    pub active_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[buildstructor::buildstructor]
impl UpdateBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        name: Option<String>,
        active_at: Option<DateTime<Utc>>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            name,
            active_at,
            updated_at,
        }
    }
}

#[derive(Debug)]
pub struct DeleteBakeryDTO {
    pub id: i32,
    pub deleted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[buildstructor::buildstructor]
impl DeleteBakeryDTO {
    #[builder(entry = "builder", exit = "build", visibility = "pub")]
    fn build_new(
        id: i32,
        deleted_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            deleted_at,
            updated_at,
        }
    }
}