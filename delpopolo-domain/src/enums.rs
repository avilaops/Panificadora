use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductCategory {
    Bread,
    Cake,
    Cookie,
    Pastry,
    Beverage,
    Sandwich,
    Snack,
    RawMaterial,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitOfMeasure {
    Unit,      // unidade
    Kilogram,  // kg
    Gram,      // g
    Liter,     // l
    Milliliter, // ml
    Dozen,     // dúzia
    Package,   // pacote
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Preparing,
    Ready,
    InDelivery,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSource {
    InStore,
    IFood,
    WhatsApp,
    Web,
    Phone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    Cash,
    DebitCard,
    CreditCard,
    Pix,
    VoucherMeal,
    VoucherFood,
    Multiple, // Pagamento misto
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Approved,
    Rejected,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MovementType {
    Purchase,      // Compra de fornecedor
    Sale,          // Venda para cliente
    Adjustment,    // Ajuste de inventário
    Loss,          // Perda/quebra
    Return,        // Devolução
    Transfer,      // Transferência entre estoques
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignType {
    Promotional,   // Promoção
    Seasonal,      // Sazonal (dia da pizza, natal, etc)
    FreshBread,    // Pão quentinho
    Birthday,      // Aniversário
    Welcome,       // Boas vindas
    Reactivation,  // Reativação de cliente inativo
    ThankYou,      // Agradecimento
    Newsletter,    // Newsletter
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Active,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignChannel {
    Email,
    WhatsApp,
    Push,
    SMS,
    IFood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    LowStock,
    OrderReceived,
    OrderReady,
    OrderDelivered,
    PaymentReceived,
    CampaignMessage,
    SystemAlert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    Push,
    SMS,
    WhatsApp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Manager,
    Cashier,
    InventoryManager,
    Kitchen,
    Delivery,
}
