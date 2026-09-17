pub mod custom_errors;
use custom_errors::*;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{ErrorKind, Write},
    path::PathBuf,
    process::Command,
};

// TODO: Customizable keybinds
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub zoom_multiplier: f32,
    pub pan_multiplier: f32,
}

impl UserSettings {
    pub fn default() -> Self {
        return Self {
            zoom_multiplier: 1.0,
            pan_multiplier: 1.0,
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

pub fn get_config_path() -> PathBuf {
    let path = PathBuf::from(
        env::home_dir()
            .unwrap() // $HOME should always be set on Linux
            .join(match env::var("XDG_CONFIG_HOME") {
                Ok(o) => o,
                Err(_) => {
                    println!("Failed to read $XDG_CONFIG_HOME, defaulting to ~/.config");
                    String::from(".config")
                }
            })
            .join("zooma")
            .join("settings.toml"),
    );

    return path;
}

pub fn get_user_settings(config_path: PathBuf) -> Result<UserSettings, SettingsError> {
    let mut user_settings = UserSettings::default();

    // If the path exists, try to read the file
    // If not, write defaults to a new file
    if config_path.try_exists()? {
        let config_file = fs::read_to_string(config_path)?;
        user_settings = toml::from_str(&config_file)?;
    } else {
        // Create parent directories first
        let p = config_path.parent().unwrap();
        fs::create_dir_all(p)?;

        let settings_as_toml = toml::to_string(&user_settings)?;
        let mut file = fs::File::create(config_path)?;
        file.write_all(&settings_as_toml.as_bytes())?;
    }

    return Ok(user_settings);
}
