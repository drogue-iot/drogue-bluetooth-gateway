use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use std::sync::Arc;
use std::time::Duration;

pub struct Device {
    bus: Arc<Connection>,
    hci: String,
    address: String,
}

impl Device {
    pub fn new(bus: Arc<Connection>, hci: String, address: String) -> Device {
        Device { bus, hci, address }
    }
    pub fn read_value(&self, service: &str, characteristic: &str) -> Result<Vec<u8>, dbus::Error> {
        let path = format!("/org/bluez/{}/dev_{}", self.hci, self.address);
        let uuids = self.uuids()?;
        //log::info!("UUIDS: {:?}", uuids);

        let mut service_idx = 0;
        for idx in 1..40 {
            let path = format!("{}/service{:04x}", path, idx);
            //            log::info!("Looking up service : {}", path);
            let service_proxy =
                self.bus
                    .with_proxy("org.bluez", &path, Duration::from_millis(10000));

            if let Ok(uuid) = service_proxy.get::<String>("org.bluez.GattService1", "UUID") {
                if uuid == service {
                    //log::info!("Found matching index at {}", idx);
                    service_idx = idx;
                    break;
                }
            }
        }

        let mut char_idx = 0;
        for idx in 1..40 {
            let path = format!("{}/service{:04x}/char{:04x}", path, service_idx, idx);
            //log::info!("Looking up char: {}", path);
            let char_proxy = self
                .bus
                .with_proxy("org.bluez", &path, Duration::from_millis(10000));
            if let Ok(uuid) = char_proxy.get::<String>("org.bluez.GattCharacteristic1", "UUID") {
                //log::info!("Found uuid for char {}", &uuid[4..8]);
                if &uuid[4..8] == characteristic {
                    //log::info!("Found matching index at {}", idx);
                    char_idx = idx;
                    break;
                }
            }
        }

        if char_idx == 0 {
            return Ok(Vec::new());
        }
        let path = format!("{}/service{:04x}/char{:04x}", path, service_idx, char_idx);
        //log::info!("Using: {}", path);
        let char_proxy = self
            .bus
            .with_proxy("org.bluez", &path, Duration::from_millis(10000));
        //let (data,): (Vec<u8>,) =
        //    char_proxy.method_call("org.bluez.GattCharacteristic1", "ReadValue", ())?;
        let data: Vec<u8> = char_proxy.get("org.bluez.GattCharacteristic1", "Value")?;
        //        let path = format!("{}/service{:04x}/char{:04x}", path, idx + 1, 1);
        //        let char_proxy = self
        //            .bus
        //            .with_proxy("org.bluez", &path, Duration::from_millis(10000));
        //        let data: Vec<u8> = char_proxy.get("org.bluez.GattCharacteristic1", "Value")?;
        /*
        let path = format!("/org/bluez/{}/dev_{}", self.hci, self.address);
        let char_proxy = self.bus.with_proxy(
            "org.bluez",
            characteristic_path,
            Duration::from_millis(10000),
        );
        let data: Vec<u8> = char_proxy.get("org.bluez.GattCharacteristic1", "Value")?;
        */
        Ok(data)
    }

    pub fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        let path = format!("/org/bluez/{}/dev_{}", self.hci, self.address);
        let proxy = self
            .bus
            .with_proxy("org.bluez", &path, Duration::from_secs(10));
        proxy.get("org.bluez.Device1", "UUIDs")
    }
}
