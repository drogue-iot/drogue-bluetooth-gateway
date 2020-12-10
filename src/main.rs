use dbus::blocking::Connection;
use log;
use std::env;
use std::time::Duration;

mod cloud;
mod device;
mod manager;
mod thermometer;

use crate::cloud::Cloud;
use crate::device::Sensor;
use crate::manager::DeviceManager;

pub fn main() {
    env_logger::init();

    let hci = env::var("BLUETOOTH_INTERFACE")
        .ok()
        .unwrap_or("hci0".to_string());

    let device =
        env::var("BLUETOOTH_DEVICE").expect("BLUETOOTH_DEVICE environment variable is not set");

    let poll_interval: Duration = Duration::from_secs(
        env::var("SENSOR_POLL_INTERVAL")
            .ok()
            .map(|s| s.parse())
            .unwrap_or(Ok(10 as u64))
            .expect("error retrieving poll interval from SENSOR_POLL_INTERVAL"),
    );

    let mock_device: bool = env::var("MOCK_DEVICE")
        .ok()
        .map(|s| s.parse())
        .unwrap_or(Ok(false))
        .expect("error parsing mock_device from MOCK_DEVICE");

    let cloud_url = env::var("DROGUE_HTTP_ENDPOINT")
        .expect("unable to retrieve cloud http endpoint from DROGUE_HTTP_ENDPOINT");

    let username = env::var("DROGUE_USERNAME")
        .expect("unable to retrieve cloud username from DROGUE_USERNAME");

    let password = env::var("DROGUE_PASSWORD")
        .expect("unable to retrieve cloud password from DROGUE_PASSWORD");

    let cloud = Cloud::new(cloud_url, username, password);

    if mock_device {
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

        let device_manager = DeviceManager::new(&hci, conn);

        log::info!("BLE Gateway Started");

        // TODO: Make it possible to manage multiple devices.
        let address = &device.replace(":", "_");
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
