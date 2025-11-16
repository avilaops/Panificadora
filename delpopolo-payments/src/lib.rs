// Módulo de integração com gateways de pagamento

pub mod stone;
pub mod pix;
pub mod service;

pub use stone::StoneClient;
pub use pix::PixService;
pub use service::PaymentService;
