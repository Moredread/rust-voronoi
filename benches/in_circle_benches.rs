#![feature(test)]

extern crate test;
extern crate voronoi;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use voronoi::in_circle::{Triangle, Point2D, InCircleTestable};

    #[bench]
    fn bench_triangle_in_circle_test(b: &mut Bencher) {
        let t = Triangle::new(
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(1.0, 1.0),
        );

        let test_point = Point2D::new(0.0, 0.0);

        b.iter(|| {
            let t_box = black_box(&t);
            let test_box = black_box(&test_point);

            t_box.in_circle_test(test_box)
        });
    }
}
