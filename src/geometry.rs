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
pub struct Edge<P> {
    pub p1: P,
    pub p2: P,
}

impl<P> Edge<P> {
    pub fn new(p1: P, p2: P) -> Edge<P> {
        Edge {
            p1: p1,
            p2: p2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle<P> {
    pub p1: P,
    pub p2: P,
    pub p3: P,
}

impl<P: Copy> Triangle<P> {
    pub fn new(p1: P, p2: P, p3: P) -> Triangle<P> {
        Triangle {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }

    pub fn edges(&self) -> [Edge<P>; 3] {
        [Edge::new(self.p1, self.p2), Edge::new(self.p2, self.p3), Edge::new(self.p3, self.p1)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::{TestResult, quickcheck};

    #[test]
    fn triangle_edges_test() {
        fn triangle_edges_test(pnt1: (f64, f64), pnt2: (f64, f64), pnt3: (f64, f64)) -> TestResult {
            let p1 = Point2D::new(pnt1.0, pnt1.1);
            let p2 = Point2D::new(pnt2.0, pnt2.1);
            let p3 = Point2D::new(pnt3.0, pnt3.1);

            let t = Triangle::new(p1, p2, p3);
            let edges = t.edges();

            TestResult::from_bool(edges[0] == Edge::new(p1, p2) &&
                                  edges[1] == Edge::new(p2, p3) &&
                                  edges[2] == Edge::new(p3, p1) )
        }
        quickcheck(triangle_edges_test as fn(pnt1: (f64, f64), pnt2: (f64, f64), pnt3: (f64, f64)) -> TestResult)
    }
}
