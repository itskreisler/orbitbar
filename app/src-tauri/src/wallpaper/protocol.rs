use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static MSG_COUNTER: AtomicU64 = AtomicU64::new(1);

fn next_msg_id() -> String {
    let count = MSG_COUNTER.fetch_add(1, Ordering::Relaxed);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{now}-{count}")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Event,
    Request,
    Response,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub version: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_: MessageType,
    pub name: String,
    pub payload: serde_json::Value,
}

impl IpcMessage {
    pub const PROTOCOL_VERSION: &'static str = "1.0";

    pub fn new_event(name: &str, payload: serde_json::Value) -> Self {
        Self {
            version: Self::PROTOCOL_VERSION.to_string(),
            id: next_msg_id(),
            type_: MessageType::Event,
            name: name.to_string(),
            payload,
        }
    }

    pub fn new_request(name: &str, payload: serde_json::Value) -> Self {
        Self {
            version: Self::PROTOCOL_VERSION.to_string(),
            id: next_msg_id(),
            type_: MessageType::Request,
            name: name.to_string(),
            payload,
        }
    }

    pub fn new_response(correlation_id: &str, name: &str, payload: serde_json::Value) -> Self {
        Self {
            version: Self::PROTOCOL_VERSION.to_string(),
            id: correlation_id.to_string(),
            type_: MessageType::Response,
            name: name.to_string(),
            payload,
        }
    }

    pub fn new_error(correlation_id: &str, name: &str, error_message: &str) -> Self {
        Self {
            version: Self::PROTOCOL_VERSION.to_string(),
            id: correlation_id.to_string(),
            type_: MessageType::Error,
            name: name.to_string(),
            payload: serde_json::json!({ "error": error_message }),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version.trim().is_empty() {
            return Err("Protocol message 'version' missing".to_string());
        }
        if self.id.trim().is_empty() {
            return Err("Protocol message 'id' missing".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("Protocol message 'name' missing".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_ipc_message() {
        let msg = IpcMessage::new_event("property_update", serde_json::json!({"color": "#ff0000"}));
        assert_eq!(msg.version, "1.0");
        assert_eq!(msg.type_, MessageType::Event);
        assert!(msg.validate().is_ok());
    }

    #[test]
    fn creates_response_with_correlation_id() {
        let req_id = "request-123";
        let resp = IpcMessage::new_response(req_id, "get_info", serde_json::json!({"fps": 60}));
        assert_eq!(resp.id, req_id);
        assert_eq!(resp.type_, MessageType::Response);
    }
}
