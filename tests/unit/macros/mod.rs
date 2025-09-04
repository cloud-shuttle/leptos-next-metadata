//! Macro testing module
//! 
//! This module contains comprehensive tests for the metadata! and generate_metadata! macros,
//! covering compilation, error handling, integration, and real-world usage scenarios.

mod macro_compilation_tests;
mod macro_error_handling_tests;
mod macro_integration_tests;
mod macro_performance_tests;

// Re-export all test modules for easy access
pub use macro_compilation_tests::*;
pub use macro_error_handling_tests::*;
pub use macro_integration_tests::*;
pub use macro_performance_tests::*;
