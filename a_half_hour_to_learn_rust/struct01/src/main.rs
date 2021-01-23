fn main() {
    // declare struct
    struct Vec2 {
        x: f64,
        y: f64,
    }

    // initialized using struct literals
    let v1 = Vec2{x: 1.0, y: 2.0};
    let v2 = Vec2{y: 2.0, x: 4.0};
    //the order does not matter
    let x = 1.0;
    let y = 2.0;
    let v3 = Vec2{y, x};

    println!("v1, x = {}, y = {}", v1.x, v1.y);
    println!("v2, x = {}, y = {}", v2.x, v2.y);
    println!("v3, x = {}, y = {}", v3.x, v3.y);

    // shortcut
    let v3 = Vec2 {
        x: 3.0,
        ..v2 // no comma
    };
    println!("v3, x = {}, y = {}", v3.x, v3.y);

    // another shortcut
    let v4 = Vec2{ ..v3 };
    println!("v4, x = {}, y = {}", v4.x, v4.y);

    // destructured struct
    let Vec2 { x, y } = v3;
    println!("x = {}, y = {}", x, y);

    // destructured struct and throw
    let Vec2 {x, ..} = v1;
    println!("x = {}", x);
}
