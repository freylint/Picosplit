#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    orbtk::prelude::*,
    std::path::Path,
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("../res/sys/cli.yaml");
    let _matches = App::from(cli_argfile).get_matches();

    let _cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("PicoSplit")
                // Commented out so that window manager handles initial pos
                //.position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();

    println!("Hello, world!");
}
