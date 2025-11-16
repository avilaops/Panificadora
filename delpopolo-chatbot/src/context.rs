use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub user_id: String,
    pub session_id: String,
    pub started_at: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
    pub state: ConversationState,
    pub variables: HashMap<String, String>,
    pub message_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConversationState {
    Initial,
    AwaitingUserInfo,
    BrowsingMenu,
    BuildingOrder,
    ConfirmingOrder,
    TrackingOrder,
    ProvidingSupport,
    Completed,
}

impl ConversationContext {
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        Self {
            user_id: user_id.clone(),
            session_id: format!("{}_{}", user_id, now.timestamp()),
            started_at: now,
            last_interaction: now,
            state: ConversationState::Initial,
            variables: HashMap::new(),
            message_count: 0,
        }
    }
    
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }
    
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
    
    pub fn update_interaction(&mut self) {
        self.last_interaction = Utc::now();
        self.message_count += 1;
    }
    
    pub fn transition_to(&mut self, state: ConversationState) {
        self.state = state;
        self.update_interaction();
    }
    
    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let now = Utc::now();
        (now - self.last_interaction).num_minutes() > timeout_minutes
    }
}

pub struct ContextManager {
    contexts: HashMap<String, ConversationContext>,
    timeout_minutes: i64,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
            timeout_minutes: 30,
        }
    }
    
    pub fn get_or_create(&mut self, user_id: &str) -> &mut ConversationContext {
        self.cleanup_expired();
        
        self.contexts
            .entry(user_id.to_string())
            .or_insert_with(|| ConversationContext::new(user_id.to_string()))
    }
    
    pub fn get(&self, user_id: &str) -> Option<&ConversationContext> {
        self.contexts.get(user_id)
    }
    
    pub fn remove(&mut self, user_id: &str) {
        self.contexts.remove(user_id);
    }
    
    fn cleanup_expired(&mut self) {
        let timeout = self.timeout_minutes;
        self.contexts.retain(|_, ctx| !ctx.is_expired(timeout));
    }
    
    pub fn active_conversations(&self) -> usize {
        self.contexts.len()
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}
