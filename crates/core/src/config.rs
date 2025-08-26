//! Configuration management
//! 
//! Provides layered configuration loading (default + user + workspace)

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use dirs::config_dir;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Voice input configuration
    pub voice: VoiceConfig,
    /// AI provider configuration
    pub ai: AiConfig,
    /// Plugin configuration
    pub plugins: PluginConfig,
    /// Theming configuration
    pub themes: ThemeConfig,
    /// Security and elevation settings
    pub security: SecurityConfig,
}

/// Voice input configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Enable voice input
    pub enabled: bool,
    /// Default to push-to-talk mode
    pub push_to_talk: bool,
    /// Wake word for continuous listening
    pub wake_word: String,
    /// Language for speech recognition
    pub language: String,
}

/// AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Speech-to-text provider
    pub stt_provider: String,
    /// Text-to-speech provider 
    pub tts_provider: String,
    /// LLM provider for intent processing
    pub llm_provider: String,
    /// API keys and endpoints
    pub providers: std::collections::HashMap<String, ProviderConfig>,
}

/// Individual provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// API endpoint URL
    pub endpoint: Option<String>,
    /// API key (will be stored securely)
    pub api_key: Option<String>,
    /// Provider-specific settings
    pub settings: std::collections::HashMap<String, String>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Enable plugin system
    pub enabled: bool,
    /// Plugin directories to scan
    pub plugin_dirs: Vec<PathBuf>,
    /// Enabled plugins
    pub enabled_plugins: Vec<String>,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Current theme name
    pub current_theme: String,
    /// Theme output directory
    pub output_dir: PathBuf,
    /// Enable auto-theme application
    pub auto_apply: bool,
}

/// Security and elevation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Default elevation timeout in minutes
    pub elevation_timeout: u32,
    /// Enable audit logging
    pub audit_enabled: bool,
    /// Audit log file path
    pub audit_log: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            voice: VoiceConfig {
                enabled: true,
                push_to_talk: true,
                wake_word: "zen".to_string(),
                language: "en-US".to_string(),
            },
            ai: AiConfig {
                stt_provider: "whisper".to_string(),
                tts_provider: "local".to_string(),
                llm_provider: "local".to_string(),
                providers: std::collections::HashMap::new(),
            },
            plugins: PluginConfig {
                enabled: true,
                plugin_dirs: vec![
                    PathBuf::from("~/.config/zenterm/plugins"),
                    PathBuf::from("/usr/share/zenterm/plugins"),
                ],
                enabled_plugins: vec!["git_helper".to_string()],
            },
            themes: ThemeConfig {
                current_theme: "default".to_string(),
                output_dir: PathBuf::from("~/.config/zenterm/themes"),
                auto_apply: false,
            },
            security: SecurityConfig {
                elevation_timeout: 5,
                audit_enabled: true,
                audit_log: PathBuf::from("~/.config/zenterm/audit.log"),
            },
        }
    }
}

/// Configuration manager handles loading and saving configuration
pub struct ConfigManager {
    config: Config,
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path)?;
        
        Ok(Self {
            config,
            config_path,
        })
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the configuration file path
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// Initialize default configuration file
    pub fn init_config() -> Result<PathBuf> {
        let config_path = Self::get_config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        // Don't overwrite existing config
        if config_path.exists() {
            return Ok(config_path);
        }

        // Write default configuration
        let default_config = Config::default();
        let config_content = toml::to_string_pretty(&default_config)
            .context("Failed to serialize default configuration")?;
        
        std::fs::write(&config_path, config_content)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(config_path)
    }

    /// Validate the configuration file
    pub fn validate_config(config_path: Option<&Path>) -> Result<()> {
        let path = match config_path {
            Some(p) => p.to_path_buf(),
            None => Self::get_config_path()?,
        };

        if !path.exists() {
            return Err(anyhow::anyhow!("Configuration file does not exist: {}", path.display()));
        }

        // Try to load and parse the configuration
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        
        let _config: Config = toml::from_str(&content)
            .with_context(|| format!("Invalid configuration format in: {}", path.display()))?;

        Ok(())
    }

    /// Get the configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        Ok(config_dir.join("zenterm").join("config.toml"))
    }

    /// Load configuration with layering support
    fn load_config(config_path: &Path) -> Result<Config> {
        // Start with default configuration
        let mut config = Config::default();

        // Try to load user configuration if it exists
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)
                .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
            
            config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;
        }

        // TODO: Load workspace-specific configuration if present
        // This would involve looking for .zenterm/config.toml in current directory and parents

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.voice.enabled);
        assert!(config.voice.push_to_talk);
        assert_eq!(config.voice.wake_word, "zen");
        assert_eq!(config.ai.stt_provider, "whisper");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        
        assert_eq!(config.voice.wake_word, parsed.voice.wake_word);
        assert_eq!(config.ai.stt_provider, parsed.ai.stt_provider);
    }

    #[test]
    fn test_config_validation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        // Test missing file
        assert!(ConfigManager::validate_config(Some(&config_path)).is_err());
        
        // Test valid config
        let config = Config::default();
        let content = toml::to_string(&config).unwrap();
        fs::write(&config_path, content).unwrap();
        assert!(ConfigManager::validate_config(Some(&config_path)).is_ok());
        
        // Test invalid config
        fs::write(&config_path, "invalid toml content [[[").unwrap();
        assert!(ConfigManager::validate_config(Some(&config_path)).is_err());
    }
}