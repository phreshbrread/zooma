pub mod zooma_error;
use serde::{Deserialize, Serialize};
use std::{env, io::ErrorKind, path::PathBuf, process::Command};
use zooma_error::ZoomaError;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    zoom_multiplier: f32,
}

impl UserSettings {
    pub fn default() -> Self {
        return Self {
            zoom_multiplier: 1.0,
        };
    }
}

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
#[cfg(target_os = "windows")]
pub fn take_screenshot(ss_path: &PathBuf) -> Result<(), ZoomaError> {
    return Ok(());
}

#[cfg(target_os = "linux")]
pub fn take_screenshot(ss_path: &PathBuf) -> Result<(), ZoomaError> {
    let env = get_current_environment()?;

    match env {
        DisplayProtocol::X11 => {
            return run_screenshot_command(
                "scrot",
                vec!["-Z", "0", &ss_path.to_string_lossy(), "-o"],
            );
        }

        DisplayProtocol::Wayland => {
            let current_desktop = match env::var("XDG_CURRENT_DESKTOP") {
                Ok(o) => o,
                Err(_) => return Err(ZoomaError::MissingXdgCurrentDesktop),
            };

            if current_desktop == "KDE" {
                return run_screenshot_command(
                    "spectacle",
                    vec!["-b", "-n", "-o", &ss_path.to_string_lossy()],
                );
            }

            if current_desktop == "GNOME" {
                return run_screenshot_command(
                    "flameshot",
                    vec!["full", "-p", &ss_path.to_string_lossy()],
                );
            }

            // For non-GNOME / KDE environments we can just use grim
            return run_screenshot_command("grim", vec!["-l", "0", &ss_path.to_string_lossy()]);
        }
    }
}

pub fn run_screenshot_command(cmd: &str, args: Vec<&str>) -> Result<(), ZoomaError> {
    let output = Command::new(cmd).args(args).output();

    match output {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                return Err(ZoomaError::MissingDependency(cmd.into()));
            }
            _ => panic!("Unhandled {} error: {:#?}", cmd, e.kind()),
        },
        Ok(_) => return Ok(()),
    }
}
