use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use chrono::Local;
use clap::crate_name;
use lazy_static::lazy_static;

use crate::error::{Error, Result};

lazy_static! {
    pub static ref CONFIG_DIRECTORY: Result<PathBuf> = {
        let path = dirs::config_dir()
            .ok_or_else(|| Error::ConfigError(String::new()))?
            .join(format!(".{}", crate_name!()));

        Ok(path)
    };
    pub static ref DATA_DIRECTORY: Result<PathBuf> = {
        let path = dirs::data_dir()
            .ok_or_else(|| Error::ConfigError(String::new()))?
            .join(crate_name!());

        Ok(path)
    };
    pub static ref CACHE_DIRECTORY: Result<PathBuf> = {
        let path = dirs::cache_dir()
            .ok_or_else(|| Error::ConfigError(String::new()))?
            .join(crate_name!());

        Ok(path)
    };
}

pub fn create_directories() -> Result<()> {
    let config_dir = CONFIG_DIRECTORY.as_ref().map_err(|_| {
        Error::ConfigError("unable to get config directory".to_string())
    })?;
    if !config_dir.exists() {
        create_dir_all(config_dir)?;
    }

    let cache_dir = CACHE_DIRECTORY.as_ref().map_err(|_| {
        Error::ConfigError("unable to get cache directory".to_string())
    })?;
    if !cache_dir.exists() {
        create_dir_all(cache_dir)?;
    }

    let data_dir = DATA_DIRECTORY.as_ref().map_err(|_| {
        Error::ConfigError("unable to get data directory".to_string())
    })?;
    if !data_dir.exists() {
        create_dir_all(data_dir)?;
    }

    Ok(())
}

// More logs to come
pub enum LogType {
    Crash,
}

pub fn save_log(log_type: LogType, log: String) -> Result<String> {
    let mut dir = DATA_DIRECTORY.as_ref().unwrap().clone();
    dir.push("logs");

    if !dir.exists() {
        create_dir_all(&dir)?;
    }

    let now = Local::now().format("%Y-%m-%d_%H-%M-%S%.3f");

    match log_type {
        LogType::Crash => {
            dir.push(format!("crash-report_{}.log", now));

            let mut file = File::create(&dir)?;
            file.write_all(log.as_bytes())?;

            Ok(dir.display().to_string())
        },
    }
}
