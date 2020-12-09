use dbus::blocking::Connection;
use log;
use std::env;
use std::time::Duration;

mod device;
mod manager;
mod thermometer;

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
            .expect("error retrieving poll interval"),
    );

    let conn = Connection::new_system().expect("error creating dbus connection");

    let device_manager = DeviceManager::new(&hci, conn);

    log::info!("BLE Gateway Started");

    // TODO: Make this dynamic
    let sensor: Box<dyn Sensor> = device_manager
        .connect(&device.replace(":", "_"))
        .expect("unable to connect to device");

    loop {
        match sensor.read() {
            Ok(value) => log::info!("{}", value),
            Err(e) => {
                log::error!("{}", e);
            }
        }
        std::thread::sleep(poll_interval);
    }
}
