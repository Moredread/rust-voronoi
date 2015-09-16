use mpfr::mpfr::Mpfr;
use std::ops::{Add, Sub, Mul};

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

#[derive(Clone)]
pub struct Mat3<N> {
    pub m11: N, pub m21: N, pub m31: N,
    pub m12: N, pub m22: N, pub m32: N,
    pub m13: N, pub m23: N, pub m33: N,
}

#[derive(Clone)]
pub struct Mat4<N> {
    pub m11: N, pub m21: N, pub m31: N, pub m41: N,
    pub m12: N, pub m22: N, pub m32: N, pub m42: N,
    pub m13: N, pub m23: N, pub m33: N, pub m43: N,
    pub m14: N, pub m24: N, pub m34: N, pub m44: N,
}

impl<N> Mat4<N>
{
    fn new(m11: N,  m21: N,  m31: N,  m41: N,
           m12: N,  m22: N,  m32: N,  m42: N,
           m13: N,  m23: N,  m33: N,  m43: N,
           m14: N,  m24: N,  m34: N,  m44: N,
      ) -> Mat4<N> {
        Mat4 {
           m11: m11,  m21: m21,  m31: m31,  m41: m41,
           m12: m12,  m22: m22,  m32: m32,  m42: m42,
           m13: m13,  m23: m23,  m33: m33,  m43: m43,
           m14: m14,  m24: m24,  m34: m34,  m44: m44,
         }
    }
}

impl<N> Mat3<N>
{
    fn new(m11: N,  m21: N,  m31: N,
           m12: N,  m22: N,  m32: N,
           m13: N,  m23: N,  m33: N,
      ) -> Mat3<N> {
        Mat3 {
           m11: m11,  m21: m21,  m31: m31,
           m12: m12,  m22: m22,  m32: m32,
           m13: m13,  m23: m23,  m33: m33,
         }
    }
}


trait Det<N> {
    fn det(&self) -> N;
}

impl<N> Det<N> for Mat3<N>
    where N: Clone +
             Add<N, Output = N> +
             Sub<N, Output = N> +
             Mul<N, Output = N>
{
    fn det(&self) -> N {
        let p1 = self.m11.clone() * self.m22.clone() * self.m33.clone();
        let p2 = self.m21.clone() * self.m32.clone() * self.m13.clone();
        let p3 = self.m31.clone() * self.m12.clone() * self.m23.clone();
        let n1 = self.m31.clone() * self.m22.clone() * self.m13.clone();
        let n2 = self.m21.clone() * self.m12.clone() * self.m33.clone();
        let n3 = self.m11.clone() * self.m32.clone() * self.m23.clone();

        return p1 + p2 + p3 - n1 - n2 - n3;
    }
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
    use super::Det;
    use mpfr::mpfr::Mpfr;

    #[test]
    fn it_works() {
        let p1 = Mpfr::from(1.0);
        let p2 = Mpfr::from(2.0);
        let p3 = Mpfr::from(3.0);
        let v = Mat3::<Mpfr>::new(p1.clone(), p2.clone(), p3.clone(),
                                  p2.clone(), p3.clone(), p1.clone(),
                                  p3.clone(), p1.clone(), p2.clone(),);
        let d: Mpfr = v.det();
        assert_eq!(d.to_string(), Mpfr::from(-18.0).to_string());
    }
}
