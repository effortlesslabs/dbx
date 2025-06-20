//! Redis primitive data type adapters
//!
//! This module provides implementations for Redis primitive data types:
//! - String: Simple string values, numbers, and binary data
//! - List: Lists of strings
//! - Hash: Hash maps of string field-value pairs
//! - Set: Unordered collections of unique strings
//! - Sorted Set: Ordered collections of strings with associated scores
//!
//! Each implementation supports individual commands, pipelined operations,
//! transactions, and Lua scripts.

pub mod string;
pub mod set;
pub mod hash;

// These will be implemented in future versions:
// pub mod list;
// pub mod sorted_set;
