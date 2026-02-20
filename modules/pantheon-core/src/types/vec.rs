macro_rules! impl_vec {
    ($name:ident, $($field:ident),+) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq)]
        pub struct $name<T> {
            $(pub $field: T),+
        }

        impl<T> $name<T> {
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }

            
            pub fn dot(self, rhs: Self) -> T
                where T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default,
            {
                $(self.$field * rhs.$field +)+ T::default()
            }
        }

        impl<T: std::fmt::Debug> std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name))
                    $( .field(&self.$field) )+
                    .finish()
            }
        }

        impl<T: std::ops::Neg<Output = T>> std::ops::Neg for $name<T> {
            type Output = Self;
            fn neg(self) -> Self {
                Self { $($field: self.$field.neg()),+ }
            }
        }

        impl<T: std::ops::Add<Output = T>> std::ops::Add for $name<T> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self { $($field: self.$field + other.$field),+ }
            }
        }
    
        impl<T: std::ops::Sub<Output = T>> std::ops::Sub for $name<T> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self { $($field: self.$field - other.$field),+ }
            }
        }

        impl $name<f32> {
            pub fn length(self: Self) -> f32 {
                ($(self.$field * self.$field +)+ 0.0).sqrt()
            }
        }
    }
}

impl_vec!(Vec2, x, y);
impl_vec!(Vec3, x, y, z);
impl_vec!(Vec4, x, y, z, w);

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

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;
pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4f = Vec4<f32>;
pub type Vec4i = Vec4<i32>;
pub type Vec4u = Vec4<u32>;

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
    fn vec2_neg() {
        let vec = -Vec2f::new(-1.0, 1.0);
        assert_eq!(vec.x, 1.0);
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

    #[test]
    fn vec2_dot() {
        let vec1 = Vec2f::new(5.0, 2.0);
        let vec2 = Vec2f::new(3.0, 2.0);
        assert_eq!(vec1.dot(vec2), (5.0 * 3.0) + (2.0 * 2.0))
    }
}
