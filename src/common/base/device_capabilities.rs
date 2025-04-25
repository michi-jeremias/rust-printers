use crate::common::traits::platform::PlatformDeviceCapabilitiesGetters;

#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub bin_count: u64,
    pub bin_names: Vec<String>,
}

impl DeviceCapabilities {
    pub(crate) fn from_platform_device_capabilities_getters(
        platform_device_capabilities: &dyn PlatformDeviceCapabilitiesGetters,
    ) -> DeviceCapabilities {
        let device_capabilities = DeviceCapabilities {
            bin_count: platform_device_capabilities.get_bin_count(),
            bin_names: platform_device_capabilities.get_bin_names(),
        };

        return device_capabilities;
    }
}
