//! This module contains the repository for the source Lightroom catalogue.
//!
//! All actions are expected to be resolved against a valid Lightroom schema.
//!
//! This is NOT about the `Source` entity. TODO: Update this when the repository is known.

use super::Repository;
// use crate::entities::storage::Connection;
// use crate::entities::Result;

/// Abstracts the interaction with Storage regarding the Source.
#[derive(Debug, Clone)]
pub struct SourceRepository;

impl Repository for SourceRepository {}

impl SourceRepository {}
