use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ZoomaError {
    UnsupportedEnvironment,
    MissingXdgSessionType,
    MissingXdgCurrentDesktop,
    InvalidXdgSessionType(Box<str>),
    MissingDependency(Box<str>),
}

impl fmt::Display for ZoomaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZoomaError::MissingXdgSessionType => {
                return write!(f, "Failed to read $XDG_SESSION_TYPE environment variable");
            }
            ZoomaError::InvalidXdgSessionType(session_value) => {
                return write!(
                    f,
                    "Invalid $XDG_SESSION_TYPE, expected \
                    \"x11\" or \"wayland\", got \"{}\"",
                    session_value
                );
            }
            ZoomaError::MissingDependency(dep) => {
                return write!(f, "Missing dependency: \'{}\'", dep);
            }
            ZoomaError::UnsupportedEnvironment => {
                return write!(f, "The running environment is currently unsupported");
            }
            ZoomaError::MissingXdgCurrentDesktop => {
                return write!(
                    f,
                    "Failed to read $XDG_CURRENT_DESKTOP, please \
                    make sure it is set correctly"
                );
            }
        }
    }
}

impl Error for ZoomaError {}
