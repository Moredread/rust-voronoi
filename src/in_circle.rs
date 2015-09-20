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

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D {
    x: c_double,
    y: c_double,
    z: c_double,
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

pub trait InCircle<P> {
    fn in_circle_test(&self, point: &P) -> Option<InCircleLocation>;
}

pub struct Triangle<P> {
    p1: P,
    p2: P,
    p3: P,
}

impl InCircle<Point2D> for Triangle<Point2D> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat3_det_mpfr() {
        let p1 = Point2D { x: 0.0, y: 0.0 };
        let p2 = Point2D { x: 0.0, y: 1.0 };
        let p3 = Point2D { x: 1.0, y: 1.0 };
        let d_inside  = Point2D { x: 0.5, y: 0.5 };
        let d_outside  = Point2D { x: 2.0, y: 0.0 };
        let d_on  = Point2D { x: 1.0, y: 1.0 };

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d_inside), Some(InCircleLocation::Inside));
        assert_eq!(t.in_circle_test(&d_outside), Some(InCircleLocation::Outside));
        assert_eq!(t.in_circle_test(&d_on), Some(InCircleLocation::On));
    }
}
