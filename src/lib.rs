pub struct I32Vector {
    pub x: i32,
    pub y: i32,
}

impl I32Vector {
    pub fn new(a: i32, b: i32) -> Self {
        return Self { x: a, y: b };
    }
}

pub enum Environment {
    X11,
    Wayland,
    Windows,
}
