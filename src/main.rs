#![deny(missing_debug_implementations, missing_docs, clippy::all)]
//! Minimalist speedrun timer

mod cfg;

use {
    cfg::Cfg,
    clap::{load_yaml, App},
    std::path::Path,
    vulkano::instance::{Instance, InstanceExtensions},
};

fn main() {
    // Parse command line args
    let cli_argfile = load_yaml!("../res/sys/cli.yaml");
    let _matches = App::from(cli_argfile).get_matches();

    let _cfg = Cfg::init_cfg(Path::new("./res/cfg/cfg.toml"));
    
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");



    println!("Hello, world!");
}
