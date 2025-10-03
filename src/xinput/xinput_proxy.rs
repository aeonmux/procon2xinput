use crate::common::RuntimeError;
use evdev::Device;
use evdev::EventType;
use uinput::device::Device as UiDevice;
use uinput::event::absolute::Hat::*;
use uinput::event::absolute::Position::*;
use uinput::event::controller::{Controller, GamePad};

const PHYSICAL_LEFT_THUMBSTICK_X_AXIS_EVENT_CODE: i32 = 0; // ABS_X
const PHYSICAL_LEFT_THUMBSTICK_Y_AXIS_EVENT_CODE: i32 = 1; // ABS_Y
const PHYSICAL_RIGHT_THUMB_STICK_X_AXIS_EVENT_CODE: i32 = 3; // ABS_RX
const PHYSICAL_RIGHT_THUMBSTICK_Y_AXIS_EVENT_CODE: i32 = 5; // ABS_RZ

const D_PAD_ANALOG_RELEASED: i32 = 0;
const D_PAD_ANALOG_POS: i32 = 1;
const D_PAD_ANALOG_NEG: i32 = -1;

const GAMEPAD_KEY_EVENT_CODE_MAP: &[(i32, GamePad)] = &[
    (304, GamePad::South), // A
    (305, GamePad::East),  // B
    (307, GamePad::West),  // X
    (306, GamePad::North), // Y
    (318, GamePad::Select),
    (310, GamePad::Start),
    (316, GamePad::TL),
    (317, GamePad::TL2),
    (308, GamePad::TR),
    (309, GamePad::TR2),
    (704, GamePad::Mode),
    (705, GamePad::Z),
];

const KEY_DPAD_UP: i32 = 312;
const KEY_DPAD_RIGHT: i32 = 313;
const KEY_DPAD_LEFT: i32 = 314;
const KEY_DPAD_DOWN: i32 = 315;

pub struct XInputProxy {}

impl XInputProxy {
    pub fn proxy_inputs(
        from_device: &mut Device,
        to_device: &mut UiDevice,
    ) -> Result<(), RuntimeError> {
        loop {
            for event in from_device.fetch_events()? {
                match event.event_type() {
                    EventType::ABSOLUTE => {
                        Self::handle_thumbstick_event(
                            event.code().into(),
                            event.value(),
                            to_device,
                        )?;
                    }
                    EventType::KEY => {
                        Self::handle_key_event(event.code().into(), event.value(), to_device)?;
                    }
                    _ => {}
                }
            }
            to_device.synchronize()?;
        }
    }

    fn handle_key_event(
        code: i32,
        value: i32,
        to_device: &mut UiDevice,
    ) -> Result<(), RuntimeError> {
        let is_key_pressed = value != 0;
        Self::handle_gamepad_key_event(code, is_key_pressed, to_device)?;
        Self::handle_dpad_key_event(code, is_key_pressed, to_device)?;
        Ok(())
    }

    fn handle_gamepad_key_event(
        code: i32,
        pressed: bool,
        to_device: &mut UiDevice,
    ) -> Result<(), RuntimeError> {
        if let Some(btn) = Self::get_gamepad_key_event(code) {
            let key = Controller::GamePad(btn);
            if pressed {
                to_device.press(&key)?;
            } else {
                to_device.release(&key)?;
            }
        }
        Ok(())
    }

    fn handle_thumbstick_event(
        event_code: i32,
        event_value: i32,
        to_device: &mut UiDevice,
    ) -> Result<(), RuntimeError> {
        if let Some((axis, invert)) = Self::get_thumbstick_event(event_code) {
            let norm = Self::normalize_analog_thumbstick_value(event_value, invert);
            to_device.position(axis, norm)?;
        }
        Ok(())
    }

    fn handle_dpad_key_event(
        key_event_code: i32,
        is_pressed: bool,
        to_device: &mut UiDevice,
    ) -> Result<(), RuntimeError> {
        if let Some((axis, direction)) = Self::get_dpad_key_event(key_event_code) {
            let value = if is_pressed {
                direction
            } else {
                D_PAD_ANALOG_RELEASED
            };
            to_device.position(axis, value)?;
        }
        Ok(())
    }

    fn get_thumbstick_event(
        code: i32,
    ) -> Option<(&'static uinput::event::absolute::Position, bool)> {
        match code {
            PHYSICAL_LEFT_THUMBSTICK_X_AXIS_EVENT_CODE => Some((&X, false)),
            PHYSICAL_LEFT_THUMBSTICK_Y_AXIS_EVENT_CODE => Some((&Y, true)),
            PHYSICAL_RIGHT_THUMB_STICK_X_AXIS_EVENT_CODE => Some((&RX, false)),
            PHYSICAL_RIGHT_THUMBSTICK_Y_AXIS_EVENT_CODE => Some((&RY, true)),
            _ => None,
        }
    }

    fn get_dpad_key_event(code: i32) -> Option<(&'static uinput::event::absolute::Hat, i32)> {
        match code {
            KEY_DPAD_UP => Some((&Y0, D_PAD_ANALOG_POS)),
            KEY_DPAD_DOWN => Some((&Y0, D_PAD_ANALOG_NEG)),
            KEY_DPAD_RIGHT => Some((&X0, D_PAD_ANALOG_POS)),
            KEY_DPAD_LEFT => Some((&X0, D_PAD_ANALOG_NEG)),
            _ => None,
        }
    }

    fn get_gamepad_key_event(key_event_code: i32) -> Option<GamePad> {
        GAMEPAD_KEY_EVENT_CODE_MAP
            .iter()
            .find(|(gamepad_key_code, _)| *gamepad_key_code == key_event_code)
            .map(|(_, v)| *v)
    }

    // scale raw 0..4095 (center ~2048) to int16
    fn normalize_analog_thumbstick_value(current_thumbstick_value: i32, invert: bool) -> i32 {
        let center = 2048.0;
        let span = 4095.0;
        let mut normalized_value = (current_thumbstick_value as f64 - center) / (span / 2.0);
        if normalized_value.abs() < 0.03 {
            normalized_value = 0.0;
        }
        if normalized_value < -1.0 {
            normalized_value = -1.0;
        }
        if normalized_value > 1.0 {
            normalized_value = 1.0;
        }
        if invert {
            normalized_value = -normalized_value;
        }
        (normalized_value * 32767.0).round() as i32
    }
}
