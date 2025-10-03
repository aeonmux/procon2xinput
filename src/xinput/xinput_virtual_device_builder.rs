use crate::serial::xinput_serial_device_details;
use uinput::device::Builder;
use uinput::device::Device as UiDevice;
use uinput::event::Event;
use uinput::event::absolute::Absolute;
use uinput::event::absolute::Hat;
use uinput::event::absolute::Hat::*;
use uinput::event::absolute::Position;
use uinput::event::absolute::Position::*;
use uinput::event::controller::{Controller, GamePad};

const V_INPUT_BUTTONS: &[GamePad] = &[
    GamePad::South,  // B
    GamePad::East,   // A
    GamePad::North,  // X
    GamePad::West,   // Y
    GamePad::Select, // Minus
    GamePad::Start,  // Plus
    GamePad::TL,     // Left Trigger
    GamePad::TR,     // Right Trigger
    GamePad::TL2,    // Left Bumper
    GamePad::TR2,    // Right Bumper
    GamePad::Mode,   // Home
    GamePad::Z,      // Record
];

const V_INPUT_AXIS: &[Position] = &[
    Position::X,  // Left Thumbstick X
    Position::Y,  // Left Thumbstick Y
    Position::RX, // Right Thumbstick X
    Position::RY, // Right Thumbstick Y
];

const V_INPUT_DPAD: &[Hat] = &[
    Hat::X0, // DPad-X Axis
    Hat::Y0, // DPad-Y Axis
];

const CONTROLLER_NAME: &str = "Switch 2 Pro Controller [X-Input]";

pub struct VirtualXInputDeviceBuilder {}

impl VirtualXInputDeviceBuilder {
    pub fn build() -> std::result::Result<UiDevice, uinput::Error> {
        let mut builder: Builder = uinput::default()?;

        builder = builder.name(CONTROLLER_NAME)?;

        builder = builder.vendor(xinput_serial_device_details::XBOX_VENDOR_ID);
        builder = builder.product(xinput_serial_device_details::XBOX_360_CONTROLLER_PRODUCT_ID);

        for axis in V_INPUT_AXIS {
            builder = builder.event(Event::Absolute(Absolute::Position(*axis)))?;
        }

        for dpad_button in V_INPUT_DPAD {
            builder = builder.event(Event::Absolute(Absolute::Hat(*dpad_button)))?;
        }

        for button in V_INPUT_BUTTONS {
            builder = builder.event(Event::Controller(Controller::GamePad(*button)))?;
        }

        builder.create()
    }
}
