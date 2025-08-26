//! Intent routing and matching
//! 
//! Provides regex-based intent matching and plugin routing

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};
use zenterm_plugins_api::{IntentContext, IntentHandler, IntentParams};

/// Intent matching pattern with scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentPattern {
    /// Pattern name/identifier
    pub name: String,
    /// Regex pattern for matching
    pub pattern: String,
    /// Base score for this pattern (0.0-1.0)
    pub score: f64,
    /// Required capability for this pattern
    pub capability: String,
    /// Optional context requirements
    pub context_requirements: HashMap<String, String>,
}

impl IntentPattern {
    /// Create a new intent pattern
    pub fn new(name: String, pattern: String, score: f64, capability: String) -> Self {
        Self {
            name,
            pattern,
            score,
            capability,
            context_requirements: HashMap::new(),
        }
    }

    /// Add a context requirement
    pub fn with_context_requirement(mut self, key: String, value: String) -> Self {
        self.context_requirements.insert(key, value);
        self
    }

    /// Check if input matches this pattern
    pub fn matches(&self, input: &str) -> Result<Option<f64>> {
        let regex = Regex::new(&self.pattern)
            .with_context(|| format!("Invalid regex pattern: {}", self.pattern))?;
        
        if regex.is_match(input) {
            // Basic scoring - could be enhanced with fuzzy matching, etc.
            Ok(Some(self.score))
        } else {
            Ok(None)
        }
    }
}

/// Intent match result
#[derive(Debug, Clone)]
pub struct IntentMatch {
    /// The pattern that matched
    pub pattern: IntentPattern,
    /// Match score (0.0-1.0)
    pub score: f64,
    /// Captured groups from regex
    pub captures: Vec<String>,
}

/// Intent router manages pattern matching and plugin routing
pub struct IntentRouter {
    /// Registered intent patterns
    patterns: Vec<IntentPattern>,
    /// Registered intent handlers (capability -> handler)
    handlers: HashMap<String, Box<dyn IntentHandler>>,
}

impl IntentRouter {
    /// Create a new intent router
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            handlers: HashMap::new(),
        }
    }

    /// Register an intent pattern
    pub fn register_pattern(&mut self, pattern: IntentPattern) {
        self.patterns.push(pattern);
        debug!("Registered intent pattern: {}", self.patterns.last().unwrap().name);
    }

    /// Register an intent handler
    pub fn register_handler(&mut self, capability: String, handler: Box<dyn IntentHandler>) {
        debug!("Registered intent handler for capability: {}", capability);
        self.handlers.insert(capability, handler);
    }

    /// Match input against all patterns and return best matches
    pub fn match_intent(&self, input: &str) -> Result<Vec<IntentMatch>> {
        let mut matches = Vec::new();

        for pattern in &self.patterns {
            if let Some(score) = pattern.matches(input)? {
                // Extract captures using regex
                let regex = Regex::new(&pattern.pattern)?;
                let captures = if let Some(cap) = regex.captures(input) {
                    cap.iter()
                        .skip(1) // Skip the full match
                        .filter_map(|m| m.map(|m| m.as_str().to_string()))
                        .collect()
                } else {
                    Vec::new()
                };

                matches.push(IntentMatch {
                    pattern: pattern.clone(),
                    score,
                    captures,
                });
            }
        }

        // Sort by score (highest first)
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(matches)
    }

    /// Execute the best matching intent
    pub fn execute_intent(&self, input: &str, context: &IntentContext) -> Result<String> {
        let matches = self.match_intent(input)?;
        
        if matches.is_empty() {
            return Ok(format!("No matching intent found for: {}", input));
        }

        let best_match = &matches[0];
        info!("Executing intent: {} (score: {})", best_match.pattern.name, best_match.score);

        // Find handler for the capability
        if let Some(handler) = self.handlers.get(&best_match.pattern.capability) {
            let params = IntentParams {
                raw_input: input.to_string(),
                parsed_args: best_match.captures.clone(),
            };

            handler.execute(context, &params)
        } else {
            Ok(format!("No handler registered for capability: {}", best_match.pattern.capability))
        }
    }

    /// Get all registered patterns
    pub fn patterns(&self) -> &[IntentPattern] {
        &self.patterns
    }

    /// Get registered handler capabilities
    pub fn handler_capabilities(&self) -> Vec<&String> {
        self.handlers.keys().collect()
    }

    /// Initialize with default patterns
    pub fn with_default_patterns(mut self) -> Self {
        // Git operations
        self.register_pattern(IntentPattern::new(
            "git_status".to_string(),
            r"(?i)^(git\s+status|show\s+git\s+status|what\s+is\s+git\s+status)".to_string(),
            0.9,
            "git_helper".to_string(),
        ));

        self.register_pattern(IntentPattern::new(
            "git_add".to_string(),
            r"(?i)^(git\s+add|stage\s+files?)\s+(.+)".to_string(),
            0.8,
            "git_helper".to_string(),
        ));

        self.register_pattern(IntentPattern::new(
            "git_commit".to_string(),
            r"(?i)^(git\s+commit|commit\s+changes)\s+(.+)".to_string(),
            0.8,
            "git_helper".to_string(),
        ));

        // File operations
        self.register_pattern(IntentPattern::new(
            "list_files".to_string(),
            r"(?i)^(ls|list\s+files?|show\s+files?)\s*(.*)".to_string(),
            0.7,
            "file_ops".to_string(),
        ));

        self.register_pattern(IntentPattern::new(
            "change_directory".to_string(),
            r"(?i)^(cd|change\s+dir|go\s+to)\s+(.+)".to_string(),
            0.8,
            "file_ops".to_string(),
        ));

        // Theme operations
        self.register_pattern(IntentPattern::new(
            "generate_theme".to_string(),
            r"(?i)^(generate\s+theme|create\s+theme)\s+(.+)".to_string(),
            0.8,
            "theming".to_string(),
        ));

        self.register_pattern(IntentPattern::new(
            "list_themes".to_string(),
            r"(?i)^(list\s+themes?|show\s+themes?)".to_string(),
            0.9,
            "theming".to_string(),
        ));

        // System operations
        self.register_pattern(IntentPattern::new(
            "elevate_request".to_string(),
            r"(?i)^(sudo|elevate|get\s+admin|become\s+root)".to_string(),
            0.9,
            "elevation".to_string(),
        ));

        self.register_pattern(IntentPattern::new(
            "show_processes".to_string(),
            r"(?i)^(ps|show\s+processes?|list\s+processes?)".to_string(),
            0.8,
            "system_info".to_string(),
        ));

        self
    }
}

impl Default for IntentRouter {
    fn default() -> Self {
        Self::new().with_default_patterns()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_pattern_matching() {
        let pattern = IntentPattern::new(
            "git_status".to_string(),
            r"(?i)^git\s+status".to_string(),
            0.9,
            "git_helper".to_string(),
        );

        assert!(pattern.matches("git status").unwrap().is_some());
        assert!(pattern.matches("GIT STATUS").unwrap().is_some());
        assert!(pattern.matches("git commit").unwrap().is_none());
    }

    #[test]
    fn test_intent_router_matching() {
        let router = IntentRouter::default();
        
        let matches = router.match_intent("git status").unwrap();
        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern.name, "git_status");
        
        let matches = router.match_intent("list files").unwrap();
        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern.name, "list_files");
    }

    #[test]
    fn test_pattern_with_captures() {
        let pattern = IntentPattern::new(
            "git_add".to_string(),
            r"(?i)^git\s+add\s+(.+)".to_string(),
            0.8,
            "git_helper".to_string(),
        );

        let _router = IntentRouter::new();
        let regex = Regex::new(&pattern.pattern).unwrap();
        
        if let Some(captures) = regex.captures("git add file.txt") {
            let captured: Vec<String> = captures.iter()
                .skip(1)
                .filter_map(|m| m.map(|m| m.as_str().to_string()))
                .collect();
            assert_eq!(captured, vec!["file.txt"]);
        }
    }

    #[test]
    fn test_scoring_order() {
        let mut router = IntentRouter::new();
        
        // Add patterns with different scores
        router.register_pattern(IntentPattern::new(
            "low_score".to_string(),
            r"test".to_string(),
            0.3,
            "test".to_string(),
        ));
        
        router.register_pattern(IntentPattern::new(
            "high_score".to_string(),
            r"test".to_string(),
            0.9,
            "test".to_string(),
        ));

        let matches = router.match_intent("test").unwrap();
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].pattern.name, "high_score"); // Should be first due to higher score
        assert_eq!(matches[1].pattern.name, "low_score");
    }
}