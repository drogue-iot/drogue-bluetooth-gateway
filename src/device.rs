use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use std::sync::Arc;
use std::time::Duration;

pub trait Sensor {
    fn read(&self) -> Result<serde_json::Value, dbus::Error>;
}

pub struct Device {
    bus: Arc<Connection>,
    characteristic_path: String,
}

impl Device {
    pub fn new(bus: Arc<Connection>, characteristic_path: String) -> Device {
        Device {
            bus,
            characteristic_path,
        }
    }
    pub fn read_value(&self) -> Result<Vec<u8>, dbus::Error> {
        let char_proxy = self.bus.with_proxy(
            "org.bluez",
            &self.characteristic_path,
            Duration::from_millis(10000),
        );
        let data: Vec<u8> = char_proxy.get("org.bluez.GattCharacteristic1", "Value")?;
        Ok(data)
    }
}
