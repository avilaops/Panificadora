use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use delpopolo_domain::{Inventory, InventoryMovement};

pub struct InventoryRepository {
    pool: PgPool,
}

impl InventoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn find_by_product_id(&self, product_id: Uuid) -> Result<Option<Inventory>> {
        let inventory = sqlx::query_as!(
            InventoryRow,
            r#"
            SELECT id, product_id, quantity, reserved_quantity, available_quantity,
                   last_movement_at, created_at, updated_at
            FROM inventory
            WHERE product_id = $1
            "#,
            product_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(inventory.map(|row| row.into()))
    }
    
    pub async fn save(&self, inventory: &Inventory) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO inventory (
                id, product_id, quantity, reserved_quantity, available_quantity,
                last_movement_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (product_id) DO UPDATE SET
                quantity = $3,
                reserved_quantity = $4,
                available_quantity = $5,
                last_movement_at = $6,
                updated_at = $8
            "#,
            inventory.id,
            inventory.product_id,
            inventory.quantity,
            inventory.reserved_quantity,
            inventory.available_quantity,
            inventory.last_movement_at,
            inventory.created_at,
            inventory.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn save_movement(&self, movement: &InventoryMovement) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO inventory_movements (
                id, product_id, movement_type, quantity, unit_cost, total_cost,
                order_id, supplier_id, nfe_key, notes, performed_by, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            movement.id,
            movement.product_id,
            movement.movement_type as _,
            movement.quantity,
            movement.unit_cost,
            movement.total_cost,
            movement.order_id,
            movement.supplier_id,
            movement.nfe_key,
            movement.notes,
            movement.performed_by,
            movement.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

struct InventoryRow {
    id: Uuid,
    product_id: Uuid,
    quantity: f64,
    reserved_quantity: f64,
    available_quantity: f64,
    last_movement_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<InventoryRow> for Inventory {
    fn from(row: InventoryRow) -> Self {
        Inventory {
            id: row.id,
            product_id: row.product_id,
            quantity: row.quantity,
            reserved_quantity: row.reserved_quantity,
            available_quantity: row.available_quantity,
            last_movement_at: row.last_movement_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
