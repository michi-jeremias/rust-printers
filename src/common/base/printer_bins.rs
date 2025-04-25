use std::fmt::{Debug, Error, Formatter};

pub struct DeviceCapabilities {
    pub dc_bins: u8,
    pub dc_binnames: Vec<String>,
}

impl Debug for DeviceCapabilities {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt,
            "DeviceCapabilities {{
                \r  dc_bins: {:?},
                \r  dc_binnames: {:?},
            \r}}]",
            self.dc_bins, self.dc_binnames,
        )
    }
}

impl Clone for DeviceCapabilities {
    fn clone(&self) -> Self {
        return DeviceCapabilities {
            dc_bins: self.dc_bins.clone(),
            dc_binnames: self.dc_binnames.clone(),
        };
    }
}

impl DeviceCapabilities {
    pub(crate) fn from_platform_device_capabilities(
        platform_dc_bins: u8,
        platform_dc_binnames: Vec<String>,
    ) -> Self {
        return DeviceCapabilities {
            dc_bins: platform_dc_bins,
            dc_binnames: platform_dc_binnames,
        };
    }
}
