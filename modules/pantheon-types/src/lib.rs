pub mod color;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl<T> std::ops::Mul<T> for Vec2<T>
    where T: Copy + std::ops::Mul<Output = T>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T> std::ops::Div<T> for Vec2<T>
    where T: Copy + std::ops::Div<Output = T>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}



impl Vec2<f32> {
    pub fn length(self: Self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec2_init() {
        let vec = Vec2f::default();
        assert_eq!(vec.x, 0.0);
        assert_eq!(vec.y, 0.0);

        let vec = Vec2f::new(-1.0, -1.0);
        assert_eq!(vec.x, -1.0);
        assert_eq!(vec.y, -1.0);
    }

    #[test]
    fn vec2_addsub() {
        let vec1 = Vec2f::new(3.0, -3.0);
        let vec2 = Vec2f::new(1.0, 1.0);
        let vec = vec1 + vec2;
        assert_eq!(vec.x, 4.0);
        assert_eq!(vec.y, -2.0);
        
        let vec1 = Vec2f::new(3.0, -3.0);
        let vec2 = Vec2f::new(1.0, 1.0);
        let vec = vec1 - vec2;
        assert_eq!(vec.x, 2.0);
        assert_eq!(vec.y, -4.0);
    }
    
    #[test]
    fn vec2_scalar() {
        let vec = Vec2f::new(3.0, -3.0);
        let vec = vec * 2.0;
        assert_eq!(vec.x, 6.0);
        assert_eq!(vec.y, -6.0);
        let vec = vec / 2.0;
        assert_eq!(vec.x, 3.0);
        assert_eq!(vec.y, -3.0);
    }
    
    #[test]
    fn vec2_eq() {
        let vec1 = Vec2f::new(3.0, -3.0);
        let vec2 = Vec2f::new(3.0, -3.0);
        let vec3 = Vec2f::new(2.0, 2.0);
        assert_eq!(vec1, vec2);
        assert_ne!(vec2, vec3);
    }
}
