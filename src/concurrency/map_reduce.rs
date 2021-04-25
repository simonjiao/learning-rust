use rayon::prelude::*;

struct Person {
    age: u32,
}

pub fn map_reduce() {
    let v = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&n| n.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|x| *x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|x| *x > 30).sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!(
        "The average age of people older than 30 is {} {}",
        avg_over_30, num_over_30
    );
}
