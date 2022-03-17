#[macro_export]
macro_rules! avec {
    ($($elem: expr), *) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
    ($($elem: expr,) *) => {
        $crate::avec!($($elem), *)
    };
}

#[test]
fn empty() {
    let v: Vec<u32> = avec![];
    assert!(v.is_empty());
}

#[test]
fn single() {
    let v: Vec<u32> = avec![5];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 1);
    assert_eq!(v[0], 5);
}

#[test]
fn double() {
    let v: Vec<u32> = avec![5, 6];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 5);
    assert_eq!(v[1], 6);
}

#[test]
fn tailing() {
    let v: Vec<u32> = avec![5, 6,];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 5);
    assert_eq!(v[1], 6);
}

/// ```compile_fail
/// # should not allowed [,] to became a Vec
/// let v: Vec<u32> = vecmac::avec![,];
/// ```
#[allow(dead_code)]
struct CompileFailEmptyTailing;
