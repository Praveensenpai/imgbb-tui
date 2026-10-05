use crate::domain::error::AppError;
use crate::domain::model::HistoryEntry;
use std::path::PathBuf;

const MAX_HISTORY: usize = 50;

pub struct HistoryStore {
    path: PathBuf,
}

impl HistoryStore {
    pub fn new() -> Result<Self, AppError> {
        let dirs = crate::infra::config::Config::project_dirs()?;
        let dir = dirs.data_dir();
        std::fs::create_dir_all(dir)?;
        Ok(Self {
            path: dir.join("history.json"),
        })
    }

    pub fn load(&self) -> Vec<HistoryEntry> {
        let Ok(raw) = std::fs::read_to_string(&self.path) else {
            return Vec::new();
        };
        serde_json::from_str(&raw).unwrap_or_default()
    }

    pub fn push(&self, entry: HistoryEntry) -> Result<(), AppError> {
        let mut entries = self.load();
        entries.insert(0, entry);
        entries.truncate(MAX_HISTORY);
        let raw =
            serde_json::to_string_pretty(&entries).map_err(|e| AppError::Config(e.to_string()))?;
        std::fs::write(&self.path, raw)?;
        Ok(())
    }
}
