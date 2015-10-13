use libc::c_double;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: c_double,
    pub y: c_double,
}

impl Point2D {
    pub fn new(x: c_double, y: c_double) -> Point2D {
        Point2D { x: x, y: y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

impl Point3D {
    pub fn new(x: c_double, y: c_double, z: c_double) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle<P> {
    pub p1: P,
    pub p2: P,
    pub p3: P,
}

impl<P> Triangle<P> {
    pub fn new(p1: P, p2: P, p3: P) -> Triangle<P> {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tetrahedron<P> {
    pub p1: P,
    pub p2: P,
    pub p3: P,
    pub p4: P,
}

impl<P> Tetrahedron<P> {
    pub fn new(p1: P, p2: P, p3: P, p4: P) -> Tetrahedron<P> {
        Tetrahedron {
            p1: p1,
            p2: p2,
            p3: p3,
            p4: p4,
        }
    }
}
