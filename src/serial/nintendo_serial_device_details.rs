pub const VID_NINTENDO: u16 = 0x057E;
pub const PID_JOYCON2_R: u16 = 0x2066;
pub const PID_JOYCON2_L: u16 = 0x2067;
pub const PID_PROCON2: u16 = 0x2069;
pub const PID_GCNSO: u16 = 0x2073;

//Id like to get this to show up as a procon1 in steam but some sdl games dont play nice with that
pub const SWITCH_PROCON1_VENDOR: u16 = 0x057E;
pub const SWITCH_PROCON1_PRODUCT: u16 = 0x2009;

//Eventually we could support things like the NSO Gamecube controller
pub const NINTENDO_SWITCH_2_DEVICE_PIDS: &[u16] =
    &[PID_JOYCON2_R, PID_JOYCON2_L, PID_PROCON2, PID_GCNSO];

pub const COMMAND_INIT_HID_OUTPUT: &[u8] = &[
    0x03, 0x91, 0x00, 0x0d, 0x00, 0x08, 0x00, 0x00, 0x01, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

pub const COMMAND_SET_PLAYER_ONE_LED: &[u8] = &[
    0x09, 0x91, 0x00, 0x07, 0x00, 0x08, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
