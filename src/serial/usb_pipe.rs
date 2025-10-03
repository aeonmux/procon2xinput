use crate::RuntimeError;
use rusb::{DeviceHandle, GlobalContext, UsbContext};
use std::{thread, time::Duration};

pub struct UsbPipe {
    serial_device_handle: DeviceHandle<GlobalContext>,
    serial_interface: u8,
    serial_tx: u8,
    serial_rx: u8,
}

impl UsbPipe {
    pub fn open_usb_pipe(pids: &[u16], vid: &u16) -> Result<UsbPipe, RuntimeError> {
        // Try each PID until one opens
        let (mut handle, _pid) = pids
            .iter()
            .find_map(|&pid| rusb::open_device_with_vid_pid(*vid, pid).map(|h| (h, pid)))
            .ok_or_else(|| RuntimeError::Custom("No Device Found Matching PIDs & VID".into()))?;

        let serial_interface_number = 1u8;

        if handle
            .kernel_driver_active(serial_interface_number)
            .unwrap_or(false)
        {
            let _ = handle.detach_kernel_driver(serial_interface_number);
        }

        handle.claim_interface(serial_interface_number)?;

        let serial_device = handle.device();
        let _device_descriptor = serial_device.device_descriptor()?;
        let configuration_description = serial_device.config_descriptor(0)?;

        let mut serial_tx = None;
        let mut serial_rx = None;

        for iface in configuration_description.interfaces() {
            for if_desc in iface.descriptors() {
                if if_desc.interface_number() != serial_interface_number {
                    continue;
                }
                for ep in if_desc.endpoint_descriptors() {
                    if ep.transfer_type() == rusb::TransferType::Bulk {
                        match ep.direction() {
                            rusb::Direction::Out => serial_tx = Some(ep.address()),
                            rusb::Direction::In => serial_rx = Some(ep.address()),
                        }
                    }
                }
            }
        }

        let serial_tx = serial_tx
            .ok_or_else(|| RuntimeError::Custom("No Serial WRITE Endpoint Found".into()))?;
        let serial_rx = serial_rx
            .ok_or_else(|| RuntimeError::Custom("No Serial READ  Endpoint Found".into()))?;

        Ok(UsbPipe {
            serial_device_handle: handle,
            serial_interface: serial_interface_number,
            serial_tx,
            serial_rx,
        })
    }

    pub fn usb_write(pipe: &mut UsbPipe, data: &[u8]) -> Result<usize, RuntimeError> {
        let wrote = pipe.serial_device_handle.write_bulk(
            pipe.serial_tx,
            data,
            Duration::from_millis(200),
        )?;

        thread::sleep(Duration::from_millis(10));
        Ok(wrote)
    }
}
