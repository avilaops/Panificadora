use sqlx::PgPool;
use delpopolo_domain::Customer;

pub struct CustomerRepository {
    _pool: PgPool,
}

impl CustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { _pool: pool }
    }
    
    pub async fn find_by_cpf(&self, _cpf: &str) -> Option<Customer> {
        // TODO: Implement
        None
    }
}
