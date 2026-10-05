use crate::domain::error::AppError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "Praveensenpai";
const APPLICATION: &str = "imgbb-tui";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: String,
    pub auto_copy: bool,
    pub keep_history: bool,
}

impl Config {
    pub fn project_dirs() -> Result<ProjectDirs, AppError> {
        ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .ok_or_else(|| AppError::Config("cannot resolve config directory".to_string()))
    }

    pub fn path() -> Result<PathBuf, AppError> {
        let dirs = Self::project_dirs()?;
        let dir = dirs.config_dir();
        std::fs::create_dir_all(dir)?;
        Ok(dir.join("config.toml"))
    }

    pub fn load() -> Result<Self, AppError> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| AppError::Config(e.to_string()))
    }

    pub fn save(&self) -> Result<(), AppError> {
        let path = Self::path()?;
        let raw = toml::to_string_pretty(self).map_err(|e| AppError::Config(e.to_string()))?;
        std::fs::write(&path, raw)?;
        Ok(())
    }

    pub fn needs_api_key(&self) -> bool {
        self.api_key.trim().is_empty()
    }
}
