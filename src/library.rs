use std::process::{self, Command};
use std::{env, io::ErrorKind, path::PathBuf, sync::OnceLock};

use crate::zooma_error::ZoomaError;

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

// --- Temporary screenshot path (global) -------------------------------
// Path is set this way so it's platform-agnostic, and so that main.rs can also access it.
// Could probably be better, but I lack the knowledge at the moment.
pub static TMP_SS_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn set_temp_ss_path() -> Result<(), PathBuf> {
    TMP_SS_PATH.set(PathBuf::from(std::env::temp_dir().join("zooma.png")))?;
    return Ok(());
}
// ----------------------------------------------------------------------

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
                .args([
                    "-Z",
                    "0",
                    &TMP_SS_PATH.get().unwrap().to_string_lossy(),
                    "-o",
                ])
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
                    .args([
                        "-b",
                        "-n",
                        "-o",
                        &TMP_SS_PATH.get().unwrap().to_string_lossy(),
                    ])
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

            let cmd = Command::new("grim")
                .args(["-l", "0", &TMP_SS_PATH.get().unwrap().to_string_lossy()])
                .output();

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

pub fn handle_zooma_error(ze: ZoomaError) -> ! {
    match ze {
        ZoomaError::MissingXdgSessionType => {
            println!("Failed to read $XDG_SESSION_TYPE environment variable");
            process::exit(1);
        }
        ZoomaError::InvalidXdgSessionType(session_value) => {
            println!(
                "Invalid $XDG_SESSION_TYPE, expected \
                \"x11\" or \"wayland\", got \"{:}\"",
                session_value
            );
            process::exit(1);
        }
        ZoomaError::MissingDependency(dep) => {
            println!("Missing dependency: \'{:}\'", dep);
            process::exit(1);
        }
        ZoomaError::UnsupportedEnvironment => {
            println!("The running environment is currently unsupported");
            process::exit(1);
        }
        ZoomaError::MissingXdgCurrentDesktop => {
            println!(
                "Failed to read $XDG_CURRENT_DESKTOP, please \
                make sure it is set correctly"
            );
            process::exit(1);
        }
    }
}
