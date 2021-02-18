use dbus::blocking::Connection;
use envconfig::Envconfig;
use log;
use std::time::Duration;

mod cloud;
mod device;
mod manager;
mod thermometer;

use crate::cloud::Cloud;
use crate::device::Sensor;
use crate::manager::DeviceManager;


#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "BLUETOOTH_INTERFACE", default = "hci0")]
    pub hci: String,
    #[envconfig(from = "BLUETOOTH_DEVICE")]
    pub device: String,
    #[envconfig(from = "SENSOR_POLL_INTERVAL", default = "10")]
    pub poll_interval: u64,
    #[envconfig(from = "MOCK_DEVICE", default = "false")]
    pub mock_device: bool,
    #[envconfig(from = "DROGUE_HTTP_ENDPOINT")]
    pub cloud_url: String,
    #[envconfig(from = "DROGUE_APPLICATION")]
    pub app: String,
    #[envconfig(from = "DROGUE_USERNAME")]
    pub username: String,
    #[envconfig(from = "DROGUE_PASSWORD")]
    pub password: String,
}

pub fn main() {
    env_logger::init();
    let config = Config::init_from_env().unwrap();

    let cloud = Cloud::new(config.cloud_url, config.app, config.username, config.password);
    let poll_interval = Duration::from_secs(config.poll_interval);

    if config.mock_device {
        loop {
            let s = "{\"temperature\": 24.3}";
            match cloud.publish(s.to_string()) {
                Ok(_) => {
                    log::info!("value published to the cloud");
                }
                Err(e) => {
                    log::error!("error publishing value: {}", e);
                }
            }
            std::thread::sleep(poll_interval);
        }
    } else {
        let conn = Connection::new_system().expect("error creating dbus connection");

        let device_manager = DeviceManager::new(&config.hci, conn);

        log::info!("BLE Gateway Started");

        // TODO: Make it possible to manage multiple devices.
        let address = &config.device.replace(":", "_");
        let sensor: Box<dyn Sensor> = device_manager
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
                    match cloud.publish(s) {
                        Ok(_) => {
                            log::info!("value published to the cloud");
                        }
                        Err(e) => {
                            log::error!("error publishing value: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            }
            std::thread::sleep(poll_interval);
        }
        log::info!("BLE sensor disconnected, shutting down");
    }
}
