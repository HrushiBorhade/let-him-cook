use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    let mut int_vec = vec![1, 2, 15, 7, 10, 24];
    int_vec.sort();
    println!("Sorted Int Vec : {:#?}", int_vec);

    let a = 5.0;
    let b = f64::NAN;

    let comparison = match a.partial_cmp(&b) {
        Some(Ordering::Less) => "a < b",
        Some(Ordering::Greater) => "a > b",
        Some(Ordering::Equal) => "a == b",
        None => "none",
    };
    let result = format!("Comparing {a} to {b} => {:#?}", comparison);
    println!("{}", result);

    let mut float_vec = vec![1.1, 2.5, 2.2, 4.02, 5.07];

    float_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("sorted float vec: {:#?}", float_vec);

    let p1 = Point::new(2, 2);
    let p2 = Point::new(1, 1);

    let mut point_vec = vec![p1, p2];

    // Sorting points based on their Euclidean distance from the origin
    point_vec.sort_by(|point1, point2| {
        let dist1 = ((point1.x.pow(2) + point1.y.pow(2)) as f64).sqrt();
        let dist2 = ((point2.x.pow(2) + point2.y.pow(2)) as f64).sqrt();
        dist1.partial_cmp(&dist2).unwrap()
    });

    println!("Sorted Points: {:#?}", point_vec);
}
