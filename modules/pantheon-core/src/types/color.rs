#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r: {}, g: {}, b: {}, a: {})", self.r, self.g, self.b, self.a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Color {
    pub const fn new(r:f32, g:f32, b:f32, a:f32) -> Self {
        Color {r, g, b, a}
    }

    pub fn approx_eq(self, other: Self) -> bool {
        const EPS: f32 = 1e-6;
        (self.r - other.r).abs() < EPS &&
        (self.g - other.g).abs() < EPS &&
        (self.b - other.b).abs() < EPS &&
        (self.a - other.a).abs() < EPS
    }
}

#[allow(dead_code)]
pub mod colors {
    use super::*;

    pub static TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    pub static BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub static WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    pub static GRAY: Color = Color::new(0.5, 0.5, 0.5, 1.0);
    pub static DIM_GRAY: Color = Color::new(0.25, 0.25, 0.25, 1.0);
    pub static DARK_GRAY: Color = Color::new(0.1, 0.1, 0.1, 1.0);

    pub static RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub static GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub static BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub static YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub static CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub static MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);

    pub static WINE: Color = Color::new(0.5, 0.0, 0.0, 1.0);
    pub static FOREST: Color = Color::new(0.0, 0.5, 0.0, 1.0);
    pub static MARINE: Color = Color::new(0.0, 0.0, 0.5, 1.0);

    // Others
    pub static NEON_ORANGE: Color = Color::new(1.0, 0.6470588, 0.0, 1.0);
    pub static LIGHT_MAGENTA: Color = Color::new(1.0, 0.5, 1.0, 1.0);
    pub static BABY_PINK: Color = Color::new(1.0, 0.70588235294, 1.0, 1.0);
    pub static AZURE: Color = Color::new(0.0, 0.5, 1.0, 1.0);
}
