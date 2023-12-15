use crate::color::Color;
use crate::point::Point3D;
use kdtree::KdTree;

/// Vector3D is a struct that represents a point in 3D space.
#[derive(Debug, Clone, Copy)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}
impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }
    fn normalize(&self) -> Vector3D {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length == 0.0 {
            Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            Vector3D {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    }
}

/// PointCloud is a struct that represents a collection of points in 3D space.
#[derive(Debug, Clone)]
struct PointCloud {
    points: KdTree<f64, i64, Point3D>,
}
impl PointCloud {
    fn new(points: KdTree<f64, i64, Point3D>) -> PointCloud {
        PointCloud { points }
    }
}

/// Plane is a struct that represents a plane in 3D space.
/// The plane is represented by the equation ax + by + cz + d = 0.
#[derive(Debug, Clone, Copy)]

struct Plane {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    color: Color,
}
impl Plane {
    fn new(a: f64, b: f64, c: f64, d: f64, color: Color) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
            color: color,
        }
    }
    fn new_without_color(a: f64, b: f64, c: f64, d: f64) -> Plane {
        Plane {
            a: a,
            b: b,
            c: c,
            d: d,
            color: (0, 0, 0),
        }
    }
    fn get_normal_vector(&self) -> Vector3D {
        Vector3D::new(self.a, self.b, self.c).normalize()
    }
}
struct Project {
    project_name: String,
    point_cloud: PointCloud,
}
impl Project {
    fn new(project_name: String, point_cloud: PointCloud) -> Project {
        Project {
            project_name,
            point_cloud,
        }
    }
}
