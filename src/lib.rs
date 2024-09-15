//! # Example
//! ```
//! use regex;
//! let expr = "abc|(de|cd)+";
//! let line = "decddede";
//!
//! // Checks if the regex expression matches the line by DFS.
//! regex::is_match(expr, line, true);
//! ```
mod engine;
mod helper;

pub use engine::{is_match, print};
