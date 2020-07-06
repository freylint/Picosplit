#![deny(missing_docs, clippy::all)] 
//! Minimalist speedrun timer

use conrod::{widget, color, Colorable, Positionable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

#[derive(Debug)]
struct ApplicationMetaInfo {
    name: String,
    version : String,
}

fn main() {
    // Setup meta information for build
    let _meta = ApplicationMetaInfo { 
        name: "Picosplit".to_string(), 
        version : "0.0.1".to_string(),
    };

    println!("Hello, world!");
}
