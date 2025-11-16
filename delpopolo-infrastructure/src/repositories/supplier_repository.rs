use sqlx::PgPool;
use delpopolo_domain::Supplier;

pub struct SupplierRepository {
    _pool: PgPool,
}

impl SupplierRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { _pool: pool }
    }
    
    pub async fn find_active_suppliers(&self) -> Vec<Supplier> {
        // TODO: Implement
        vec![]
    }
}
