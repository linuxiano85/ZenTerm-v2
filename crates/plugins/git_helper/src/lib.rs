//! Git Helper Plugin
//! 
//! Example plugin implementation for Git VCS integration.

use anyhow::Result;
use zenterm_plugins_api::{Capability, IntentContext, IntentHandler, IntentParams};

pub struct GitHelperPlugin {
    capability: Capability,
}

impl GitHelperPlugin {
    pub fn new() -> Self {
        Self {
            capability: Capability::new(
                "git-helper",
                "0.1.0",
                "Git repository management and automation"
            ),
        }
    }
}

impl IntentHandler for GitHelperPlugin {
    fn match_intent(&self, input: &str) -> f64 {
        if input.starts_with("git ") || input.contains("commit") || input.contains("branch") {
            0.8
        } else {
            0.0
        }
    }

    fn execute(&self, _ctx: &IntentContext, params: &IntentParams) -> Result<String> {
        // TODO: Implement actual git operations
        Ok(format!("Git command executed: {}", params.raw_input))
    }

    fn capability(&self) -> &Capability {
        &self.capability
    }
}