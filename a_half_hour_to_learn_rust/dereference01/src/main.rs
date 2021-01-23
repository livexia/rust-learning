fn main() {
    println!("Hello, world!");

    // use * to dereference, but can only do it if the type is Copy
    let p = Point{ x: 1.0, y: 2.0 };
    let p_ref = &p;
    println!("{:?}", negate(*p_ref));

}

#[derive(Debug, Clone, Copy)] // Copy needed because dereference, Clone needed because Copy
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
    }
}
