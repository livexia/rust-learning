use std::usize;

/*
给定一个正整数数组 A，如果 A 的某个子数组中不同整数的个数恰好为 K，则称 A 的这个连续、不一定独立的子数组为好子数组。

（例如，[1,2,3,1,2] 中有 3 个不同的整数：1，2，以及 3。）

返回 A 中好子数组的数目。

示例 1：
输入：A = [1,2,1,2,3], K = 2
输出：7
解释：恰好由 2 个不同整数组成的子数组：[1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2].

示例 2：
输入：A = [1,2,1,3,4], K = 3
输出：3
解释：恰好由 3 个不同整数组成的子数组：[1,2,1,3], [2,1,3], [1,3,4].

提示：
1 <= A.length <= 20000
1 <= A[i] <= A.length
1 <= K <= A.length

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/subarrays-with-k-different-integers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
    at_most_k_distinct(&a, k) - at_most_k_distinct(&a, k-1)
}

pub fn at_most_k_distinct(a: &Vec<i32>, k: i32) -> i32 {
    let len = a.len();
    let mut freq = vec![0; len + 1];

    let mut left = 0;
    let mut right = 0;

    let mut count = 0;
    let mut res = 0;

    while right < len {
        if freq[a[right] as usize] == 0 {
            count += 1;
        }
        freq[a[right] as usize] += 1;
        right += 1;

        while count > k {
            freq[a[left] as usize] -= 1;
            if freq[a[left] as usize] == 0 {
                count -= 1;
            }
            left += 1;
        }
        res += right - left;
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use crate::subarrays_with_k_distinct;

    #[test]
    fn it_works() {
        assert_eq!(subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    }
}
