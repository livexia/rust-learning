#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr), *) => {{
        const C: usize = count!(@COUNT; $($key), *);

        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::with_capacity(C);
        $(map.insert($key, $value);)*
        map
    }};
    ($($key: expr => $value: expr,)*) => {
        hashmap!{$($key => $value), *}
    };
}

// see: https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length
// counting the element to avoid expensice grow of vector capacity
#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@SUBST; $_elem: tt) => { () };
    (@COUNT; $($elem: tt), *) => {
        <[()]>::len(&[$(count!(@SUBST; $elem)), *])
    }
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

#[test]
fn hashmap_trailing() {
    use std::collections::HashMap;
    let map: HashMap<u32, char> = hashmap! {
        1 => 'a',
        2 => 'b',
    };
    assert!(!map.is_empty());
    assert_eq!(map.len(), 2);
    assert_eq!(map[&1], 'a');
    assert_eq!(map[&2], 'b');
}
