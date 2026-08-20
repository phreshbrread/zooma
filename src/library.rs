use std::env;
use std::io::ErrorKind;
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
        Err(_) => return Err(ZoomaError::MissingXdgSessionType),
    };

    match e.as_str() {
        "x11" => return Ok(DisplayProtocol::X11),
        "wayland" => return Ok(DisplayProtocol::Wayland),
        _ => return Err(ZoomaError::InvalidXdgSessionType(e.into())),
    }
}

pub fn take_screenshot() -> Result<(), ZoomaError> {
    let e = get_current_environment()?;

    match e {
        // --- X11 ----------------------------------------------------
        DisplayProtocol::X11 => {
            let cmd = Command::new("scrot")
                .args(["-Z", "0", TMP_SS_PATH, "-o"])
                .output();

            match cmd {
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        return Err(ZoomaError::MissingDependency("scrot".into()));
                    }
                    _ => panic!("Unhandled scrot error: {:#?}", e.kind()),
                },
                Ok(_) => return Ok(()),
            }
        }
        // ------------------------------------------------------------

        // --- Wayland ------------------------------------------------
        // TODO: Remove duplicate code
        DisplayProtocol::Wayland => {
            let current_desktop = match env::var("XDG_CURRENT_DESKTOP") {
                Ok(o) => o,
                Err(_) => return Err(ZoomaError::MissingXdgCurrentDesktop),
            };

            if current_desktop == "KDE" {
                let cmd = Command::new("spectacle")
                    .args(["-b", "-n", "-o", TMP_SS_PATH])
                    .output();

                match cmd {
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            return Err(ZoomaError::MissingDependency("spectacle".into()));
                        }
                        _ => panic!("Unhandled spectacle error: {:#?}", e.kind()),
                    },
                    Ok(_) => (),
                }
            }

            let cmd = Command::new("grim").args(["-l", "0", TMP_SS_PATH]).output();

            match cmd {
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        return Err(ZoomaError::MissingDependency("grim".into()));
                    }
                    _ => panic!("Unhandled grim error: {:#?}", e.kind()),
                },
                Ok(_) => (),
            }

            // Safe to unwrap here because this only executes if cmd succeeded
            if !cmd.unwrap().status.success() {
                return Err(ZoomaError::UnsupportedEnvironment);
            }

            return Ok(());
        } // ------------------------------------------------------------
    }
}
