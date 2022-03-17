#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($key: expr => $value: expr) => {{
        let mut map = ::std::collections::HashMap::new();
        map.insert($key, $value);
        map
    }};
}

#[test]
fn empty_hashmap() {
    use std::collections::HashMap;
    let map: HashMap<u32, u32> = hashmap! {};
    assert!(map.is_empty());
}

#[test]
fn sigle_hashmap() {
    use std::collections::HashMap;
    let map: HashMap<u32, u32> = hashmap! {1 => 2};
    assert!(!map.is_empty());
    assert_eq!(map.len(), 1);
    assert_eq!(map[&1], 2);
}
