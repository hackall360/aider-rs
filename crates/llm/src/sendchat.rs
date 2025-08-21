use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Check that messages alternate between user and assistant and the last
/// non-system message is from the user.
pub fn sanity_check_messages(messages: &[Message]) -> Result<(), String> {
    let mut last_role: Option<&str> = None;
    let mut last_non_system: Option<&str> = None;

    for msg in messages {
        let role = msg.role.as_str();
        if role == "system" {
            continue;
        }
        if let Some(prev) = last_role {
            if role == prev {
                return Err("Messages don't properly alternate user/assistant".into());
            }
        }
        last_role = Some(role);
        last_non_system = Some(role);
    }

    match last_non_system {
        Some("user") => Ok(()),
        _ => Err("Last non-system message must be from user".into()),
    }
}

/// Ensure messages alternate between assistant and user roles by inserting
/// empty messages of the opposite role when needed.
pub fn ensure_alternating_roles(messages: &[Message]) -> Vec<Message> {
    if messages.is_empty() {
        return Vec::new();
    }
    let mut fixed = Vec::new();
    let mut prev_role: Option<&str> = None;

    for msg in messages {
        let current = msg.role.as_str();
        if let Some(prev) = prev_role {
            if current == prev {
                let opposite = if current == "user" {
                    "assistant"
                } else {
                    "user"
                };
                fixed.push(Message {
                    role: opposite.to_string(),
                    content: String::new(),
                });
            }
        }
        fixed.push(msg.clone());
        prev_role = Some(current);
    }

    fixed
}
