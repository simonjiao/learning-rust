fn sort_integer_vector() {
    let mut v = vec![1, 5, 10, 2, 15];

    let mut v1 = v.clone();
    v1.sort_unstable(); //faster
    v.sort();

    assert_eq!(v1, vec![1, 2, 5, 10, 15]);
    assert_eq!(v1, v);
}

fn sort_float() {
    let mut v = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    let mut v1 = v.clone();
    //v1.sort(); //the trait bound `{float}: Ord` is not satisfied
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v1.sort_by(|a, b| b.partial_cmp(a).unwrap());

    assert_eq!(v, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
    assert_eq!(v1, vec![5.5, 2.0, 1.15, 1.123, 1.1]);
    //assert_eq!(v, v1);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn sort_struct() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_owned(), 3),
        Person::new("John".to_string(), 1),
    ];

    // sort people by natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_owned(), 1),
            Person::new("John".to_owned(), 3),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    //sort by people age
    people.sort_by(|a, b| a.age.partial_cmp(&b.age).unwrap());

    assert_eq!(
        people,
        vec![
            Person::new("John".to_owned(), 1),
            Person::new("John".to_owned(), 3),
            Person::new("Zoe".to_string(), 25),
            Person::new("Al".to_string(), 60),
        ]
    );

    //sort by people age descendly
    people.sort_by(|a, b| b.age.partial_cmp(&a.age).unwrap());

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_owned(), 3),
            Person::new("John".to_owned(), 1),
        ]
    );
}

fn main() {
    sort_integer_vector();

    sort_float();

    sort_struct();
}
