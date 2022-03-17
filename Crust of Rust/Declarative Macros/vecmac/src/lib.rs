#[macro_export]
macro_rules! avec {
    ($($elem: expr), *) => {{
        const C: usize = $crate::count!(@COUNT; $($elem: expr), *);

        #[allow(unused_mut)]
        let mut v = Vec::with_capacity(C);
        $(v.push($elem);)*
        v
    }};
    ($($elem: expr,) *) => {
        $crate::avec![$($elem), *]
    };
    ($elem: expr; $count: expr) => {{
        let mut v = Vec::new();
        v.resize($count, $elem);
        v
    }}
}

// see: https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length
// counting the element to avoid expensice grow of vector capacity
#[macro_export]
macro_rules! count {
    (@SUBST; $_elem: expr) => { () };   // use () to make sure there is no allocation
    (@COUNT; $($elem: expr), *) => {
        <[()]>::len(&[$(count!(@SUBST; $elem)), *])
        // [$(count!(@SUBST; $elem)), *].len::<[()]>()
    }
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

#[test]
fn clone() {
    let v: Vec<u32> = avec![5; 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 5);
    assert_eq!(v[1], 5);
}

#[test]
fn clone_option() {
    let mut x = Some(5);
    let v: Vec<u32> = avec![x.take().unwrap(); 2];
    assert!(!v.is_empty());
    assert_eq!(v.len(), 2);
    assert_eq!(v[0], 5);
    assert_eq!(v[1], 5);
}

/// ```compile_fail
/// # should not allowed not Clone type to became a Vec by using clone
/// struct Foo;
/// let v: Vec<Foo> = vecmac::avec![Foo; 2];
/// ```
#[allow(dead_code)]
struct CompileFailClone;
