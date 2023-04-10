use std::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

use crate::UsbId;

pub fn find_reboot_endpoint(
    usb_id: UsbId,
) -> Result<(DeviceHandle<GlobalContext>, u8), anyhow::Error> {
    log::debug!("Iterating USB devices");

    let device = rusb::devices()?
        .iter()
        .find(|device| {
            let descriptor = device.device_descriptor().unwrap();

            let id = UsbId {
                vid: descriptor.vendor_id(),
                pid: descriptor.product_id(),
            };

            log::trace!("Checking {id}");
            id == usb_id
        })
        .ok_or(anyhow::anyhow!(
            "Could not find device with correct VID:PID"
        ))?;

    log::debug!("Found {:?}", device);

    let handle = device.open()?;

    let lang = handle
        .read_languages(Duration::from_millis(500))?
        .get(0)
        .ok_or(anyhow::anyhow!("Could not read languages..."))?
        .clone();

    let config = device.active_config_descriptor()?;

    for interface in config.interfaces() {
        for descriptor in interface.descriptors() {
            let index = if let Some(idx) = descriptor.description_string_index() {
                idx
            } else {
                continue;
            };

            let iface_num = descriptor.interface_number();
            log::trace!("Checking interface {iface_num}");

            let name = if let Ok(name) =
                handle.read_string_descriptor(lang, index, Duration::from_millis(500))
            {
                name
            } else {
                continue;
            };

            log::trace!("Checking {}", name);

            if name == usbd_reboot::USB_INTERFACE_NAME {
                let endpoint = descriptor
                    .endpoint_descriptors()
                    .next()
                    .ok_or(anyhow::anyhow!("Could not read endpoint descriptors"))?
                    .address();

                log::debug!("Found endpoint {name} (interface #{iface_num}, endpoint #{endpoint})");

                return Ok((handle, endpoint));
            }
        }
    }

    Err(anyhow::anyhow!("Could not find USB Reboot Bulk endpoint"))
}
