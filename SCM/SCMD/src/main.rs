use crate::config::ConfigManager;
use log::{info, error};

mod logger;
mod battery;
mod config;
mod constants;
mod utils;

fn main() {
    // init
    logger::init_logger().expect("Failed to initialize logger"); 
    let cfg_manager = ConfigManager::new();
    let battery_info = crate::battery::read_battery_info();
    
    cfg_manager.watch();
    info!("Listening to configuration file....");
    let level = battery_info.level();
    info!("Level: {:?}", level);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

}
