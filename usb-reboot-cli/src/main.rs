mod usb;
use clap::Parser;
use usb::find_reboot_endpoint;

use std::time::Duration;

use usbd_reboot::REBOOT_MAGIC;

#[derive(Debug, Clone, PartialEq)]
pub struct UsbId {
    vid: u16,
    pid: u16,
}

impl core::fmt::Display for UsbId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}:{:04X}", self.vid, self.pid)
    }
}

fn parse_usb_id(input: &str) -> Result<UsbId, String> {
    let mut parts = input.split(':');

    if parts.clone().count() != 2 {
        return Err(format!("Malformed VID:PID \"{}\"", input));
    }

    let (vid, pid) = (parts.next().unwrap(), parts.next().unwrap());

    let vid = if let Ok(vid) = u16::from_str_radix(vid, 16) {
        vid
    } else {
        return Err(format!("Malformed VID \"{vid}\""));
    };

    let pid = if let Ok(pid) = u16::from_str_radix(pid, 16) {
        pid
    } else {
        return Err(format!("Malformed PID \"{pid}\""));
    };

    Ok(UsbId { vid, pid })
}

/// A program that reboots your RP2040 into the USB bootloader.
///
/// Note that this requires the RP2040 to run an endpoint with the class provided by
/// `rp2040-reboot-usb`.
#[derive(clap::Parser)]
pub struct Options {
    /// The USB ID (given as `VID:PID`, with PID and VID as hex values)
    /// on which the RP2040 Reboot Interface present.
    #[clap(value_parser = parse_usb_id)]
    pub usb_id: UsbId,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or("info".into()))
        .init();

    let opts = Options::parse();

    let usb_id = opts.usb_id;

    log::trace!("Parsed USB ID {usb_id}");

    log::info!("Finding USB Device");

    let (handle, endpoint) = find_reboot_endpoint(usb_id)?;

    log::info!("Writing USB Reboot Trigger");

    handle
        .write_bulk(endpoint, &REBOOT_MAGIC, Duration::from_millis(500))
        .unwrap();

    log::info!("Wrote restart command");

    Ok(())
}
