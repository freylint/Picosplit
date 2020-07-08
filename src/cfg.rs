//! Module containing logic for application configuration

use {
    serde::{Deserialize, Serialize},
    std::{path::Path,fs::File, io::prelude::*},
};

/// Struct containing application configuration data
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg<'a> {
    title: &'a str,
    width: u32,
    height: u32,
}

impl<'a> Cfg<'a> {
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
            let config = Self::default();
            // Write config to disk
            match Self::write(cfg_path, &config) {
                // Return config
                Ok(_count) => config,
                Err(e) => {
                    println!("Failed to write default config.");
                    panic!(e)
                },
            }
        }

    }

    /// Read Cfg config from file
    pub fn read(path: &Path) -> Self {
        // Open file
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open config file for reading.");
                panic!(err)
            }
        };

        // Read file into buffer
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);

        let mut data = String::new();
        String::clone_from(&mut data, & buffer);

        // Deserialize Cfg
        match toml::from_str(data.as_str()) {
            Ok(config) => config,
            Err(err) => {
                println!("Failed to deserialize user config.");
                panic!(err)
            }
        }
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
