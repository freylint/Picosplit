#![deny(missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;
mod cli;

use {
    clap::{crate_authors, crate_version, App, Arg},
    conrod::{
        backend::glium::{
            glium,
            glium::{DisplayBuild, Surface},
        },
        color, widget, Colorable, Positionable, Widget,
    },
    toml::{Deserializer, Serializer},
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("./res/sys/cli.yml");
    let matches = App::from(cli_argfile).get_matches();

    // Process command line arguments
    match matches {

    }
    


    // Build application window window.
    let display = glium::glutin::WindowBuilder::new()
        //.with_vsync()
        .with_dimensions(400, 600)
        .with_title("PicoSplit")
        .with_decorations(false)
        //.with_multisampling(4)
        .build_glium()
        .unwrap();

    // Construct UI
    // FIXME load from cfg
    let mut ui = conrod::UiBuilder::new([400.0, 600.0]).build();

    println!("Hello, world!");
}
