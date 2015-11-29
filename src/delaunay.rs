use geometry::*;
use in_circle::*;

pub struct Delaunay<P> {
    triangles: Vec<Triangle<P>>,
    domain: Triangle<P>
}

trait Triangulation<P> {
    fn triangles(&self) -> Vec<Triangle<P>>;
}

impl<P> Triangulation<P> for Delaunay<P> where
   Triangle<P>: Clone {
    fn triangles(&self) -> Vec<Triangle<P>> {
        self.triangles.clone()
    }
}

impl<P> Delaunay<P> where
   Triangle<P>: Clone {
    pub fn new(t: Triangle<P>) -> Option<Delaunay<P>> {
        Some(Delaunay { triangles: vec!(t.clone()), domain: t.clone() })
    }

    pub fn domain(&self) -> Triangle<P> {
        self.domain.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::*;
    use in_circle::*;

    use delaunay::Triangulation;

    use quickcheck::{TestResult, quickcheck, QuickCheck, StdGen};

    #[test]
    fn new_delaunay_test() {
        fn new_delaunay_test(pnt1: (f64, f64), pnt2: (f64, f64), pnt3: (f64, f64)) -> TestResult {
            let p1 = Point2D::new(pnt1.0, pnt1.1);
            let p2 = Point2D::new(pnt2.0, pnt2.1);
            let p3 = Point2D::new(pnt3.0, pnt3.1);

            let t = Triangle::new(p1, p2, p3);
            let d = Delaunay::new(t).unwrap();

            TestResult::from_bool( d.domain() == t &&
                                   d.triangles() == vec!(t) )
        }
        quickcheck(new_delaunay_test as fn(pnt1: (f64, f64), pnt2: (f64, f64), pnt3: (f64, f64)) -> TestResult)
    }
}
