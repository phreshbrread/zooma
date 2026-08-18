use std::env;
use std::process::Command;

pub const TMP_SS_PATH: &str = "/tmp/zooma.png";

pub struct I32Vector {
    pub x: i32,
    pub y: i32,
}

pub enum Environment {
    X11,
    Wayland,
    Windows,
}

impl I32Vector {
    pub fn new(a: i32, b: i32) -> Self {
        return Self { x: a, y: b };
    }
}

pub fn determine_environment() -> Environment {
    let e = env::var("XDG_SESSION_TYPE").expect("Failed to read $XDG_SESSION_TYPE");

    match e.as_str() {
        "x11" => return Environment::X11,
        "wayland" => return Environment::Wayland,
        _ => todo!(),
    }
}

pub fn take_screenshot(e: Environment) {
    match e {
        Environment::X11 => {
            Command::new("scrot")
                .args(["-Z", "0", TMP_SS_PATH, "-o"])
                .output()
                .expect("Failed to execute scrot");
        }
        // TODO: Change screenshot method since this only works on wlroots
        Environment::Wayland => {
            let grim = Command::new("grim")
                .args(["-l", "0", TMP_SS_PATH])
                .output()
                .expect("Failed to execute grim");

            if !grim.status.success() {
                println!("grim: {}", String::from_utf8_lossy(&grim.stderr));
                todo!("Handle grim failure")
            }
        }
        Environment::Windows => {
            todo!("Windows");
        }
    }
}
