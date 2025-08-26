//! Session management
//! 
//! Provides session tracking, context management, and state persistence

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use parking_lot::RwLock;
use dashmap::DashMap;

/// Represents a ZenTerm session with context and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier
    pub id: Uuid,
    /// Session creation timestamp
    pub created_at: u64,
    /// Last activity timestamp  
    pub last_activity: u64,
    /// Session context (environment variables, working directory, etc.)
    pub context: HashMap<String, String>,
    /// Session state (persistent data between commands)
    pub state: HashMap<String, serde_json::Value>,
    /// Whether session has elevated privileges
    pub elevated: bool,
    /// Elevation expiry timestamp (if elevated)
    pub elevation_expires: Option<u64>,
}

impl Session {
    /// Create a new session
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            created_at: now,
            last_activity: now,
            context: HashMap::new(),
            state: HashMap::new(),
            elevated: false,
            elevation_expires: None,
        }
    }

    /// Create a session with a specific ID
    pub fn with_id(id: Uuid) -> Self {
        let mut session = Self::new();
        session.id = id;
        session
    }

    /// Update last activity timestamp
    pub fn touch(&mut self) {
        self.last_activity = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Set a context value
    pub fn set_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
        self.touch();
    }

    /// Get a context value
    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }

    /// Set session state
    pub fn set_state<T: Serialize>(&mut self, key: String, value: T) -> Result<()> {
        let json_value = serde_json::to_value(value)
            .context("Failed to serialize state value")?;
        self.state.insert(key, json_value);
        self.touch();
        Ok(())
    }

    /// Get session state
    pub fn get_state<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        match self.state.get(key) {
            Some(value) => {
                let deserialized = serde_json::from_value(value.clone())
                    .context("Failed to deserialize state value")?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    /// Grant elevated privileges with timeout
    pub fn elevate(&mut self, timeout_minutes: u32) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.elevated = true;
        self.elevation_expires = Some(now + (timeout_minutes as u64 * 60));
        self.touch();
    }

    /// Revoke elevated privileges
    pub fn revoke_elevation(&mut self) {
        self.elevated = false;
        self.elevation_expires = None;
        self.touch();
    }

    /// Check if session has valid elevation
    pub fn is_elevated(&self) -> bool {
        if !self.elevated {
            return false;
        }

        if let Some(expires) = self.elevation_expires {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            return now < expires;
        }

        false
    }

    /// Get remaining elevation time in seconds
    pub fn elevation_remaining_seconds(&self) -> Option<u64> {
        if let Some(expires) = self.elevation_expires {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            if now < expires {
                return Some(expires - now);
            }
        }
        None
    }

    /// Check if session is expired based on inactivity
    pub fn is_expired(&self, max_idle_minutes: u32) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let max_idle_seconds = max_idle_minutes as u64 * 60;
        (now - self.last_activity) > max_idle_seconds
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Session manager handles multiple concurrent sessions
pub struct SessionManager {
    /// Active sessions indexed by ID
    sessions: DashMap<Uuid, RwLock<Session>>,
    /// Default session timeout in minutes
    default_timeout: u32,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(default_timeout_minutes: u32) -> Self {
        Self {
            sessions: DashMap::new(),
            default_timeout: default_timeout_minutes,
        }
    }

    /// Create a new session
    pub fn create_session(&self) -> Uuid {
        let session = Session::new();
        let session_id = session.id;
        self.sessions.insert(session_id, RwLock::new(session));
        session_id
    }

    /// Get a session by ID
    pub fn get_session(&self, session_id: &Uuid) -> Option<dashmap::mapref::one::Ref<'_, Uuid, RwLock<Session>>> {
        self.sessions.get(session_id)
    }

    /// Remove a session
    pub fn remove_session(&self, session_id: &Uuid) -> Option<Session> {
        self.sessions.remove(session_id).map(|(_, session)| {
            session.into_inner()
        })
    }

    /// Get all active session IDs
    pub fn active_sessions(&self) -> Vec<Uuid> {
        self.sessions.iter().map(|entry| *entry.key()).collect()
    }

    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&self) -> usize {
        let expired_sessions: Vec<Uuid> = self.sessions
            .iter()
            .filter_map(|entry| {
                let session = entry.value().read();
                if session.is_expired(self.default_timeout) {
                    Some(*entry.key())
                } else {
                    None
                }
            })
            .collect();

        let count = expired_sessions.len();
        for session_id in expired_sessions {
            self.sessions.remove(&session_id);
        }
        count
    }

    /// Clean up expired elevation for all sessions
    pub fn cleanup_expired_elevations(&self) -> usize {
        let mut count = 0;
        
        for entry in self.sessions.iter() {
            let mut session = entry.value().write();
            if session.elevated && !session.is_elevated() {
                session.revoke_elevation();
                count += 1;
            }
        }
        
        count
    }

    /// Get session count
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Update session activity
    pub fn touch_session(&self, session_id: &Uuid) -> bool {
        if let Some(session_ref) = self.sessions.get(session_id) {
            session_ref.write().touch();
            true
        } else {
            false
        }
    }

    /// Set session context
    pub fn set_session_context(&self, session_id: &Uuid, key: String, value: String) -> bool {
        if let Some(session_ref) = self.sessions.get(session_id) {
            session_ref.write().set_context(key, value);
            true
        } else {
            false
        }
    }

    /// Get session context
    pub fn get_session_context(&self, session_id: &Uuid, key: &str) -> Option<String> {
        if let Some(session_ref) = self.sessions.get(session_id) {
            session_ref.read().get_context(key).cloned()
        } else {
            None
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(30) // 30 minute default timeout
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert!(session.context.is_empty());
        assert!(session.state.is_empty());
        assert!(!session.elevated);
        assert!(session.elevation_expires.is_none());
    }

    #[test]
    fn test_session_context() {
        let mut session = Session::new();
        session.set_context("working_dir".to_string(), "/tmp".to_string());
        
        assert_eq!(session.get_context("working_dir"), Some(&"/tmp".to_string()));
        assert_eq!(session.get_context("nonexistent"), None);
    }

    #[test]
    fn test_session_state() {
        let mut session = Session::new();
        
        // Test string state
        session.set_state("last_command".to_string(), "ls -la".to_string()).unwrap();
        let last_cmd: Option<String> = session.get_state("last_command").unwrap();
        assert_eq!(last_cmd, Some("ls -la".to_string()));
        
        // Test number state
        session.set_state("command_count".to_string(), 42u32).unwrap();
        let count: Option<u32> = session.get_state("command_count").unwrap();
        assert_eq!(count, Some(42));
    }

    #[test]
    fn test_session_elevation() {
        let mut session = Session::new();
        
        // Not elevated initially
        assert!(!session.is_elevated());
        
        // Elevate for 1 minute
        session.elevate(1);
        assert!(session.is_elevated());
        assert!(session.elevation_remaining_seconds().is_some());
        
        // Revoke elevation
        session.revoke_elevation();
        assert!(!session.is_elevated());
        assert!(session.elevation_remaining_seconds().is_none());
    }

    #[test]
    fn test_session_manager() {
        let manager = SessionManager::new(1); // 1 minute timeout
        
        // Create sessions
        let session1 = manager.create_session();
        let session2 = manager.create_session();
        
        assert_eq!(manager.session_count(), 2);
        assert!(manager.active_sessions().contains(&session1));
        assert!(manager.active_sessions().contains(&session2));
        
        // Test context setting
        assert!(manager.set_session_context(&session1, "test".to_string(), "value".to_string()));
        assert_eq!(manager.get_session_context(&session1, "test"), Some("value".to_string()));
        
        // Remove session
        assert!(manager.remove_session(&session1).is_some());
        assert_eq!(manager.session_count(), 1);
        assert!(!manager.active_sessions().contains(&session1));
    }

    #[test]
    fn test_session_expiry() {
        let mut session = Session::new();
        
        // Should not be expired with reasonable timeout
        assert!(!session.is_expired(30));
        
        // Manually set old last_activity to test expiry
        session.last_activity = 0; // Set to epoch (very old)
        
        // Should be expired with any reasonable timeout when last_activity is very old
        assert!(session.is_expired(1));
    }
}