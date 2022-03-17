#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $value);)*
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
    let map: HashMap<u32, u32> = hashmap! { 1 => 2 };
    assert!(!map.is_empty());
    assert_eq!(map.len(), 1);
    assert_eq!(map[&1], 2);
}

#[test]
fn double_hashmap() {
    use std::collections::HashMap;
    let map: HashMap<u32, char> = hashmap! {
        1 => 'a',
        2 => 'b'
    };
    assert!(!map.is_empty());
    assert_eq!(map.len(), 2);
    assert_eq!(map[&1], 'a');
    assert_eq!(map[&2], 'b');
}
