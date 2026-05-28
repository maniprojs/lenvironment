use std::{
    collections::btree_map::Entry, fs, path::PathBuf
};

use anyhow::{Ok, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub runtime: String,
    pub image: String,
}

impl Environment {
    pub fn path(name: &str) -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("dev", "lenvironment", "lenv")
            .expect("failed to get project directories");

        let dir = proj_dirs.data_dir().join("environments");

        fs::create_dir_all(&dir)?;

        Ok(dir.join(format!("{name}.toml")))
    }

    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string_pretty(self)?;
        
        fs::write(Self::path(&self.name)?, toml)?;

        Ok(())
    }

    pub fn load(name: &str) -> Result<Self> {
        let content = fs::read_to_string(Self::path(name)?)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn list() -> Result<Vec<Self>> {
        let proj_dirs = ProjectDirs::from("dev", "lenvironment", "lenv")
            .expect("failed to get project directories");

        let dir = proj_dirs.data_dir().join("environments");

        fs::create_dir_all(&dir)?;

        let mut envs = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let content = fs::read_to_string(entry.path())?;

            let env: Self = toml::from_str(&content)?;

            envs.push(env);
        }

        Ok(envs)
    }
}
