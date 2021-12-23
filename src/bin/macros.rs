use learning_rust::{create_function, maximum, print_result, say_hello, test};

create_function!(foo);
create_function!(bar);

#[allow(unused_macros)]
macro_rules! find_min {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr), +) => {
        std::cmp::min($x, find_min!($($y), +))
    }
}

fn main() {
    say_hello!();

    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;

        x * x + 2 * x + 1
    });

    assert_eq!(4, maximum!(2 + 1, 4));

    //test!(1u32 + 3 == 4u32; and 2u32 == 2u32);
    test!(true; or false);

    //assert_eq!(4 - 1, find_min!(2 + 1));
    //assert_eq!(1, find_min!(3, 2, 1, 4));
}
