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
            _ => todo!("Implement fmt::Display for ZoomaError"),
        }
    }
}

impl Error for ZoomaError {}
