//! Module containing logic for application configuration

use {
    serde::{Deserialize, Serialize},
    std::{fs::File, io::prelude::*, path::Path},
};

/// Struct containing application configuration data
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Cfg {
    /// Create Cfg from given data
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Cfg {
            title: title.to_owned(),
            width,
            height,
        }
    }

    /// Create Cfg using default values
    pub fn default() -> Self {
        Self::new("PicoSplit", 400, 600)
    }

    pub fn init_cfg(cfg_path: &Path) -> Box<Self> {
        let config = Cfg::default();
        // Check to see if config exists
        if cfg_path.exists() {
            // Load config
            Box::new(*Self::read(cfg_path))
        } else {
            // Write config to disk
            match Self::write(cfg_path, &config) {
                // Return config
                Ok(_count) => Box::new(config),
                Err(e) => {
                    // Fixme handle as a warning
                    println!("Failed to write default config.");
                    Box::new(config)
                }
            }
        }
    }

    /// Read Cfg config from file
    pub fn read(path: &Path) -> Box<Self> {
        // Open file
        let mut file = File::open(path).expect("Failed to open file for deserializing cfg.");

        // Read file into buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read cfg file into buffer");

        // Deserialize Cfg
        Box::new(toml::from_slice(&buffer).expect("Failed to deserialize cfg data"))
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
