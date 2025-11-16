use sqlx::PgPool;
use delpopolo_domain::Order;

pub struct OrderRepository {
    _pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { _pool: pool }
    }
    
    pub async fn find_by_order_number(&self, _order_number: &str) -> Option<Order> {
        // TODO: Implement
        None
    }
}
