//! ZenTerm Core Engine
//!
//! Provides the foundational components for command orchestration,
//! task execution, and plugin management.

pub mod execution;
pub mod session;
pub mod task;
pub mod event; // NEW

pub use execution::*;
pub use session::*;
pub use task::*;
// Re-export convenient event prelude
pub use event::prelude::*;