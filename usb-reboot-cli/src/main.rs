mod usb;
use clap::Parser;
use usb::find_reboot_endpoint;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct UsbId {
    vid: u16,
    pid: u16,
}

fn bruh(input: &str) -> Result<UsbId, String> {
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
    #[clap(value_parser = bruh)]
    pub usb_id: UsbId,
}

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();

    let (handle, endpoint) = find_reboot_endpoint(opts.usb_id)?;

    handle
        .write_bulk(
            endpoint,
            &0xDEAD_BEEFu32.to_be_bytes(),
            Duration::from_millis(500),
        )
        .unwrap();

    println!("Wrote restart command");

    Ok(())
}
