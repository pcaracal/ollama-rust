use serde::Serialize;

/// Specifies how long the model will stay loaded in memory following this request.
/// Ollama unloads models after 5 minutes of inactivity by default.
#[derive(Debug, Clone)]
pub enum KeepAlive {
    /// Model will stay loaded indefinitely.
    Forever,

    /// Model will be unloaded after the request is complete.
    UntilCompletion,

    /// Duration with unit: "10s", "5m", "1h"
    Custom(String),
}

impl Serialize for KeepAlive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            KeepAlive::Forever => serializer.serialize_i8(-1),
            KeepAlive::UntilCompletion => serializer.serialize_i8(0),
            KeepAlive::Custom(duration) => serializer.serialize_str(duration),
        }
    }
}

/// Specifies the level of "thinking" or reasoning a model should apply before responding.
/// Can be a boolean (Enabled/Disabled) or a string ("high", "medium", "low") for supported models.
#[derive(Debug, Clone)]
pub enum Think {
    Enabled,
    Disabled,
    High,
    Medium,
    Low,
}

impl Serialize for Think {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Think::Enabled => serializer.serialize_bool(true),
            Think::Disabled => serializer.serialize_bool(false),
            Think::High => serializer.serialize_str("high"),
            Think::Medium => serializer.serialize_str("medium"),
            Think::Low => serializer.serialize_str("low"),
        }
    }
}
