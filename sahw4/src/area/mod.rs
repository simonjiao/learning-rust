pub use circle::Circle;
pub use rectangle::Square;
pub use shape::Shape;
pub use triangle::Triangle;

pub fn area<T>(shape: &T) -> f64
where
    T: Shape + std::fmt::Debug,
{
    shape.area()
}

mod shape {
    pub trait Shape {
        fn area(&self) -> f64;

        fn valid(&self) -> bool {
            true
        }
    }
}

mod circle {
    use super::shape::*;
    use std::f64::consts::PI;

    #[derive(Debug)]
    pub struct Circle(f64);

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Self(radius)
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            PI * self.0 * self.0
        }

        fn valid(&self) -> bool {
            self.0 > 0f64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn circle_case() {
            let c = Circle::new(1f64);
            assert!(c.valid());
            assert_eq!(c.area(), PI * 1f64 * 1f64);

            assert!(!Circle::new(0f64).valid());
        }
    }
}

mod triangle {
    use super::shape::*;

    #[derive(Debug)]
    pub struct Triangle(f64, f64, f64);

    impl Triangle {
        pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
            Self(e0, e1, e2)
        }
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            let hp = (self.0 + self.1 + self.2) / 2f64;

            (hp * (hp - self.0) * (hp - self.1) * (hp - self.2)).sqrt()
        }

        fn valid(&self) -> bool {
            let v0 = self.0 + self.1 > self.2;
            let v1 = self.0 + self.2 > self.1;
            let v2 = self.1 + self.2 > self.0;

            let v3 = self.0 > 0f64 && self.1 > 0f64 && self.2 > 0f64;

            v0 && v1 && v2 && v3
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn triangle_case() {
            assert!(!Triangle::new(0f64, 0f64, 0f64).valid());
            assert!(!Triangle::new(1f64, 2f64, 0f64).valid());

            assert!(!Triangle::new(1f64, 2f64, 3f64).valid());
            assert!(!Triangle::new(1f64, 3f64, 2f64).valid());
            assert!(!Triangle::new(2f64, 3f64, 1f64).valid());
            assert!(!Triangle::new(2f64, 3f64, 1f64).valid());
            assert!(!Triangle::new(3f64, 2f64, 1f64).valid());
            assert!(!Triangle::new(3f64, 1f64, 2f64).valid());

            let x = Triangle::new(3f64, 4f64, 5f64);
            assert!(x.valid());
            assert_eq!(x.area(), 6f64);
        }
    }
}

mod rectangle {
    use super::shape::*;

    #[derive(Debug)]
    pub struct Square(f64);

    impl Square {
        pub fn new(e: f64) -> Self {
            Self(e)
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.0 * self.0
        }

        fn valid(&self) -> bool {
            self.0 > 0f64
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn square_case() {
            assert!(!Square::new(0f64).valid());

            let x = Square::new(10f64);
            assert!(x.valid());
            assert_eq!(x.area(), 100f64);
        }
    }
}
