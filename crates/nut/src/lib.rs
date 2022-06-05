pub mod config;
pub mod entities;
pub mod functions;
pub mod pyramid;
pub mod repositories;
pub mod services;

pub type Version = [u16; 3];

// Version decomposes as:
//
// 1. Major
// 2. Minor
// 3. Patch
pub const VERSION: Version = [0, 1, 0];

pub use entities::{Result, Storage, State, Location};
