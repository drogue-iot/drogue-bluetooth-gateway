use dbus::blocking::Connection;
use envconfig::Envconfig;
use log;
use std::time::Duration;

mod board;
mod device;
mod manager;

use crate::board::BurrBoard;
use crate::manager::DeviceManager;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "BLUETOOTH_INTERFACE", default = "hci0")]
    pub hci: String,
    #[envconfig(from = "BLUETOOTH_DEVICE")]
    pub device: String,
    #[envconfig(from = "SENSOR_POLL_INTERVAL", default = "10")]
    pub poll_interval: u64,
}

pub fn main() {
    env_logger::init();
    let config = Config::init_from_env().unwrap();

    let poll_interval = Duration::from_secs(config.poll_interval);

    let conn = Connection::new_system().expect("error creating dbus connection");

    let device_manager = DeviceManager::new(&config.hci, conn);

    log::info!("BLE Gateway Started");

    // TODO: Make it possible to manage multiple devices.
    let address = &config.device.replace(":", "_");
    let sensor: BurrBoard = device_manager
        .connect(&address)
        .expect("unable to connect to device");

    // TODO: Handle reconnects
    while device_manager
        .is_connected(&address)
        .expect("unable to check connectivity")
    {
        match sensor.read() {
            Ok(value) => {
                let s = value.to_string();
                log::info!("{}", &s);
                /*   match cloud.publish(s) {
                    Ok(_) => {
                        log::info!("value published to the cloud");
                    }
                    Err(e) => {
                        log::error!("error publishing value: {}", e);
                    }
                }
                */
            }
            Err(e) => {
                log::error!("{}", e);
            }
        }
        std::thread::sleep(poll_interval);
    }
    log::info!("BLE sensor disconnected, shutting down");
}
