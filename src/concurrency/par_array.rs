use rayon::prelude::*;

pub fn mutate_array() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}

pub fn par_test_array() {
    let mut vec = vec![2, 4, 8, 6];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));
}

pub fn par_search_array() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
    assert_eq!(f3, Some(&9));
    //assert_eq!(f3, Some(&11));
}

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn par_sort_array() {
    let mut vec = vec![String::new(); 100_000];

    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        let f: Vec<u8> = (0..5).map(|_| rng.sample(&Alphanumeric)).collect();
        *p = String::from_utf8(f).unwrap();
    });

    vec.par_sort_unstable();

    //let mut rng = thread_rng();
    //let n = rng.gen_range(0..vec.len() - 10);
    //(n..n + 10).for_each(|i| println!("{}", vec[i]));
}
