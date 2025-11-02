use std::{collections::HashMap, pin::Pin};

pub trait Tool: Send + Sync {
    /// The information provided to the model when providing this tool
    /// Use `ToolFunction::new()` builder
    fn tool_function(&self) -> ToolFunction;

    /// Will be called when the model invokes this tool
    #[allow(clippy::missing_errors_doc)]
    fn execute(
        &self,
        arguments: ToolCallArguments,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + Sync + 'static>>;
}

impl std::fmt::Debug for dyn Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("ToolFunction", &self.tool_function())
            .finish()
    }
}

impl ToolFunction {
    /// Builder for creating a new tool function
    /// Use the `parameter()` method to add parameters
    pub fn new<S: Into<String>>(name: S, description: S) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: ToolFunctionParameters {
                p_type: "object".to_string(),
                properties: HashMap::new(),
                required: vec![],
            },
        }
    }

    /// Adds a parameter to the tool function
    #[must_use]
    pub fn parameter<S: Into<String>>(mut self, name: S, description: S, required: bool) -> Self {
        let name = name.into();

        self.parameters.properties.insert(
            name.clone(),
            ToolProperty {
                p_type: "string".to_string(),
                description: description.into(),
            },
        );

        if required {
            self.parameters.required.push(name);
        }

        self
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInfo {
    #[serde(rename = "type")]
    pub tool_type: ToolType,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

pub type ToolCallArguments = HashMap<String, String>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCallFunction {
    pub index: i32,
    pub name: String,
    pub arguments: ToolCallArguments,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: ToolFunctionParameters,
}

pub type ToolPropertiesMap = HashMap<String, ToolProperty>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolFunctionParameters {
    #[serde(rename = "type")]
    pub p_type: String,
    pub properties: ToolPropertiesMap,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolProperty {
    #[serde(rename = "type")]
    pub p_type: String,
    pub description: String,
}
