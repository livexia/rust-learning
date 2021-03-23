/**
给你一个嵌套的整型列表。请你设计一个迭代器，使其能够遍历这个整型列表中的所有整数。

列表中的每一项或者为一个整数，或者是另一个列表。其中列表的元素也可能是整数或是其他列表。

示例 1:
输入: [[1,1],2,[1,1]]
输出: [1,1,2,1,1]
解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。

示例 2:
输入: [1,[4,[6]]]
输出: [1,4,6]
解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/flatten-nested-list-iterator
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}
struct NestedIterator {
    iter: Vec<i32>,
    cur: usize,
    end: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let iter = dfs(nestedList);
        let n = iter.len();
        Self { iter, cur: 0, end: n }
    }
    
    fn next(&mut self) -> i32 {
        if self.cur < self.end {
            self.cur += 1;
            self.iter[self.cur - 1]
        } else {
            panic!("has no next")
        }
    }
    
    fn has_next(&self) -> bool {
        if self.cur < self.end {
            true
        } else {
            false
        }
    }
}

fn dfs(nested_list: Vec<NestedInteger>) -> Vec<i32>{
    let mut flatten = vec![];
    for nestedInteger in nested_list {
        match nestedInteger {
            NestedInteger::Int(n) => flatten.push(n),
            NestedInteger::List(v) => {
                flatten.append(&mut dfs(v))
            }
        }
    }
    flatten
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
