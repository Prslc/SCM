use crate::config::ConfigManager;
use log::{info, error};

mod logger;
mod battery;
mod config;
mod constants;

fn main() {
    logger::init_logger(); 
    crate::battery::read_battery_info();

    let cfg_manager = ConfigManager::new();
    if let Some(cfg) = cfg_manager.get() {
        info!("Loaded config: {:?}", cfg);
    } else {
        error!("Using default configuration or exiting.");
    }
}
