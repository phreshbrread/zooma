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

impl Error for ZoomaError {}

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

#[derive(Debug)]
pub enum SettingsError {
    Io(std::io::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}

impl Error for SettingsError {}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SettingsError::Io(e) => return write!(f, "IO error: {}", e),
            SettingsError::TomlDe(e) => return write!(f, "{}", e),
            SettingsError::TomlSer(e) => return write!(f, "{}", e),
        }
    }
}

impl From<std::io::Error> for SettingsError {
    fn from(value: std::io::Error) -> Self {
        return Self::Io(value);
    }
}
impl From<toml::de::Error> for SettingsError {
    fn from(value: toml::de::Error) -> Self {
        return Self::TomlDe(value);
    }
}
impl From<toml::ser::Error> for SettingsError {
    fn from(value: toml::ser::Error) -> Self {
        return Self::TomlSer(value);
    }
}
