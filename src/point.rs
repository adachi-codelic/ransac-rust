use crate::color::Color;
/// Point3D is a struct that represents a point in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
    xyz: [f64; 3],
    id: i64,
    color: Color,
}
impl Point3D {
    fn new(x: f64, y: f64, z: f64, id: i64, color: Color) -> Point3D {
        Point3D {
            x,
            y,
            z,
            xyz: [x, y, z],
            id,
            color,
        }
    }
    fn new_without_color(x: f64, y: f64, z: f64, id: i64) -> Point3D {
        Point3D {
            x,
            y,
            z,
            xyz: [x, y, z],
            id,
            color: (0, 0, 0),
        }
    }
}
impl AsRef<[f64]> for Point3D {
    fn as_ref(&self) -> &[f64] {
        &self.xyz
    }
}
