use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use log;
use std::sync::Arc;
use std::time::Duration;

use crate::device::*;
use crate::thermometer::*;

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

    pub fn connect(&self, address: &str) -> Result<Box<dyn Sensor>, dbus::Error> {
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
        let uuids: Vec<String> = proxy.get("org.bluez.Device1", "UUIDs")?;

        let (idx, _) = uuids
            .iter()
            .enumerate()
            .find(|(_, n)| *n == ENVIRONMENTAL_SENSING_UUID)
            .expect("Unable to locate environmental sensing service");

        Ok(Box::new(Thermometer::new(Device::new(
            self.bus.clone(),
            format!("{}/service{:04}/char{:04}", path, idx + 1, 2),
        ))))
    }
}
