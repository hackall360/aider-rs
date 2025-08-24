use serde::Serialize;
use serde_yaml::Value;
use std::collections::BTreeMap;

/// Replace sensitive API keys in the provided text with their last four characters.
pub fn scrub_sensitive_info(
    openai_api_key: Option<&str>,
    anthropic_api_key: Option<&str>,
    text: &str,
) -> String {
    let mut result = text.to_string();

    if let Some(key) = openai_api_key {
        if !key.is_empty() {
            let last4 = &key[key.len().saturating_sub(4)..];
            result = result.replace(key, &format!("...{}", last4));
        }
    }

    if let Some(key) = anthropic_api_key {
        if !key.is_empty() {
            let last4 = &key[key.len().saturating_sub(4)..];
            result = result.replace(key, &format!("...{}", last4));
        }
    }

    result
}

/// Format settings for display, similar to the original Python implementation.
///
/// This converts a serializable structure of arguments into a sorted list of
/// key/value pairs, scrubbing any sensitive API keys from the output.
pub fn format_settings<T>(
    args: &T,
    openai_api_key: Option<&str>,
    anthropic_api_key: Option<&str>,
) -> String
where
    T: Serialize,
{
    let mut output = String::from("Option settings:\n");
    let value = serde_yaml::to_value(args).unwrap_or(Value::Null);

    if let Value::Mapping(map) = value {
        let mut entries: BTreeMap<String, String> = BTreeMap::new();
        for (k, v) in map {
            if let (Some(key), Some(val)) = (k.as_str(), v.as_str()) {
                entries.insert(key.to_string(), val.to_string());
            } else if let Some(key) = k.as_str() {
                let val = serde_yaml::to_string(&v).unwrap_or_default();
                entries.insert(key.to_string(), val.trim().to_string());
            }
        }

        for (key, val) in entries {
            let val = scrub_sensitive_info(openai_api_key, anthropic_api_key, &val);
            output.push_str(&format!("  - {}: {}\n", key, val));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct Args {
        openai_api_key: Option<String>,
        anthropic_api_key: Option<String>,
        other: String,
    }

    #[test]
    fn scrub_replaces_keys() {
        let text = "key sk-abcdef and anth anth-1234";
        let cleaned = scrub_sensitive_info(Some("sk-abcdef"), Some("anth-1234"), text);
        assert!(!cleaned.contains("sk-abcdef"));
        assert!(!cleaned.contains("anth-1234"));
        assert!(cleaned.contains("...cdef"));
        assert!(cleaned.contains("...1234"));
    }

    #[test]
    fn format_outputs_settings() {
        let args = Args {
            openai_api_key: Some("sk-abcdef".into()),
            anthropic_api_key: None,
            other: "value".into(),
        };
        let formatted = format_settings(&args, args.openai_api_key.as_deref(), None);
        assert!(formatted.contains("Option settings:"));
        assert!(formatted.contains("other: value"));
        assert!(formatted.contains("openai_api_key: ...cdef"));
    }
}
