//! ZenTerm Plugins API
//! 
//! Foundational traits and types for the ZenTerm plugin system.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Capability metadata for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Capability {
    pub fn new(name: impl Into<String>, version: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
        }
    }
}

/// Intent handler context
#[derive(Debug, Clone)]
pub struct IntentContext {
    pub session_id: String,
    pub user_id: Option<String>,
}

/// Intent execution parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentParams {
    pub raw_input: String,
    pub parsed_args: Vec<String>,
}

/// Intent handler trait for processing user commands
pub trait IntentHandler: Send + Sync {
    /// Returns a score (0.0-1.0) indicating how well this handler matches the intent
    fn match_intent(&self, input: &str) -> f64;
    
    /// Execute the intent with given context and parameters
    fn execute(&self, ctx: &IntentContext, params: &IntentParams) -> Result<String>;
    
    /// Get the capability metadata for this handler
    fn capability(&self) -> &Capability;
}

/// Theme specification for rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSpec {
    pub name: String,
    pub palette: Vec<String>, // Hex color codes
    pub variants: Vec<String>, // light, dark, etc.
}

/// Theming renderer trait for generating theme assets
pub trait ThemingRenderer: Send + Sync {
    /// Get the unique identifier for this renderer
    fn id(&self) -> &'static str;
    
    /// Render theme specification to concrete assets
    fn render(&self, theme_spec: &ThemeSpec) -> Result<Vec<u8>>;
    
    /// Get supported output formats (e.g., "gtk-css", "kde-colors")
    fn supported_formats(&self) -> Vec<&'static str>;
}