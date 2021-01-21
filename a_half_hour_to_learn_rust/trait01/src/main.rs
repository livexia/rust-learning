// You can implement:
// one of your traits on anyone's type
// anyone's trait on one of your types
// but not a foreign trait on a foreign type
// These are called the "orphan rules".
fn main() {
    // an implementation of our trait on our type
    let n = Number{ odd: false, value: -54 };
    println!("{}", n.is_strictly_negative());

    // trait on a foreign type
    let x = -32;
    println!("{}", x.is_strictly_negative());
    
    // A foreign trait on our type
    let y = Number{ odd: true, value: -21};
    let z = -y;
    println!("z negative? {}", z.is_strictly_negative());

}

struct Number {
    odd: bool,
    value: i32,
}

trait Signed {
    fn is_strictly_negative(self) -> bool;
}

// an implementation of our trait on our type
impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

// trait on a foreign type
impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

// A foreign trait on our type
// the `Neg` trait is used to overload `-`, the
// unary minus operator.
// 重载一员操作符“-”
impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number {
            value: -self.value,
            odd: self.odd,
        }
    }
}