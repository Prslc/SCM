use std::fs;
use std::sync::{Arc, RwLock};
use std::thread;

use inotify::{Inotify, WatchMask};
use serde::Deserialize;

use crate::constants::CONFIG_DIR;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    start_level: i32,
    stop_level: i32,
    stop_current_limit: i32,
    maintain_full_charge: bool,
    stoptime: u64,
    debug: bool,
}

pub struct ConfigManager {
    inner: Arc<RwLock<Option<Config>>>,
}

impl ConfigManager {
    pub fn new() -> Self {
        let cfg = Self::load();
        Self {
            inner: Arc::new(RwLock::new(cfg)),
        }
    }

    pub fn load() -> Option<Config> {
        let content = fs::read_to_string(CONFIG_DIR).ok()?;
        toml::from_str::<Config>(&content).ok()
    }

    /// Get a clone of the current config safely
    pub fn get(&self) -> Option<Config> {
        self.inner.read().ok()?.clone()
    }

    /// Spawn a thread to watch the config file
    pub fn watch(&self) {
        let cfg_ref = self.inner.clone();

        thread::spawn(move || {
            let mut inotify = Inotify::init().expect("Failed to initialize inotify");

            inotify
                .watches()
                .add(
                    CONFIG_DIR,
                    WatchMask::MODIFY | WatchMask::CLOSE_WRITE,
                )
                .expect("Failed to add inotify watch");

            let mut buffer = [0u8; 1024];

            loop {
                let events = inotify
                    .read_events_blocking(&mut buffer)
                    .expect("Failed to read inotify events");

                for event in events {
                    if event.mask.contains(inotify::EventMask::MODIFY)
                        || event.mask.contains(inotify::EventMask::CLOSE_WRITE)
                    {
                        println!("Config modified, reloading...");

                        if let Some(new_cfg) = Self::load() {
                            let mut lock = cfg_ref.write().unwrap();
                            *lock = Some(new_cfg.clone());

                            println!("New config: {:?}", new_cfg);
                        } else {
                            eprintln!("Failed to reload config");
                        }
                    }
                }
            }
        });
    }
}
