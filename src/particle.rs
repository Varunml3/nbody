#[from_env]
pub const COLOR: i32 = 0xffffff;

#[from_env]
pub const MASS: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub luminosity: f32,
    pub mass: f32,
}

impl Particle {
    pub const fn const_default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            luminosity: 1.0,
            mass: 1.0,
        }
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            luminosity: 1.0,
            mass: 1.0,
        }
    }
}

impl From<(f32, f32)> for Particle {
    fn from((x,y): (f32, f32)) -> Self {
        Self {
            x,y, ..Default::default()
        }
    }
}

