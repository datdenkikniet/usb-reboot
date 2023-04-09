#![no_std]

pub const USB_INTERFACE_NAME: &str = "USB Reboot Trigger Interface";

#[cfg(feature = "usb-device")]
mod usb_class;
#[cfg(feature = "usb-device")]
pub use usb_class::RebootClass;
