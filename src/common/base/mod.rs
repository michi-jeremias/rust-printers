pub mod device_capabilities;
pub mod job;
pub mod printer;

#[derive(Debug)]
pub struct PrinterWithCapabilities {
    pub printer: printer::Printer,
    pub device_capabilities: device_capabilities::DeviceCapabilities,
}
