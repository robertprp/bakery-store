use crate::bakery::repository::BakeryRepository;
use crate::services::Services;

pub struct BakeryService {
    pub services: Services,
    pub repository: BakeryRepository,
}

impl BakeryService {
    pub fn new(services: Services, repository: BakeryRepository) -> Self {
        Self { services, repository }
    }
}