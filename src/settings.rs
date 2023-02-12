use std::{any, collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};

use crate::types;

pub trait Config {
    fn get_profile(&self) -> &String;

    fn save(self) -> types::BackupResult<Self>
    where
        Self: Sized,
        Self: Serialize,
        for<'a> Self: Deserialize<'a>,
    {
        let config_file_name = Self::config_file_name();
        let profile = &self.get_profile().to_owned();

        let mut configs = match fs::read_to_string(&config_file_name) {
            Ok(json_data) => {
                let mut configs: HashMap<String, Self> = serde_json::from_str(&json_data).unwrap();
                configs.insert(profile.to_owned(), self);
                configs
            }
            Err(_) => HashMap::from([(profile.to_owned(), self)]),
        };
        fs::write(
            config_file_name,
            serde_json::to_string_pretty(&configs).unwrap(),
        )?;
        let config = configs.remove(profile).unwrap();
        return Ok(config);
    }

    fn config_file_name() -> String {
        let name = any::type_name::<Self>()
            .to_string()
            .split("::")
            .last()
            .unwrap()
            .to_string()
            .to_lowercase();
        return format!("abt-{}.json", name);
    }

    fn load(profile: &String) -> types::BackupResult<Self>
    where
        Self: Sized,
        Self: Serialize,
        for<'a> Self: Deserialize<'a>,
    {
        let config_file_name = Self::config_file_name();

        let config: Self = match fs::read_to_string(&config_file_name) {
            Ok(json_data) => {
                let mut configs: HashMap<String, Self> = serde_json::from_str(&json_data).unwrap();
                match configs.remove(profile) {
                    Some(config) => config,
                    None => Err(types::BackupError::ConfigError(
                        "config not found for the given profile",
                    ))?,
                }
            }
            Err(err) => Err(err)?,
        };

        return Ok(config);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Storage {
    Local { target_path: PathBuf },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Backup {
    profile: String,
    pub source_path: PathBuf,
    pub storage: Storage,
}

impl Config for Backup {
    fn get_profile(&self) -> &String {
        return &self.profile;
    }
}

impl Backup {
    fn new(profile: &String, source: &PathBuf, target: &PathBuf) -> Self {
        return Self {
            profile: profile.to_string(),
            source_path: source.to_path_buf(),
            storage: Storage::Local {
                target_path: target.to_path_buf(),
            },
        };
    }

    pub fn source_path(&mut self, source_path: &PathBuf) -> &Self {
        self.source_path = source_path.to_path_buf();
        return self;
    }

    pub fn target_path(&mut self, target_path: &PathBuf) -> &Self {
        self.storage = Storage::Local {
            target_path: target_path.to_path_buf(),
        };
        return self;
    }

    pub fn load_or_create(
        profile: &String,
        source_path: &Option<PathBuf>,
        target_path: &Option<PathBuf>,
    ) -> types::BackupResult<Self>
    where
        Self: Sized,
        Self: Serialize,
        for<'a> Self: Deserialize<'a>,
    {
        let config = match Self::load(profile) {
            Ok(config) => config,
            Err(_) => {
                let source_path = source_path
                    .as_ref()
                    .ok_or(types::BackupError::ConfigError("Missing source_path field"))?;
                let target_path = target_path
                    .as_ref()
                    .ok_or(types::BackupError::ConfigError("Missing target_path field"))?;
                let config = Self::new(profile, source_path, target_path);
                config.save()?
            }
        };

        return Ok(config);
    }
}
