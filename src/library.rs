use std::env;
use std::error::Error;
use std::process::Command;

use crate::zooma_error::ZoomaError;

pub const TMP_SS_PATH: &str = "/tmp/zooma.png";

pub struct I32Vector {
    pub x: i32,
    pub y: i32,
}

pub enum DisplayProtocol {
    X11,
    Wayland,
}

impl I32Vector {
    pub fn new(a: i32, b: i32) -> Self {
        return Self { x: a, y: b };
    }
}

pub fn get_current_environment() -> Result<DisplayProtocol, ZoomaError> {
    let e = match env::var("XDG_SESSION_TYPE") {
        Ok(o) => o,
        Err(_) => return Err(ZoomaError::NoXdgSessionType),
    };

    match e.as_str() {
        "x11" => return Ok(DisplayProtocol::X11),
        "wayland" => return Ok(DisplayProtocol::Wayland),
        _ => todo!("Handle non-x11 or wayland values"),
    }
}

pub fn take_screenshot() -> Result<(), ZoomaError> {
    let e = get_current_environment()?;

    match e { // Returns nothing on success, but the error on failure
        DisplayProtocol::X11 => {
            Command::new("scrot")
                .args(["-Z", "0", TMP_SS_PATH, "-o"])
                .output();

            // TODO: On fail
            return Err(ZoomaError::MissingDependency(String::from("scrot")));

            return Ok(());
        }

        DisplayProtocol::Wayland => {
            let grim = Command::new("grim")
                .args(["-l", "0", TMP_SS_PATH])
                .output().unwrap();

            if !grim.status.success() {
                return Err(ZoomaError::NoWlroots);
            }

            return Ok(());
        }
    }
}
