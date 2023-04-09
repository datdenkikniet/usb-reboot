use std::time::Duration;

use rusb::{DeviceHandle, GlobalContext};

use crate::UsbId;

pub fn find_reboot_endpoint(
    usb_id: UsbId,
) -> Result<(DeviceHandle<GlobalContext>, u8), anyhow::Error> {
    let UsbId { vid, pid } = usb_id;

    let device = rusb::devices()?
        .iter()
        .find(|device| {
            let descriptor = device.device_descriptor().unwrap();

            descriptor.vendor_id() == vid && descriptor.product_id() == pid
        })
        .ok_or(anyhow::anyhow!(
            "Could not find device with correct VID:PID"
        ))?;

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

            let name = handle.read_string_descriptor(lang, index, Duration::from_millis(500))?;

            if name == usbd_reboot::USB_INTERFACE_NAME {
                let endpoint = descriptor
                    .endpoint_descriptors()
                    .next()
                    .ok_or(anyhow::anyhow!("Could not read endpoint descriptors"))?
                    .address();

                return Ok((handle, endpoint));
            }
        }
    }

    Err(anyhow::anyhow!("Could not find USB Bulk reboot endpoint"))
}
