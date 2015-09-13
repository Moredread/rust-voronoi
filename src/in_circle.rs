use na::{Vec3, Rot3, Rotation};
use mpfr::MPFR;

enum InCircleLocation {
    Inside,
    Outside,
    On
}

trait InCircle<P> {
    fn inCircleTest(&self, point: &P) -> InCircleLocation;
}

struct Point2D {
    x: f64,
    y: f64,
}

pub struct Triangle<P> {
    p1: P,
    p2: P,
    p3: P,
}


#[cfg(test)]
mod tests {
    use super::*;
    use mpfr::MPFR;
    use na::Vec3;

    #[test]
    fn it_works() {
        let p1 = MPFR::from_float(1.0);
    
        assert_eq!(1.0f64, 1.0f64);
    }
}
