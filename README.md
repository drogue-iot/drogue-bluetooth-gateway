# drogue-bluetooth-gateway

The drogue-bluetooth-gateway is a process that runs on an bluetooth-enabled edge device running the bluetooth daemon (bluez). The gateway communicates with devices using BlueZ (via DBus), reads sensor data from BLE GATT services exposed by devices, and pushes the data to the Drogue Cloud HTTP endpoint.

Currently, only environment sensing service GATT service is supported, and there is no device management.
