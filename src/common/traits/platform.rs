use crate::common::base::{
    device_capabilities::DeviceCapabilities,
    job::PrinterJobState,
    printer::{Printer, PrinterState},
    PrinterWithCapabilities,
};
use std::time::SystemTime;

pub trait PlatformPrinterGetters {
    fn get_name(&self) -> String;
    fn get_is_default(&self) -> bool;
    fn get_system_name(&self) -> String;
    fn get_marker_and_model(&self) -> String;
    fn get_is_shared(&self) -> bool;
    fn get_uri(&self) -> String;
    fn get_location(&self) -> String;
    fn get_state(&self) -> String;
    fn get_port_name(&self) -> String;
    fn get_processor(&self) -> String;
    fn get_description(&self) -> String;
    fn get_data_type(&self) -> String;
}

pub trait PlatformPrinterJobGetters {
    fn get_id(&self) -> u64;
    fn get_name(&self) -> String;
    fn get_state(&self) -> u64;
    fn get_printer(&self) -> String;
    fn get_media_type(&self) -> String;
    fn get_created_at(&self) -> SystemTime;
    fn get_processed_at(&self) -> Option<SystemTime>;
    fn get_completed_at(&self) -> Option<SystemTime>;
}

pub trait PlatformActions {
    fn get_printers() -> Vec<Printer>;
    fn print(
        printer_system_name: &str,
        buffer: &[u8],
        job_name: Option<&str>,
    ) -> Result<(), &'static str>;
    fn print_file(
        printer_system_name: &str,
        file_path: &str,
        job_name: Option<&str>,
    ) -> Result<(), &'static str>;
    fn get_printer_jobs(
        printer_name: &str,
        active_only: bool,
    ) -> Vec<crate::common::base::job::PrinterJob>;
    fn get_default_printer() -> Option<Printer>;
    fn get_printer_by_name(printer_name: &str) -> Option<Printer>;
    fn parse_printer_state(platform_state: &str) -> PrinterState;
    fn parse_printer_job_state(platform_state: u64) -> PrinterJobState;
    fn get_device_capabilities_by_name(printer_name: &str) -> Option<DeviceCapabilities>;
    fn get_printers_with_capabilities() -> Vec<PrinterWithCapabilities>;
}

pub trait PlatformDeviceCapabilitiesGetters {
    fn get_bin_count(&self) -> u64;
    fn get_bin_names(&self) -> Vec<String>;
}
