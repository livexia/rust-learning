#![feature(map_first_last)]

/**
给你一个整数 n ，请你找出并返回第 n 个 丑数 。

丑数 就是只包含质因数 2、3 和/或 5 的正整数。

示例 1：
输入：n = 10
输出：12
解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。

示例 2：
输入：n = 1
输出：1
解释：1 通常被视为丑数。

提示：
1 <= n <= 1690

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/ugly-number-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Reverse;

pub fn nth_ugly_number(n: i32) -> i32 {
    let factors = [2, 3, 5];
    let mut seen: HashSet<i64> = HashSet::new();
    let mut heap = BinaryHeap::new();
    seen.insert(1);
    heap.push(Reverse(1));
    let mut ugly = 0;
    for _ in 0..n {
        let cur = heap.pop().unwrap();
        ugly = cur.0;
        for f in &factors {
            let next = ugly * f;
            if seen.insert(next) {
                heap.push(Reverse(next))
            }
        }
    }
    ugly as i32
}

use std::collections::BTreeSet;
pub fn nth_ugly_number_btree_set(n: i32) -> i32 {
    let factors = [2, 3, 5];
    let mut btree: BTreeSet<i64> = BTreeSet::new();
    btree.insert(1);
    let mut ugly = 0;
    for _ in 0..n {
        let cur = btree.pop_first().unwrap();
        ugly = cur;
        for f in &factors {
            btree.insert(f * ugly);
        }
    }
    ugly as i32
}

pub fn nth_ugly_number_dp(n: i32) -> i32 {
    let (mut p2, mut p3, mut p5) = (0, 0, 0);
    let mut ret = vec![1];
    for _ in 1..n as usize {
        let &new = [ret[p2]*2, ret[p3]*3, ret[p5]*5].iter().min().unwrap();
        ret.push(new);
        if ret[p2] * 2 == new { p2 += 1 }
        if ret[p3] * 3 == new { p3 += 1 }
        if ret[p5] * 5 == new { p5 += 1 }
    }
    ret[n as usize - 1]
}

#[cfg(test)]
mod tests {
    use crate::nth_ugly_number;
    use crate::nth_ugly_number_btree_set;
    use crate::nth_ugly_number_dp;

    #[test]
    fn it_works() {
        assert_eq!(nth_ugly_number(10), 12);
        assert_eq!(nth_ugly_number_btree_set(10), 12);
        assert_eq!(nth_ugly_number_dp(10), 12);
    }
}
