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
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
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
        let mut config = Cfg::default();
        // Check to see if config exists
        if cfg_path.exists() {
            // Load config
            config = *Self::read(cfg_path);

            // Return heap allocated Cfg instance
            return Box::new(config);
        } else {
            // Write config to disk
            match Self::write(cfg_path, &config) {
                // Return config
                Ok(_count) => Box::new(config),
                Err(e) => {
                    println!("Failed to write default config.");
                    panic!(e)
                }
            }
        }
    }

    /// Read Cfg config from file
    pub fn read(path: &Path) -> Box<Self> {
        if path.exists() {
            // Open file
            let mut file = match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    println!("Failed to open config file for reading.");
                    panic!(err)
                }
            };

            // Read file into buffer
            let mut buffer = Vec::new();
            match file.read(&mut *buffer) {
                Ok(_size) => {}
                Err(err) => {
                    print!("Failed to read cfg file into buffer");
                    panic!(err)
                }
            };

            // Deserialize Cfg
            Box::new(match toml::from_slice(&buffer) {
                Ok(config) => config,
                Err(err) => {
                    println!("Failed to deserialize user config.");
                    panic!(err)
                }
            })
        } else {
            panic!("No cofig file at given path.")
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
