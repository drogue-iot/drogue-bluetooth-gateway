use serde_json::json;

use crate::device::*;

pub struct Thermometer {
    device: Device,
}

impl Thermometer {
    pub fn new(device: Device) -> Thermometer {
        Thermometer { device }
    }
}

impl Sensor for Thermometer {
    fn read(&self) -> Result<serde_json::Value, dbus::Error> {
        let data = self.device.read_value()?;
        let mut temp: u32 = 0;
        temp |= (data[0] as u32) << 24;
        temp |= (data[1] as u32) << 16;
        temp |= (data[2] as u32) << 8;
        temp |= data[3] as u32;

        Ok(json!({ "temperature": temp }))
    }
}
