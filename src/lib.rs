pub mod zooma_error;
use zooma_error::ZoomaError;

use std::{
    env,
    io::ErrorKind,
    path::PathBuf,
    process::{self, Command},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct I32Vector2 {
    pub x: i32,
    pub y: i32,
}

pub enum DisplayProtocol {
    X11,
    Wayland,
}

impl I32Vector2 {
    pub fn new(a: i32, b: i32) -> Self {
        return Self { x: a, y: b };
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
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

pub fn take_screenshot(ss_path: &PathBuf) -> Result<(), ZoomaError> {
    let env = get_current_environment()?;

    match env {
        // --- X11 ----------------------------------------------------
        DisplayProtocol::X11 => {
            let cmd = Command::new("scrot")
                .args(["-Z", "0", &ss_path.to_string_lossy(), "-o"])
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
                    .args(["-b", "-n", "-o", &ss_path.to_string_lossy()])
                    .output();

                match cmd {
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            return Err(ZoomaError::MissingDependency("spectacle".into()));
                        }
                        _ => panic!("Unhandled spectacle error: {:#?}", e.kind()),
                    },
                    Ok(_) => return Ok(()),
                }
            }

            if current_desktop == "GNOME" {
                let cmd = Command::new("flameshot")
                    .args(["full", "-p", &ss_path.to_string_lossy()])
                    .output();

                match cmd {
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => {
                            return Err(ZoomaError::MissingDependency("flameshot".into()));
                        }
                        _ => panic!("Unhandled screenshot error: {:#?}", e.kind()),
                    },
                    Ok(_) => return Ok(()),
                }
            }

            let cmd = Command::new("grim")
                .args(["-l", "0", &ss_path.to_string_lossy()])
                .output();

            match cmd {
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        return Err(ZoomaError::MissingDependency("grim".into()));
                    }
                    _ => panic!("Unhandled grim error: {:#?}", e.kind()),
                },
                Ok(_) => return Ok(()),
            }

            // Safe to unwrap here because this only executes if cmd succeeded
            if !cmd.unwrap().status.success() {
                return Err(ZoomaError::UnsupportedEnvironment);
            }

            return Ok(());
        } // ------------------------------------------------------------
    }
}
