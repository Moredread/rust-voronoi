use libc::{c_double, c_void};
use std::cmp::{Ordering};
use std::sync::{Once, ONCE_INIT};

static EXACTINIT: Once = ONCE_INIT;

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    x: c_double,
    y: c_double,
}

impl Point2D {
    pub fn new(x: c_double, y: c_double) -> Point2D {
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
    pub fn new(x: c_double, y: c_double, z: c_double) -> Point3D {
        Point3D { x: x, y: y, z: z }
    }
}

#[link(name = "predicates")]
extern "C" {
    fn exactinit() -> c_void;
    fn orient2d(pa: *const Point2D, pb: *const Point2D, pc: *const Point2D) -> c_double;
    fn incircle(pa: *const Point2D, pb: *const Point2D, pc: *const Point2D, pd: *const Point2D) -> c_double;
    fn orient3d(pa: *const Point3D, pb: *const Point3D, pc: *const Point3D, pd: *const Point3D) -> c_double;
    fn insphere(pa: *const Point3D, pb: *const Point3D, pc: *const Point3D, pd: *const Point3D, pe: *const Point3D) -> c_double;
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

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    Positive,
    Negative
}

impl Orientation {
    pub fn to_f64_multiplier(&self) -> f64 {
        match *self {
            Orientation::Positive => {  1.0 },
            Orientation::Negative => { -1.0 }
        }
    }
}

pub trait InCircleTestable<P> {
    fn in_circle_test(&self, point: &P) -> Option<InCircleLocation>;
}

pub trait Orientable {
    fn orientation(&self) -> Option<Orientation>;
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

fn det_to_in_circle_location(det: f64) -> Option<InCircleLocation> {
    match 0.0.partial_cmp(&det) {
        Some(Ordering::Greater) => { Some(InCircleLocation::Inside) }
        Some(Ordering::Less) => { Some(InCircleLocation::Outside) }
        Some(Ordering::Equal) => { Some(InCircleLocation::On) }
        None => { None }
    }
}

fn det_to_orientation(det: f64) -> Option<Orientation> {
    match 0.0.partial_cmp(&det) {
        Some(Ordering::Greater) => { Some(Orientation::Positive) }
        Some(Ordering::Less) => { Some(Orientation::Negative) }
        _ => { None }
    }
}

impl Orientable for Triangle<Point2D> {
    fn orientation(&self) -> Option<Orientation> {
        init_predicates();

        let orientation_det = unsafe { orient2d(&self.p1, &self.p2, &self.p3) };

        det_to_orientation(orientation_det)
    }
}

impl InCircleTestable<Point2D> for Triangle<Point2D> {
    fn in_circle_test(&self, point: &Point2D) -> Option<InCircleLocation> {
        init_predicates();

        let orienation_multiplier: f64 = match self.orientation() {
            Some(p) => { p.to_f64_multiplier() },
            None => { return None; }
        };

        let incircle_det = unsafe { incircle(&self.p1, &self.p2, &self.p3, point) };

        det_to_in_circle_location(orienation_multiplier * incircle_det)
    }
}

impl Orientable for Tetrahedron<Point3D> {
    fn orientation(&self) -> Option<Orientation> {
        init_predicates();

        let orientation_det = unsafe { orient3d(&self.p1, &self.p2, &self.p3, &self.p4) };

        det_to_orientation(orientation_det)
    }
}

impl InCircleTestable<Point3D> for Tetrahedron<Point3D> {
    fn in_circle_test(&self, point: &Point3D) -> Option<InCircleLocation> {
        init_predicates();

        let orienation_multiplier: f64 = match self.orientation() {
            Some(p) => { p.to_f64_multiplier() },
            None => { return None; }
        };

        let incircle_det = unsafe { insphere(&self.p1, &self.p2, &self.p3, &self.p4, point) };

        det_to_in_circle_location(orienation_multiplier * incircle_det)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::f64;

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
    fn in_circle_2d_nan_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::NAN);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_nan_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::NAN);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_infinity_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::INFINITY);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_infinity_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::INFINITY);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_neg_infinity_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::NEG_INFINITY);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_neg_infinity_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::NEG_INFINITY);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle { p1: p1, p2: p2, p3: p3 };

        assert_eq!(t.in_circle_test(&d), None);
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
