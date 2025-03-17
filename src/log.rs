use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use crate::device::Device;

#[derive(Debug)]
pub struct Log<'a> {
    device: &'a Device,
    rsx_log: File,
    xmd_log: File,
    log_data: HashMap<&'static str, String>
}

impl Log<'_> {
    pub fn from_device(device: &Device) -> Result<Log, Box<dyn Error>> {
        let logging_folder = format!("./log/{}", device.serial_number);

        let rsx_log_path = format!("./log/{}/{}_rsx.log", device.serial_number, device.serial_number);
        let xmd_log_path = format!("./log/{}/{}_xmd_log", device.serial_number, device.serial_number);

        fs::create_dir_all(&logging_folder)?;

        File::create(&rsx_log_path)?;
        File::create(&xmd_log_path)?;

        let rsx_log = File::open(&rsx_log_path)?;
        let xmd_log = File::open(&xmd_log_path)?;
        let log_data = HashMap::<&'static str, String>::new();

        Ok(
            Log {
                device,
                rsx_log,
                xmd_log,
                log_data
            }
        )

    }
}
