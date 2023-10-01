use log::info;
use std::env;
use std::fs;

use serde::Deserialize;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod frametimer;
mod websrv;
//mod world;
mod transport;

// Switch to https://stackoverflow.com/a/65972328/12292193 if needed
const MAJOR_VERSION: u8 = 0;
const MINOR_VERSION: u8 = 1;
const PATCH_VERSION: u8 = 0;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
struct Config {
    websrv: Websrv,
}

#[derive(Deserialize)]
struct Websrv {
    port: u16,
}

fn get_sigint_flag() -> Result<Arc<AtomicBool>, Box<dyn Error>> {
    let flag = Arc::new(AtomicBool::new(true));

    let f = flag.clone();
    ctrlc::set_handler(move || {
        info!("Received SIGINT (CTRL+C), exiting...");
        f.store(false, Ordering::SeqCst);
    })?;

    Ok(flag)
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "trace")
    }
    env_logger::init();

    info!("Starting server V{MAJOR_VERSION}.{MINOR_VERSION}.{PATCH_VERSION}");

    let running = get_sigint_flag()?;
    let config: Config = toml::from_str(fs::read_to_string(CONFIG_PATH)?.as_str())?;

    info!("Port: {}", config.websrv.port);

    //websrv::init();
    //world::init();
    frametimer::init();

    info!("Server ready");

    while running.load(Ordering::SeqCst) {
        frametimer::wait_for_frame();
        //world::simulate_frame();
    }

    //websrv::stop();
    //world::stop();
    info!("Shutdown successful");

    Ok(())
}
