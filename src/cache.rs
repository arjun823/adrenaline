use anyhow::Result;
/// Cache system
/// Hash-based compilation caching
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Cache {
    cache_dir: PathBuf,
}

impl Cache {
    pub fn new(_base_dir: &Path) -> Result<Self> {
        let adrenaline_home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".adrenaline");
        let cache_dir = adrenaline_home.join(".cache");
        fs::create_dir_all(&cache_dir)?;
        Ok(Self { cache_dir })
    }

    pub fn get_hash(source: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(source.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn get_cache_path(&self, source_hash: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.rs", source_hash))
    }

    pub fn has_cached(&self, source_hash: &str) -> bool {
        self.get_cache_path(source_hash).exists()
    }

    pub fn get_cached(&self, source_hash: &str) -> Result<String> {
        let path = self.get_cache_path(source_hash);
        Ok(fs::read_to_string(path)?)
    }

    pub fn cache(&self, source_hash: &str, code: &str) -> Result<()> {
        let path = self.get_cache_path(source_hash);
        fs::write(path, code)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    pub fn size(&self) -> Result<u64> {
        let mut total = 0u64;
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                total += metadata.len();
            }
        }
        Ok(total)
    }
}
