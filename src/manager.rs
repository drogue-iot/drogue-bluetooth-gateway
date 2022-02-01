use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use log;
use std::sync::Arc;
use std::time::Duration;

use crate::board::*;
use crate::device::*;

pub struct DeviceManager {
    hci: String,
    bus: Arc<Connection>,
}

const ENVIRONMENTAL_SENSING_UUID: &str = "0000181a-0000-1000-8000-00805f9b34fb";

impl DeviceManager {
    pub fn new(ble_interface: &str, conn: Connection) -> DeviceManager {
        DeviceManager {
            hci: ble_interface.to_string(),
            bus: Arc::new(conn),
        }
    }

    pub fn is_connected(&self, address: &str) -> Result<bool, dbus::Error> {
        let path = format!("/org/bluez/{}/dev_{}", self.hci, address);
        let proxy = self
            .bus
            .with_proxy("org.bluez", &path, Duration::from_secs(10));
        let connected: bool = proxy.get("org.bluez.Device1", "Connected")?;
        Ok(connected)
    }

    pub fn connect(&self, address: &str) -> Result<BurrBoard, dbus::Error> {
        let path = format!("/org/bluez/{}/dev_{}", self.hci, address);
        let proxy = self
            .bus
            .with_proxy("org.bluez", &path, Duration::from_secs(10));
        // Retry connect until it succeeds or if we get an already connected error
        loop {
            let result: std::result::Result<(), dbus::Error> =
                proxy.method_call("org.bluez.Device1", "Connect", ());
            match result {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    if let Some(name) = e.name() {
                        if name == "org.bluez.Error.AlreadyConnected" {
                            break;
                        }
                    }
                    log::warn!("Error connecting, retrying...: {:?}", e);
                    std::thread::sleep(Duration::from_millis(2000));
                }
            }
        }

        let name: String = proxy.get("org.bluez.Device1", "Name")?;
        log::info!("Connected to {}", name);
        Ok(BurrBoard::new(Device::new(
            self.bus.clone(),
            self.hci.clone(),
            address.to_string(),
        )))
    }
}
