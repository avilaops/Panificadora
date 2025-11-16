use anyhow::Result;
use uuid::Uuid;
use tracing::{info, warn};
use delpopolo_domain::{Product, Inventory, InventoryMovement, MovementType};
use delpopolo_infrastructure::repositories::InventoryRepository;

pub struct InventoryService {
    inventory_repo: InventoryRepository,
}

impl InventoryService {
    pub fn new(inventory_repo: InventoryRepository) -> Self {
        Self { inventory_repo }
    }
    
    /// Adiciona quantidade ao estoque
    pub async fn add_stock(
        &self,
        product_id: Uuid,
        quantity: f64,
        unit_cost: Option<f64>,
        nfe_key: Option<String>,
    ) -> Result<()> {
        info!("Adding {} units to product {}", quantity, product_id);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .unwrap_or_else(|| Inventory::new(product_id));
        
        inventory.add_quantity(quantity);
        
        let mut movement = InventoryMovement::new(
            product_id,
            MovementType::Purchase,
            quantity,
        );
        
        if let Some(cost) = unit_cost {
            movement = movement.with_cost(cost);
        }
        
        if let Some(key) = nfe_key {
            movement = movement.with_nfe(key);
        }
        
        self.inventory_repo.save(&inventory).await?;
        self.inventory_repo.save_movement(&movement).await?;
        
        Ok(())
    }
    
    /// Remove quantidade do estoque (venda)
    pub async fn remove_stock(
        &self,
        product_id: Uuid,
        quantity: f64,
        order_id: Option<Uuid>,
    ) -> Result<()> {
        info!("Removing {} units from product {}", quantity, product_id);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        inventory.remove_quantity(quantity)?;
        
        let mut movement = InventoryMovement::new(
            product_id,
            MovementType::Sale,
            quantity,
        );
        movement.order_id = order_id;
        
        self.inventory_repo.save(&inventory).await?;
        self.inventory_repo.save_movement(&movement).await?;
        
        Ok(())
    }
    
    /// Reserva quantidade para um pedido
    pub async fn reserve_stock(
        &self,
        product_id: Uuid,
        quantity: f64,
        order_id: Uuid,
    ) -> Result<()> {
        info!("Reserving {} units of product {} for order {}", 
            quantity, product_id, order_id);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        inventory.reserve(quantity)?;
        
        self.inventory_repo.save(&inventory).await?;
        
        Ok(())
    }
    
    /// Libera reserva de estoque
    pub async fn release_reservation(
        &self,
        product_id: Uuid,
        quantity: f64,
    ) -> Result<()> {
        info!("Releasing {} units reservation for product {}", quantity, product_id);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        inventory.release_reservation(quantity);
        
        self.inventory_repo.save(&inventory).await?;
        
        Ok(())
    }
    
    /// Ajuste manual de estoque
    pub async fn adjust_stock(
        &self,
        product_id: Uuid,
        new_quantity: f64,
        reason: String,
        performed_by: Uuid,
    ) -> Result<()> {
        warn!("Manual stock adjustment for product {}: reason={}", product_id, reason);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .unwrap_or_else(|| Inventory::new(product_id));
        
        let old_quantity = inventory.quantity;
        let difference = new_quantity - old_quantity;
        
        inventory.quantity = new_quantity;
        inventory.recalculate_available();
        inventory.last_movement_at = Some(chrono::Utc::now());
        inventory.updated_at = chrono::Utc::now();
        
        let mut movement = InventoryMovement::new(
            product_id,
            MovementType::Adjustment,
            difference.abs(),
        );
        movement.notes = Some(reason);
        movement.performed_by = Some(performed_by);
        
        self.inventory_repo.save(&inventory).await?;
        self.inventory_repo.save_movement(&movement).await?;
        
        Ok(())
    }
    
    /// Registra perda/quebra de estoque
    pub async fn register_loss(
        &self,
        product_id: Uuid,
        quantity: f64,
        reason: String,
        performed_by: Uuid,
    ) -> Result<()> {
        warn!("Stock loss registered for product {}: {} units - {}", 
            product_id, quantity, reason);
        
        let mut inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        inventory.remove_quantity(quantity)?;
        
        let mut movement = InventoryMovement::new(
            product_id,
            MovementType::Loss,
            quantity,
        );
        movement.notes = Some(reason);
        movement.performed_by = Some(performed_by);
        
        self.inventory_repo.save(&inventory).await?;
        self.inventory_repo.save_movement(&movement).await?;
        
        Ok(())
    }
    
    /// Verifica se há estoque disponível
    pub async fn check_availability(
        &self,
        product_id: Uuid,
        required_quantity: f64,
    ) -> Result<bool> {
        let inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        Ok(inventory.available_quantity >= required_quantity)
    }
    
    /// Obtém quantidade disponível
    pub async fn get_available_quantity(&self, product_id: Uuid) -> Result<f64> {
        let inventory = self.inventory_repo
            .find_by_product_id(product_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Inventory not found"))?;
        
        Ok(inventory.available_quantity)
    }
}
