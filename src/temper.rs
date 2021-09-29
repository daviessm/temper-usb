use rusb::{Device, DeviceHandle, GlobalContext};
use std::time::Duration;

pub struct TemperDevice {
    device_handle: DeviceHandle<GlobalContext>,
}

impl TemperDevice {
    pub fn new(device: Device<GlobalContext>) -> TemperDevice {
        let mut device = device.open().unwrap();

        for iface in 0x00..0x01 {
            if device.kernel_driver_active(iface).unwrap() {
                eprintln!("Detaching kernel driver for iface {}", iface);
                device.detach_kernel_driver(iface).unwrap();
            }
        }

        device.set_active_configuration(0x01).unwrap();
        device.claim_interface(0x01).unwrap();
        device.release_interface(0x01).unwrap();
        TemperDevice {
            device_handle: device,
        }
    }

    pub fn get_temp(&self) -> f32 {
        self.device_handle
            .write_control(
                0x21,
                0x09,
                0x0200,
                0x01,
                &[0x01, 0x80, 0x33, 0x01, 0x00, 0x00, 0x00, 0x00],
                Duration::new(5, 0),
            )
            .unwrap();

        let mut buf = [0u8; 8];
        self.device_handle
            .read_bulk(0x82, &mut buf, Duration::new(5, 0))
            .unwrap();

        let b2 = buf[2] as u16;
        let b3 = buf[3] as u16;
        if b2 == 0x4e && b3 == 0x20 {
            panic!("Failed to read from device");
        }

        let t: i16 = (b2 as i16) << 8u32 | b3 as i16;
        (t as f32) / 256.0
    }
}

pub fn iterate_usb_devices() -> Option<TemperDevice> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == 0x0c45 && device_desc.product_id() == 0x7401 {
            return Some(TemperDevice::new(device));
        }
    }
    panic!("No TEMPer device found!");
}
