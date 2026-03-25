//! Assets Manager and Data Analyst agent utilities.
//!
//! This module maintains a structured source library for external assets and APIs.

use crate::{RalphError, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// A single discovered external asset entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRecord {
    pub id: String,
    pub title: String,
    pub source_url: String,
    pub asset_type: String,
    pub discovered_at_utc: String,
    pub relevance: String,
    pub used_in_paths: Vec<String>,
    pub notes: String,
}

/// Source library persisted under `ll/_data`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetLibrary {
    pub generated_by: String,
    pub generated_at_utc: String,
    pub records: Vec<AssetRecord>,
}

impl Default for AssetLibrary {
    fn default() -> Self {
        Self {
            generated_by: "Assets Manager and Data Analyst Agent".to_string(),
            generated_at_utc: Utc::now().to_rfc3339(),
            records: Vec::new(),
        }
    }
}

/// Lightweight utility agent for managing project asset/source metadata.
#[derive(Debug, Clone)]
pub struct AssetsManagerDataAnalystAgent {
    library_path: PathBuf,
}

impl AssetsManagerDataAnalystAgent {
    pub fn new(library_path: impl Into<PathBuf>) -> Self {
        Self {
            library_path: library_path.into(),
        }
    }

    pub fn library_path(&self) -> &Path {
        &self.library_path
    }

    /// Ensure the asset library exists and has valid JSON.
    pub fn ensure_library(&self) -> Result<()> {
        if let Some(parent) = self.library_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                RalphError::Configuration(format!(
                    "Failed to create asset library directory '{}': {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        if !self.library_path.exists() {
            let initial = AssetLibrary::default();
            self.write_library(&initial)?;
        }

        Ok(())
    }

    /// Create starter records from a prompt to seed discovery work.
    pub fn seed_from_prompt(&self, prompt: &str) -> Result<()> {
        let mut library = self.read_library()?;
        let now = Utc::now().to_rfc3339();
        let keywords = prompt
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| w.len() > 4)
            .take(6)
            .collect::<Vec<_>>();

        for keyword in keywords {
            library.records.push(AssetRecord {
                id: Uuid::new_v4().to_string(),
                title: format!("Discovery queue: {}", keyword),
                source_url: format!("https://www.google.com/search?q={}", keyword),
                asset_type: "search-seed".to_string(),
                discovered_at_utc: now.clone(),
                relevance: "candidate".to_string(),
                used_in_paths: vec![],
                notes: "Seed query for continual web discovery (photos/media/sheets/pdfs/apis)."
                    .to_string(),
            });
        }

        library.generated_at_utc = now;
        self.write_library(&library)
    }

    fn read_library(&self) -> Result<AssetLibrary> {
        let content = fs::read_to_string(&self.library_path).map_err(|e| {
            RalphError::Configuration(format!(
                "Failed to read asset library '{}': {}",
                self.library_path.display(),
                e
            ))
        })?;
        serde_json::from_str(&content).map_err(|e| {
            RalphError::Configuration(format!(
                "Asset library JSON is invalid at '{}': {}",
                self.library_path.display(),
                e
            ))
        })
    }

    fn write_library(&self, library: &AssetLibrary) -> Result<()> {
        let content = serde_json::to_string_pretty(library).map_err(|e| {
            RalphError::Configuration(format!("Failed to serialize asset library: {}", e))
        })?;
        fs::write(&self.library_path, content).map_err(|e| {
            RalphError::Configuration(format!(
                "Failed to write asset library '{}': {}",
                self.library_path.display(),
                e
            ))
        })?;
        Ok(())
    }
}
