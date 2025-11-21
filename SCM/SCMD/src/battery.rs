use crate::constants::BASE_DIR;

use std::fs;
use std::path::Path;

#[derive(Debug)]
#[allow(dead_code)]
pub struct BatteryInfo {
    capacity: Option<i32>,
    voltage_now: Option<i32>,
    current_now: Option<i32>,
    status: Option<String>,
    health: Option<String>,
}

fn read_file<P: AsRef<Path>>(path: P) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    Some(content.trim().to_string())
}

fn read_int<P: AsRef<Path>>(path: P) -> Option<i32> {
    read_file(path)?.parse::<i32>().ok()
}

pub fn read_battery_info() -> BatteryInfo {
    let base = &BASE_DIR;

    BatteryInfo {
        capacity: read_int(format!("{}/capacity", base)),
        voltage_now: read_int(format!("{}/voltage_now", base)),
        current_now: read_int(format!("{}/current_now", base)),
        status: read_file(format!("{}/status", base)),
        health: read_file(format!("{}/health", base)),
    }
}

impl BatteryInfo {
    crate::opt_getter!(level, capacity);
    crate::opt_getter!(voltage, voltage_now);
    crate::opt_getter!(current, current_now);
    crate::opt_getter!(status, status, str);
    crate::opt_getter!(health, health, str);
}

pub struct ChargeSwitch {
    valid_path: Option<String>,
}

impl ChargeSwitch {
    pub fn new() -> Self {
        /* Charge-control sysfs paths are vendor-specific and not guaranteed to work
        * on all devices. These entries are collected from common OEM kernels, but
        * not all of them have been tested.
        *
        * If you find missing or incorrect paths, please open an issue or submit a PR.
        */

        let switch_file_path = [
            "/sys/class/power_supply/battery/input_suspend",    // Qualcomm
            "/sys/class/power_supply/battery/charging_enabled",  // Qualcomm old path
            "/sys/class/power_supply/battery/op_disable_charge", // Oppo and OnePlus
            "/sys/class/power_supply/battery/batt_slate_mode",   // MTK
            "/sys/class/power_supply/battery/store_mode"    // Samsung
        ];

        let valid_path = switch_file_path.iter()
            .find(|p| Path::new(p).exists())
            .map(|s| s.to_string());

        Self { valid_path }
    }

    pub fn switch(&self, enable: bool) {
        if let Some(path) = &self.valid_path {
            let _ = fs::write(path, if enable { "1" } else { "0" });
        }
    }
}