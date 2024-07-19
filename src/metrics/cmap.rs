use anyhow::Result;
use core::fmt;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>, //Arc<Mutex<DashMap<String, i64>>>
}

impl CmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

impl Default for CmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}
