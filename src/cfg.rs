//! Module containing logic for application configuration

/// Struct containing application configuration data
#[derive(Debug)]
pub struct Cfg {
    title: String,
    width: u32,
    height: u32
}


impl Cfg {

    /// Create Cfg from given data
    pub fn new(title: String, width: u32, height: u32) -> Self{
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


}