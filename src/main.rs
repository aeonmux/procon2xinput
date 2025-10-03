mod common;
mod device;
mod serial;
mod xinput;
use common::RuntimeError;
use device::ProCon2DeviceManager;
use xinput::XInputProxy;
use xinput::xinput_virtual_device_builder::VirtualXInputDeviceBuilder;

fn main() -> std::result::Result<(), RuntimeError> {
    // Get the physical device
    let mut procon2_device = ProCon2DeviceManager::get_procon2_device()?;
    ProCon2DeviceManager::init_hid_output();
    // Tell everyone else that the original physical device is not to be used
    let _ = ProCon2DeviceManager::grab(&mut procon2_device);

    // Create a virtual xinput device
    let mut virtual_xinput_device = VirtualXInputDeviceBuilder::build()?;

    // Proxy the outputs from the procon2 to the virtual xinput device
    XInputProxy::proxy_inputs(&mut procon2_device, &mut virtual_xinput_device)
}
