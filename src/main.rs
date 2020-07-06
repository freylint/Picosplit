#![deny(missing_docs, clippy::all)]
//! Minimalist speedrun timer


use {
    conrod::{
        backend::glium::{
            glium,
            glium::{DisplayBuild, Surface},
        },
        color, widget, Colorable, Positionable, Widget,
    },
    clap::{App, Arg, crate_version, crate_authors},
    toml::{Deserializer, Serializer},
};


fn main() {
    // Parse command line args
    let args = App::new("Picosplit")
        // Load meta info from cargo
       .version(crate_version!())
       .author(crate_authors!())
       // Set manual meta info
       .about("Minimalist speedrun timer")

       // Config path arg
       .arg(Arg::with_name("cfg-dir")
            .short("cfg")
            .long("cfg-dir")
            .value_name("FILE")
            .default_value("./res/cfg/cfg.toml")
            .help("Sets a path to load configuration file from")
            .takes_value(true))

        // TODO Width arg
        // TODO Height arg
       .get_matches(); 

    // Build application window window.
    let display = glium::glutin::WindowBuilder::new()
        //.with_vsync()
        .with_dimensions(400, 600)
        .with_title("PicoSplit")
        //.with_multisampling(4)
        .build_glium()
        .unwrap();
    // Construct UI
    // FIXME load from cfg
    let mut ui =
        conrod::UiBuilder::new([400.0, 600.0]).build();

    println!("Hello, world!");
}
