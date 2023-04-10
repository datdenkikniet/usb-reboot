#![no_std]

pub const USB_INTERFACE_NAME: &str = "USB Reboot Trigger Interface";

pub const REBOOT_MAGIC: [u8; 4] = 0xDABED000u32.to_be_bytes();

#[cfg(feature = "usb-device")]
mod usb_class;
#[cfg(feature = "usb-device")]
pub use usb_class::RebootClass;
