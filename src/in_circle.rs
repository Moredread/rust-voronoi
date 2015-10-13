use geometry::*;

use libc::{c_double, c_void};
use std::cmp::{Ordering};
use std::sync::{Once, ONCE_INIT};

static EXACTINIT: Once = ONCE_INIT;

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

pub trait InCircleTestable<P> {
    fn in_circle_test(&self, point: &P) -> Option<InCircleLocation>;
}

fn det_to_in_circle_location(det: f64) -> Option<InCircleLocation> {
    match 0.0.partial_cmp(&det) {
        Some(Ordering::Greater) => { Some(InCircleLocation::Inside) }
        Some(Ordering::Less) => { Some(InCircleLocation::Outside) }
        Some(Ordering::Equal) => { Some(InCircleLocation::On) }
        None => { None }
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

#[derive(Debug, Eq, PartialEq)]
pub enum Orientation {
    Positive,
    Negative
}

pub trait Orientable {
    fn orientation(&self) -> Option<Orientation>;
}

impl Orientation {
    pub fn to_f64_multiplier(&self) -> f64 {
        match *self {
            Orientation::Positive => {  1.0 },
            Orientation::Negative => { -1.0 }
        }
    }
}

impl Orientable for Tetrahedron<Point3D> {
    fn orientation(&self) -> Option<Orientation> {
        init_predicates();

        let orientation_det = unsafe { orient3d(&self.p1, &self.p2, &self.p3, &self.p4) };

        det_to_orientation(orientation_det)
    }
}

impl Orientable for Triangle<Point2D> {
    fn orientation(&self) -> Option<Orientation> {
        init_predicates();

        let orientation_det = unsafe { orient2d(&self.p1, &self.p2, &self.p3) };

        det_to_orientation(orientation_det)
    }
}

fn det_to_orientation(det: f64) -> Option<Orientation> {
    match 0.0.partial_cmp(&det) {
        Some(Ordering::Greater) => { Some(Orientation::Positive) }
        Some(Ordering::Less) => { Some(Orientation::Negative) }
        _ => { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::*;

    use std::f64;
    use std::f64::consts;
    use rand;
    use quickcheck::{TestResult, quickcheck, QuickCheck, StdGen};

    #[test]
    fn in_circle_2d() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);
        let d_inside = Point2D::new(0.5, 0.5);
        let d_outside = Point2D::new(2.0, 0.0);
        let d_on = Point2D::new(1.0, 1.0);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d_inside), Some(InCircleLocation::Inside));
        assert_eq!(t.in_circle_test(&d_outside), Some(InCircleLocation::Outside));
        assert_eq!(t.in_circle_test(&d_on), Some(InCircleLocation::On));
    }

    #[test]
    fn in_circle_2d_triangle_constructed_from_circle() {
        fn in_circle_2d_triangle_constructed_from_circle(center: (f64, f64), radius: f64, test_point_radius: f64, angles: (f64, f64, f64, f64)) -> TestResult {
            if radius <= 0.0 { return TestResult::discard()};
            if test_point_radius <= 0.0 { return TestResult::discard()};

            fn to_pnt(center: (f64, f64), radius: f64, angle: f64) -> Point2D {
                let x = center.0 + radius * angle.cos();
                let y = center.1 + radius * angle.sin();
                Point2D::new(x, y)
            }
            let p1 = to_pnt(center, radius, angles.0);
            let p2 = to_pnt(center, radius, angles.1);
            let p3 = to_pnt(center, radius, angles.2);
            let test_point = to_pnt(center, test_point_radius, angles.3);

            let triangle = Triangle::new(p1, p2, p3);

            let expected =
                if test_point_radius < radius {
                    Some(InCircleLocation::Inside) }
                else {
                    Some(InCircleLocation::Outside)
                };
            TestResult::from_bool(triangle.in_circle_test(&test_point) == expected)
        }
        quickcheck(in_circle_2d_triangle_constructed_from_circle as fn(center: (f64, f64), radius: f64, test_point_radius: f64, angles: (f64, f64, f64, f64)) -> TestResult)
    }

    #[test]
    fn in_circle_2d_tetrahedron_constructed_from_sphere() {
        const DOMAIN: usize = 100;
        fn in_circle_2d_tetrahedron_constructed_from_sphere(
            center: (f64, f64, f64), radius: f64, test_point_radius: f64,
            angle_inputs: ((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64)),
        ) -> TestResult {
            if radius <= 0.0 { return TestResult::discard() };
            if test_point_radius <= 0.0 { return TestResult::discard() };
            if radius == test_point_radius { return TestResult::discard() };

            /// Generate a point on the sphere given by the "spherical" input coordinates.
            /// angle_inputs should be on the interval [-DOMAIN, DOMAIN], as converting to real
            /// spherical coordinates is also done by this function.
            /// See e.g. http://mathworld.wolfram.com/SpherePointPicking.html how uniform sampling
            /// on a sphere works.
            fn to_pnt(center: (f64, f64, f64), radius: f64, angle_inputs: (f64, f64)) -> Point3D {
                let domain_f64 = DOMAIN as f64;

                // Transform input to random variable on [0, 1]
                let u = 0.5 * (angle_inputs.0 / domain_f64 + 1.0);
                // Transform other input to random variable on [-1, 1]
                let v = angle_inputs.1 / domain_f64;

                let theta = 2.0 * consts::PI * u;
                let phi = v.acos();

                let x = center.0 + radius * theta.cos() * phi.sin();
                let y = center.1 + radius * theta.sin() * phi.sin();
                let z = center.2 + radius * phi.cos();

                Point3D::new(x, y, z)
            }

            let p1 = to_pnt(center, radius, angle_inputs.0);
            let p2 = to_pnt(center, radius, angle_inputs.1);
            let p3 = to_pnt(center, radius, angle_inputs.2);
            let p4 = to_pnt(center, radius, angle_inputs.3);
            let test_point = to_pnt(center, test_point_radius, angle_inputs.4);

            let tetrahedron = Tetrahedron::new(p1, p2, p3, p4);

            let expected =
                if test_point_radius < radius {
                    Some(InCircleLocation::Inside) }
                else {
                    Some(InCircleLocation::Outside)
                };
            TestResult::from_bool(tetrahedron.in_circle_test(&test_point) == expected)
        }
        QuickCheck::new().gen(StdGen::new(rand::thread_rng(), DOMAIN)).quickcheck(in_circle_2d_tetrahedron_constructed_from_sphere as fn(
            center: (f64, f64, f64), radius: f64, test_point_radius: f64,
            angle_inputs: ((f64, f64), (f64, f64), (f64, f64), (f64, f64), (f64, f64)),
        ) -> TestResult)
    }

    #[test]
    fn in_circle_2d_nan_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::NAN);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_nan_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::NAN);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_infinity_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::INFINITY);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_infinity_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::INFINITY);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_neg_infinity_in_test_point() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, 1.0);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, f64::NEG_INFINITY);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_neg_infinity_in_triangle() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(0.0, f64::NEG_INFINITY);
        let p3 = Point2D::new(1.0, 1.0);

        let d = Point2D::new(1.0, 1.0);

        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.in_circle_test(&d), None);
    }

    #[test]
    fn in_circle_2d_doubled_point_in_triangle() {
        fn in_circle_2d_doubled_point_in_triangle(p1: (f64, f64), p2: (f64, f64), p_test: (f64, f64)) -> TestResult {
            let point1 = Point2D::new(p1.0, p1.1);
            let point2 = Point2D::new(p2.0, p2.1);
            let test_point = Point2D::new(p_test.0, p_test.1);

            let t1 = Triangle::new(point1, point2, point2);
            let t2 = Triangle::new(point1, point2, point2);
            let t3 = Triangle::new(point1, point2, point2);

            // This should better be seperate tests, but I don't care right now
            TestResult::from_bool(
                    t1.in_circle_test(&test_point) == None ||
                    t2.in_circle_test(&test_point) == None ||
                    t3.in_circle_test(&test_point) == None
                )
        }
        quickcheck(in_circle_2d_doubled_point_in_triangle as fn(p1: (f64, f64), p2: (f64, f64), p_test: (f64, f64)) -> TestResult)
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

        let t = Tetrahedron::new(p1, p2, p3, p4);

        assert_eq!(t.in_circle_test(&d_inside), Some(InCircleLocation::Inside));
        assert_eq!(t.in_circle_test(&d_outside), Some(InCircleLocation::Outside));
        assert_eq!(t.in_circle_test(&d_on), Some(InCircleLocation::On));
    }

}
