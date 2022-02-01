use crate::device::*;
use serde_json::json;

pub struct BurrBoard {
    device: Device,
}

impl BurrBoard {
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    pub fn read(&self) -> Result<serde_json::Value, dbus::Error> {
        const BOARD_SERVICE_UUID: &str = "00001860-0000-1000-8000-00805f9b34fb";
        const TEMPERATURE_CHAR_UUID: &str = "2a6e";

        let data = self
            .device
            .read_value(BOARD_SERVICE_UUID, TEMPERATURE_CHAR_UUID)?;
        let temp: u32 = data[0] as u32;

        Ok(json!({ "temperature": temp }))
    }
}
