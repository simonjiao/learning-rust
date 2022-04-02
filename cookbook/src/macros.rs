#![allow(unused)]

use std::ops::{Add, Mul, Sub};

// This is a simple macro named `say_hello`
#[macro_export]
macro_rules! say_hello {
    // `()` indicates that the macro tke no argument
    () => {
        println!("hello!");
    };
}

#[macro_export]
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("This is {} calling", stringify!($func_name));
        }
    };
}

#[macro_export]
macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[macro_export]
macro_rules! maximum {
    ($left:expr, $right:expr) => {
        if $left > $right {
            $left
        } else {
            $right
        }
    };
}

#[macro_export]
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };

    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

macro_rules! assert_equal_len {
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?} dimension mismatch {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        )
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output = T> + Copy>(xs: &mut Vec<T>, ys: &[T]) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

#[cfg(test)]
mod tests {
    #[test]
    fn test_maximum() {
        assert_eq!(maximum!(30, 20), 30);
        assert_ne!(maximum!(30, 40), 30);
        //maximum!(30, "20");
    }

    use std::iter;
    macro_rules! test_op {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    test_op!(add_assign, 1u32, 2u32, 3u32);
    test_op!(mul_assign, 1u32, 2u32, 2u32);
    test_op!(sub_assign, 2u32, 1u32, 1u32);
}
