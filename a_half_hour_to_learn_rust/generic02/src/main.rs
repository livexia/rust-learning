// struct generic
fn main() {
    println!("Hello, world!");
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1);
    print_type_name(&p2);

    // Vec is generic
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1);
    print_type_name(&v2);

    // vec literals
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false];
    print_type_name(&v1);
    print_type_name(&v2);
    
}

struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}
