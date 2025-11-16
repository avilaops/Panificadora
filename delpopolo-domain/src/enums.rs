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
    Dozen,     // d�zia
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
    Adjustment,    // Ajuste de invent�rio
    Loss,          // Perda/quebra
    Return,        // Devolu��o
    Transfer,      // Transfer�ncia entre estoques
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CampaignType {
    Promotional,   // Promo��o
    Seasonal,      // Sazonal (dia da pizza, natal, etc)
    FreshBread,    // P�o quentinho
    Birthday,      // Anivers�rio
    Welcome,       // Boas vindas
    Reactivation,  // Reativa��o de cliente inativo
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
