#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($elem: expr) => {{
        let mut v = Vec::new();
        v.push($elem);
        v
    }};
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
