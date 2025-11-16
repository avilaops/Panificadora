pub mod intent;
pub mod nlp;
pub mod context;
pub mod responses;
pub mod deepseek;

pub use intent::{Intent, IntentClassifier};
pub use nlp::NLPEngine;
pub use context::{ConversationContext, ContextManager};
pub use responses::ResponseGenerator;
pub use deepseek::DeepSeekClient;
