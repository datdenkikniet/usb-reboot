use usb_device::class_prelude::*;

pub struct RebootClass<'a, B: UsbBus> {
    interface: InterfaceNumber,
    name: StringIndex,
    read_ep: EndpointOut<'a, B>,
}

impl<'a, B: UsbBus> RebootClass<'a, B> {
    pub fn new(max_packet_size: u16, alloc: &'a UsbBusAllocator<B>) -> Self {
        Self {
            interface: alloc.interface(),
            name: alloc.string(),
            read_ep: alloc.bulk(max_packet_size),
        }
    }

    #[cfg(feature = "rp2040-hal")]
    pub fn process_rp2040(&mut self, gpio_activity_pin_mask: u32, disable_interface_mask: u32) {
        if self.should_restart() {
            rp2040_hal::rom_data::reset_to_usb_boot(gpio_activity_pin_mask, disable_interface_mask);
        }
    }

    pub fn should_restart(&mut self) -> bool {
        let mut buf = [0u8; 4];
        match self.read_ep.read(&mut buf) {
            Ok(size) if size >= 4 => {
                let value = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);

                value == 0xDEADBEEF
            }
            _ => false,
        }
    }
}

impl<B: UsbBus> UsbClass<B> for RebootClass<'_, B> {
    fn get_configuration_descriptors(
        &self,
        writer: &mut DescriptorWriter,
    ) -> usb_device::Result<()> {
        writer.interface_alt(self.interface, 0, 0xFF, 0, 0, Some(self.name))?;
        writer.endpoint(&self.read_ep)?;
        Ok(())
    }

    fn get_string(&self, index: StringIndex, _lang_id: u16) -> Option<&str> {
        if index == self.name {
            Some(super::USB_INTERFACE_NAME)
        } else {
            None
        }
    }
}
