/*
see: https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html
*/

#[macro_export]
macro_rules! using_a {
    ($a:ident, $e:expr) => {{
        let $a = 42;
        $e
    }};
    ($e:expr) => {{ // fail
        let a = 42;
        $e
    }};
}

#[test]
fn it_work() {
    let four = using_a!(a, a / 10);
    // let four = using_a!(a / 10); // fail
    assert_eq!(four, 4);
}


///```compile_fail
/// let four = using_a!(a / 10);
/// ```

#[allow(dead_code)]
struct HygieneFail;