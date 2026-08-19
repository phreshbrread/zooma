use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ZoomaError {
    NoWlroots,
    NoXdgSessionType,
    MissingDependency(String),
}
impl fmt::Display for ZoomaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}
impl Error for ZoomaError {}
