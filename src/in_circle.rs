use mpfr::mpfr::Mpfr;
use std::ops::{Add, Sub, Mul};
use mat::{Mat4};

enum InCircleLocation {
    Inside,
    Outside,
    On
}

trait InCircle<P> {
    fn in_circle_test(&self, point: &P) -> InCircleLocation;
}

#[derive(Debug, Clone, PartialEq)]
struct Point2D {
    x: f64,
    y: f64,
}

pub struct Triangle<P> {
    p1: P,
    p2: P,
    p3: P,
}


impl InCircle<Point2D> for Triangle<Point2D> {
    fn in_circle_test(&self, point: &Point2D) -> InCircleLocation {
        let mat = Mat4::new(self.p1.x, self.p1.y, self.p1.x*self.p1.x + self.p1.y*self.p1.y, 1f64,
                            self.p2.x, self.p2.y, self.p2.x*self.p2.x + self.p2.y*self.p2.y, 1f64,
                            self.p3.x, self.p3.y, self.p3.x*self.p3.x + self.p3.y*self.p3.y, 1f64,
                            point.x, point.y, point.x * point.x + point.y * point.y, 1f64);
        return InCircleLocation::Inside;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat3_det_mpfr() {
    }
}
