fn main() {
    let v1 = vec![1, 4, 10, 2, 7, 3];
    println!("result is {}", largest(&v1));
    let v2 = vec!['c', 'z', 'a', 'y', 'q', 'e'];
    println!("result is {}", largest(&v2));
    let v3 = vec![
        "banana",
        "apple",
        "waterlemon",
        "orangle",
        "lemon",
        "blackberry",
    ];
    println!("result is {}", largest(&v3));

    let v4 = vec![
        String::from("banana"),
        String::from("apple"),
        String::from("waterlemon"),
        String::from("orangle"),
        String::from("lemon"),
        String::from("blackberry"),
    ];
    println!("result is {}", largest(&v4));
}

fn largest<T>(v: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut max = &v[0];
    for item in v.iter() {
        if item > max {
            max = item;
        }
    }
    max
}
