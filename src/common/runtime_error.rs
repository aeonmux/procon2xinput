#[derive(Debug)]
pub enum RuntimeError {
    Io(std::io::Error),
    Uinput(uinput::Error),
    Usb(rusb::Error),
    Custom(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::Io(e) => write!(f, "I/O error: {e}"),
            RuntimeError::Uinput(e) => write!(f, "uinput error: {e}"),
            RuntimeError::Usb(e) => write!(f, "USB error: {e}"),
            RuntimeError::Custom(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RuntimeError::Io(e) => Some(e),
            RuntimeError::Uinput(e) => Some(e),
            RuntimeError::Usb(e) => Some(e),
            RuntimeError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<uinput::Error> for RuntimeError {
    fn from(e: uinput::Error) -> Self {
        Self::Uinput(e)
    }
}
impl From<rusb::Error> for RuntimeError {
    fn from(e: rusb::Error) -> Self {
        Self::Usb(e)
    }
}

