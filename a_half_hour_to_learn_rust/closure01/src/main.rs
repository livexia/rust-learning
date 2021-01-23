fn main() {
    println!("Hello, world!");
    for_each_planet(|planet| println!("{}", planet));

    // borrow rule also apply
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));

    for_each_planet_static(move |planet| println!("{}, {}", greeting, planet));
    // println!("{}", greeting); // greeting moved

    foobar(|x| x * 2);

    foobar_mut(|x| x * 2);

    let mut acc = 2;
    foobar_mut(|x| {
        acc += 1;
        acc * x
    }); // 2 + 1 = 3, 3 * 2 = 6, 3 + 1 = 4, 4 * 6 = 24

    
    // let mut acc = 2;
    // foobar(|x| {
    //     acc += 1;
    //     // error[E0594]: cannot assign to `acc`, as it is a captured variable in a `Fn` closure
    //     acc * x
    // }); // 2 + 1 = 3, 3 * 2 = 6, 3 + 1 = 4, 4 * 6 = 24


    // FnOnce
    let s = String::from("alright");
    let s_clone = s.clone();
    foobar_once(move || s);
    // foobar_once(move || s); //error[E0382]: use of moved value: `s`
    foobar_once(|| s_clone.clone());
    foobar_once(|| s_clone.clone());

    foobar_two_args(32, 64, |x, y| x > y);
    foobar_two_args(128, 64, |x, y| x > y);
    // ignoring both its arguments
    // foobar_two_args(128, 64, |_, _| panic!("Comparing is futile!")); // thread 'main' panicked at 'Comparing is futile!', src\main.rs:42:37

    countdown(10, |i| println!("tick {}...", i));

    //toilet closure
    countdown(3, |_| ()); // |_| () looks like a toilet
}

fn for_each_planet<F>(f: F) 
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn for_each_planet_static<F>(f: F) 
    where F: Fn(&'static str) + 'static
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2)));
}

// fn foobar_mut<F>(mut f: F)
//     where F: FnMut(i32) -> i32
// {
//     println!("{}", f(f(2))); // error[E0499]: cannot borrow `f` as mutable more than once at a time
// }

fn foobar_mut<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let temp = f(2);
    println!("{}", f(temp));
}

fn foobar_once<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f());
    // println!("{}", f()); // error[E0382]: use of moved value: `f`
}

// closure with two arguments
fn foobar_two_args<F>(x: i32, y: i32, is_greater: F) 
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}

fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}