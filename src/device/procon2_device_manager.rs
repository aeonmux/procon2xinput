use crate::common::RuntimeError;
use crate::serial::UsbPipe;
use crate::serial::nintendo_serial_device_details;
use evdev::Device;
use hidapi::HidApi;
use rusb::{DeviceHandle, GlobalContext, UsbContext};
use std::{fs, io};
use std::{thread, time::Duration};
const PROCON2_CONTROLLER_EVDEV_NAME_HINT: &str = "Nintendo Pro Controller";

pub struct ProCon2DeviceManager {}

impl ProCon2DeviceManager {
    pub fn get_procon2_device() -> io::Result<Device> {
        let procon2_device_name_hint = PROCON2_CONTROLLER_EVDEV_NAME_HINT.to_lowercase();
        for maybe_input_device in fs::read_dir("/dev/input")? {
            let input_device = maybe_input_device?;
            let input_device_filename = input_device.file_name();
            if !input_device_filename.to_string_lossy().starts_with("event") {
                continue;
            }
            let input_device_path = input_device.path();
            if let Ok(input_device) = Device::open(&input_device_path) {
                if let Some(device_name) = input_device.name() {
                    if device_name
                        .to_lowercase()
                        .contains(&procon2_device_name_hint)
                    {
                        return Ok(input_device);
                    }
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Switch 2 Controller Not Found",
        ))
    }

    pub fn grab(device: &mut Device) -> io::Result<()> {
        device
            .grab()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn init_hid_output() -> Result<(), RuntimeError> {
        let mut pipe = UsbPipe::open_usb_pipe(
            &nintendo_serial_device_details::NINTENDO_SWITCH_2_DEVICE_PIDS,
            &nintendo_serial_device_details::VID_NINTENDO,
        )?;

        UsbPipe::usb_write(
            &mut pipe,
            nintendo_serial_device_details::COMMAND_INIT_HID_OUTPUT,
        )?;

        UsbPipe::usb_write(
            &mut pipe,
            nintendo_serial_device_details::COMMAND_SET_PLAYER_ONE_LED,
        )?;

        Ok(())
    }
}
