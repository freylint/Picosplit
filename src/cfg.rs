//! Module containing logic for application configuration

use {
    serde::{Deserialize, Serialize},
    std::{fs::File, io::prelude::*},
};

/// Struct containing application configuration data
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    title: String,
    width: u32,
    height: u32,
}

impl Cfg {
    /// Create Cfg from given data
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Cfg {
            title: title,
            width: width,
            height: height,
        }
    }

    /// Create Cfg using default values
    pub fn default() -> Self {
        Cfg {
            title: "PicoSplit".to_owned(),
            width: 400,
            height: 600,
        }
    }

    /// Read Cfg config from file
    pub fn read(path: &str) -> Self {
        // Open file
        let mut file = File::open(path).unwrap();
        let mut buffer = vec![];

        // Read file to buffer
        file.read(&mut buffer).unwrap();
        
        // Deserialize vuffer
        toml::from_slice(&mut buffer).unwrap()
    }

    /// Writes a config to file
    pub fn write(path: &str, cfg: &Cfg) -> std::io::Result<()> {
        // Open file
        let mut file = File::create(path)?;

        // Buffer serialized default config
        let buffer = toml::to_vec(cfg).unwrap();

        // Write buffer to file
        file.write_all(&buffer)
    }
}
