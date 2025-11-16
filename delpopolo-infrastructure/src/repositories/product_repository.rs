use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use delpopolo_domain::{Product, ProductCategory, UnitOfMeasure, Money};
use delpopolo_core::traits::Repository;

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    pub async fn find_low_stock_products(&self) -> Result<Vec<Product>> {
        let products = sqlx::query_as!(
            ProductRow,
            r#"
            SELECT 
                id, name, description, sku, barcode,
                category as "category: ProductCategory",
                unit_of_measure as "unit_of_measure: UnitOfMeasure",
                price_amount, price_currency,
                cost_amount, cost_currency,
                stock_quantity, min_stock_level, max_stock_level,
                is_active, is_available_online,
                image_url, weight, preparation_time_minutes,
                supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                created_at, updated_at
            FROM products
            WHERE stock_quantity <= min_stock_level AND is_active = true
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(products.into_iter().map(|row| row.into()).collect())
    }
    
    pub async fn find_by_barcode(&self, barcode: &str) -> Result<Option<Product>> {
        let product = sqlx::query_as!(
            ProductRow,
            r#"
            SELECT 
                id, name, description, sku, barcode,
                category as "category: ProductCategory",
                unit_of_measure as "unit_of_measure: UnitOfMeasure",
                price_amount, price_currency,
                cost_amount, cost_currency,
                stock_quantity, min_stock_level, max_stock_level,
                is_active, is_available_online,
                image_url, weight, preparation_time_minutes,
                supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                created_at, updated_at
            FROM products
            WHERE barcode = $1
            "#,
            barcode
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(product.map(|row| row.into()))
    }
    
    pub async fn search(&self, query: &str, category: Option<ProductCategory>) -> Result<Vec<Product>> {
        let products = if let Some(cat) = category {
            sqlx::query_as!(
                ProductRow,
                r#"
                SELECT 
                    id, name, description, sku, barcode,
                    category as "category: ProductCategory",
                    unit_of_measure as "unit_of_measure: UnitOfMeasure",
                    price_amount, price_currency,
                    cost_amount, cost_currency,
                    stock_quantity, min_stock_level, max_stock_level,
                    is_active, is_available_online,
                    image_url, weight, preparation_time_minutes,
                    supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                    created_at, updated_at
                FROM products
                WHERE (name ILIKE $1 OR sku ILIKE $1) 
                  AND category = $2
                  AND is_active = true
                ORDER BY name
                "#,
                format!("%{}%", query),
                cat as ProductCategory
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                ProductRow,
                r#"
                SELECT 
                    id, name, description, sku, barcode,
                    category as "category: ProductCategory",
                    unit_of_measure as "unit_of_measure: UnitOfMeasure",
                    price_amount, price_currency,
                    cost_amount, cost_currency,
                    stock_quantity, min_stock_level, max_stock_level,
                    is_active, is_available_online,
                    image_url, weight, preparation_time_minutes,
                    supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                    created_at, updated_at
                FROM products
                WHERE (name ILIKE $1 OR sku ILIKE $1)
                  AND is_active = true
                ORDER BY name
                "#,
                format!("%{}%", query)
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        Ok(products.into_iter().map(|row| row.into()).collect())
    }
}

#[async_trait]
impl Repository<Product> for ProductRepository {
    async fn find_by_id(&self, id: Uuid) -> Option<Product> {
        sqlx::query_as!(
            ProductRow,
            r#"
            SELECT 
                id, name, description, sku, barcode,
                category as "category: ProductCategory",
                unit_of_measure as "unit_of_measure: UnitOfMeasure",
                price_amount, price_currency,
                cost_amount, cost_currency,
                stock_quantity, min_stock_level, max_stock_level,
                is_active, is_available_online,
                image_url, weight, preparation_time_minutes,
                supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                created_at, updated_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .ok()?
        .map(|row| row.into())
    }
    
    async fn find_all(&self) -> Vec<Product> {
        sqlx::query_as!(
            ProductRow,
            r#"
            SELECT 
                id, name, description, sku, barcode,
                category as "category: ProductCategory",
                unit_of_measure as "unit_of_measure: UnitOfMeasure",
                price_amount, price_currency,
                cost_amount, cost_currency,
                stock_quantity, min_stock_level, max_stock_level,
                is_active, is_available_online,
                image_url, weight, preparation_time_minutes,
                supplier_id, nfe_ncm, nfe_cest, nfe_cfop,
                created_at, updated_at
            FROM products
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| row.into())
        .collect()
    }
    
    async fn save(&self, entity: &Product) -> Result<Product, Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            INSERT INTO products (
                id, name, description, sku, barcode, category, unit_of_measure,
                price_amount, price_currency, cost_amount, cost_currency,
                stock_quantity, min_stock_level, max_stock_level,
                is_active, is_available_online, image_url, weight,
                preparation_time_minutes, supplier_id,
                nfe_ncm, nfe_cest, nfe_cfop,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25
            )
            "#,
            entity.id,
            entity.name,
            entity.description,
            entity.sku,
            entity.barcode,
            entity.category as ProductCategory,
            entity.unit_of_measure as UnitOfMeasure,
            entity.price.amount,
            entity.price.currency,
            entity.cost.amount,
            entity.cost.currency,
            entity.stock_quantity,
            entity.min_stock_level,
            entity.max_stock_level,
            entity.is_active,
            entity.is_available_online,
            entity.image_url,
            entity.weight,
            entity.preparation_time_minutes,
            entity.supplier_id,
            entity.nfe_ncm,
            entity.nfe_cest,
            entity.nfe_cfop,
            entity.created_at,
            entity.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(entity.clone())
    }
    
    async fn update(&self, entity: &Product) -> Result<Product, Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE products SET
                name = $2, description = $3, sku = $4, barcode = $5,
                category = $6, unit_of_measure = $7,
                price_amount = $8, price_currency = $9,
                cost_amount = $10, cost_currency = $11,
                stock_quantity = $12, min_stock_level = $13, max_stock_level = $14,
                is_active = $15, is_available_online = $16,
                image_url = $17, weight = $18, preparation_time_minutes = $19,
                supplier_id = $20, nfe_ncm = $21, nfe_cest = $22, nfe_cfop = $23,
                updated_at = $24
            WHERE id = $1
            "#,
            entity.id,
            entity.name,
            entity.description,
            entity.sku,
            entity.barcode,
            entity.category as ProductCategory,
            entity.unit_of_measure as UnitOfMeasure,
            entity.price.amount,
            entity.price.currency,
            entity.cost.amount,
            entity.cost.currency,
            entity.stock_quantity,
            entity.min_stock_level,
            entity.max_stock_level,
            entity.is_active,
            entity.is_available_online,
            entity.image_url,
            entity.weight,
            entity.preparation_time_minutes,
            entity.supplier_id,
            entity.nfe_ncm,
            entity.nfe_cest,
            entity.nfe_cfop,
            entity.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(entity.clone())
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!("DELETE FROM products WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// Helper struct for sqlx mapping
#[allow(dead_code)]
struct ProductRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    sku: String,
    barcode: Option<String>,
    category: ProductCategory,
    unit_of_measure: UnitOfMeasure,
    price_amount: f64,
    price_currency: String,
    cost_amount: f64,
    cost_currency: String,
    stock_quantity: f64,
    min_stock_level: f64,
    max_stock_level: Option<f64>,
    is_active: bool,
    is_available_online: bool,
    image_url: Option<String>,
    weight: Option<f64>,
    preparation_time_minutes: Option<i32>,
    supplier_id: Option<Uuid>,
    nfe_ncm: Option<String>,
    nfe_cest: Option<String>,
    nfe_cfop: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ProductRow> for Product {
    fn from(row: ProductRow) -> Self {
        Product {
            id: row.id,
            name: row.name,
            description: row.description,
            sku: row.sku,
            barcode: row.barcode,
            category: row.category,
            unit_of_measure: row.unit_of_measure,
            price: Money::new(row.price_amount, row.price_currency),
            cost: Money::new(row.cost_amount, row.cost_currency),
            stock_quantity: row.stock_quantity,
            min_stock_level: row.min_stock_level,
            max_stock_level: row.max_stock_level,
            is_active: row.is_active,
            is_available_online: row.is_available_online,
            image_url: row.image_url,
            weight: row.weight,
            preparation_time_minutes: row.preparation_time_minutes,
            supplier_id: row.supplier_id,
            nfe_ncm: row.nfe_ncm,
            nfe_cest: row.nfe_cest,
            nfe_cfop: row.nfe_cfop,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
