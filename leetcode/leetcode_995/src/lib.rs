/**
在仅包含 0 和 1 的数组 A 中，一次 K 位翻转包括选择一个长度为 K 的（连续）子数组，同时将子数组中的每个 0 更改为 1，而每个 1 更改为 0。

返回所需的 K 位翻转的最小次数，以便数组没有值为 0 的元素。如果不可能，返回 -1。

示例 1：
输入：A = [0,1,0], K = 1
输出：2
解释：先翻转 A[0]，然后翻转 A[2]。

示例 2：
输入：A = [1,1,0], K = 2
输出：-1
解释：无论我们怎样翻转大小为 2 的子数组，我们都不能使数组变为 [1,1,1]。

示例 3：
输入：A = [0,0,0,1,0,1,1,0], K = 3
输出：3
解释：
翻转 A[0],A[1],A[2]: A变成 [1,1,1,1,0,1,1,0]
翻转 A[4],A[5],A[6]: A变成 [1,1,1,1,1,0,0,0]
翻转 A[5],A[6],A[7]: A变成 [1,1,1,1,1,1,1,1]
 
提示：
1 <= A.length <= 30000
1 <= K <= A.length

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-number-of-k-consecutive-bit-flips
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn min_k_bit_flips_diff(a: Vec<i32>, k: i32) -> i32 {
    let n = a.len();
    let mut diff = vec![0; n + 1];
    let mut res = 0;
    let mut rev = 0;

    for i in 0..n {
        rev += diff[i];
        if a[i] + rev % 2 == 0{
            if (i + k as usize) > n {
                return -1;
            }
            res += 1;
            rev += 1;
            diff[i+k as usize] -= 1;
        }
    }
    res
}

pub fn min_k_bit_flips(mut a: Vec<i32>, k: i32) -> i32 {
    let n = a.len();
    let mut res = 0;
    let mut rev = 0;
    let k = k as usize;

    for i in 0..n {
        if i >= k && a[i-k] > 1 {
            rev ^= 1;
        }
        if a[i] == rev {
            if i + k > n {
                return -1;
            }
            res += 1;
            rev ^= 1;
            a[i] += 2;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::min_k_bit_flips;

    #[test]
    fn it_works() {
        assert_eq!(min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }
}
