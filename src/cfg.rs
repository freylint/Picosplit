//! Module containing logic for application configuration

use {
    derive_more::Constructor,
    serde::{Deserialize, Serialize},
    std::{fs::File, io::prelude::*, path::Path},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResolutionMode {
    Inner,
    Outer,
}

/// Struct containing application configuration data
#[derive(Constructor, Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resolution_mode: ResolutionMode,
}

impl Cfg {
    /// Create Cfg using default values
    pub fn default() -> Self {
        Self::new("PicoSplit".to_owned(), 400, 600, ResolutionMode::Inner)
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
                Err(_msg) => {
                    // Fixme handle as a warning
                    println!("WARN Failed to write default config. {}", _msg);
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
        file.read_to_end(&mut buffer)
            .expect("Failed to read cfg file into buffer");

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
