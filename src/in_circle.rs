use std::ops::{Add, Sub, Mul};
use libc::{c_char, c_int, c_ulong, c_long, c_double, c_void};
use std::ffi::*;
use std::cmp::{Ordering};
use std::slice;
use std::sync::{Once, ONCE_INIT};

static EXACTINIT: Once = ONCE_INIT;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    x: c_double,
    y: c_double,
}

impl Point2D {
    fn new(x: c_double, y: c_double) -> Point2D {
        Point2D { x: x, y: y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D {
    x: c_double,
    y: c_double,
    z: c_double,
}

impl Point3D {
    fn new(x: c_double, y: c_double, z: c_double) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }
}

#[link(name = "predicates")]
extern "C" {
    fn exactinit() -> c_void;
    fn insphere(pa: *const Point3D, pb: *const Point3D, pc: *const Point3D, pd: *const Point3D, pe: *const Point3D) -> c_double;
    fn incircle(pa: *const Point2D, pb: *const Point2D, pc: *const Point2D, pd: *const Point2D) -> c_double;
}

fn init_predicates() {
    EXACTINIT.call_once(|| {
        unsafe { exactinit() };
    });
}

#[derive(Debug, Eq, PartialEq)]
pub enum InCircleLocation {
    Inside,
    Outside,
    On
}

pub trait InCircleTestable<P> {
    fn in_circle_test(&self, point: &P) -> Option<InCircleLocation>;
}

pub struct Triangle<P> {
    p1: P,
    p2: P,
    p3: P,
}

pub struct Tetrahedron<P> {
    p1: P,
    p2: P,
    p3: P,
    p4: P,
}

impl InCircleTestable<Point2D> for Triangle<Point2D> {
    fn in_circle_test(&self, point: &Point2D) -> Option<InCircleLocation> {
        init_predicates();

        let incircle_det = unsafe { incircle(&self.p1, &self.p2, &self.p3, point) };

        match 0.0.partial_cmp(&incircle_det) {
            Some(Ordering::Greater) => { Some(InCircleLocation::Inside) }
            Some(Ordering::Less) => { Some(InCircleLocation::Outside) }
            Some(Ordering::Equal) => { Some(InCircleLocation::On) }
            None => { None }
        }
    }
}

impl InCircleTestable<Point3D> for Tetrahedron<Point3D> {
    fn in_circle_test(&self, point: &Point3D) -> Option<InCircleLocation> {
        init_predicates();

        let incircle_det = unsafe { insphere(&self.p1, &self.p2, &self.p3, &self.p4, point) };

        match 0.0.partial_cmp(&incircle_det) {
            Some(Ordering::Less) => { Some(InCircleLocation::Inside) }
            Some(Ordering::Greater) => { Some(InCircleLocation::Outside) }
            Some(Ordering::Equal) => { Some(InCircleLocation::On) }
            None => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{TestResult, quickcheck};
    use std::f64::consts::{SQRT_2};

    #[test]
    fn in_circle_2d() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);
        let d_inside = Point2D::new(0.5, 0.5);
        let d_outside = Point2D::new(2.0, 0.0);
        let d_on = Point2D::new(1.0, 1.0);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d_inside), Some(InCircleLocation::Inside));
        assert_eq!(t.in_circle_test(&d_outside), Some(InCircleLocation::Outside));
        assert_eq!(t.in_circle_test(&d_on), Some(InCircleLocation::On));
    }

    #[test]
    fn in_circle_3d() {
        let p1 = Point3D::new(-1.0,  1.0, -1.0);
        let p2 = Point3D::new( 1.0,  1.0, -1.0);
        let p3 = Point3D::new( 0.0, -1.0, -1.0);
        let p4 = Point3D::new( 0.0,  0.0,  1.0);
        let d_inside = Point3D::new(0.0, 0.0, 0.0);
        let d_outside = Point3D::new(10.0, 10.0, 10.0);
        let d_on = Point3D::new(0.0, 0.0, 1.0);

        let t = Tetrahedron { p1: p1, p2: p2, p3: p3, p4: p4 };

        assert_eq!(t.in_circle_test(&d_inside), Some(InCircleLocation::Inside));
        assert_eq!(t.in_circle_test(&d_outside), Some(InCircleLocation::Outside));
        assert_eq!(t.in_circle_test(&d_on), Some(InCircleLocation::On));
    }

}
