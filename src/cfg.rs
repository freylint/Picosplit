//! Module containing logic for application configuration

use {
    serde::{Deserialize, Serialize},
    std::{path::Path,fs::File, io::prelude::*},
};

/// Struct containing application configuration data
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    title: &'static str,
    width: u32,
    height: u32,
}

impl Cfg {
    /// Create Cfg from given data
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Cfg {
            title,
            width,
            height,
        }
    }

    /// Create Cfg using default values
    pub fn default() -> Self {
        Self::new("PicoSplit", 400, 600)
    }

    pub fn init_cfg(cfg_path: &Path) -> Self {
        // Check to see if config exists
        if cfg_path.exists() {
            // Load config
            Self::read(cfg_path)
        } else {
            // Create default config
            Self::write(cfg_path, &Self::default());


            Self::read(cfg_path)
        }

    }

    /// Read Cfg config from file
    pub fn read(path: &Path) -> Self {
        let config = File::open(path);
        let mut buf = "";
        config.unwrap().read_to_string(&mut buf.to_string());
        toml::from_str(buf).unwrap()
    }

    /// Writes a config to file
    pub fn write(path: &Path, cfg: &Cfg) -> std::io::Result<()> {
        // Open file
        let mut file = File::create(path)?;

        // Buffer serialized default config
        let buffer = toml::to_vec(cfg).unwrap();

        // Write buffer to file
        file.write_all(&buffer)
    }
}
