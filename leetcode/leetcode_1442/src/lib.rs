/**
给你一个整数数组 arr 。

现需要从数组中取三个下标 i、j 和 k ，其中 (0 <= i < j <= k < arr.length) 。

a 和 b 定义如下：

a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]
注意：^ 表示 按位异或 操作。

请返回能够令 a == b 成立的三元组 (i, j , k) 的数目。

示例 1：
输入：arr = [2,3,1,6,7]
输出：4
解释：满足题意的三元组分别是 (0,1,2), (0,2,2), (2,3,4) 以及 (2,4,4)

示例 2：
输入：arr = [1,1,1,1,1]
输出：10

示例 3：
输入：arr = [2,3]
输出：0

示例 4：
输入：arr = [1,3,5,7,9]
输出：3

示例 5：
输入：arr = [7,11,12,9,5,2,7,17,22]
输出：8

提示：
1 <= arr.length <= 300
1 <= arr[i] <= 10^8

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn count_triplets(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    for i in 0..n {
        let mut xorsum = 0;
        for k in i..n {
            xorsum ^= arr[k];
            if xorsum == 0 {
                ans += (k - i) as i32;
            }
        }
    }
    ans
}

use std::collections::HashMap;

pub fn count_triplets_hash(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut freq = HashMap::new();
    freq.insert(0, 1);
    let mut idsum = HashMap::new();
    idsum.insert(0, 0);
    let mut xorsum = 0;
    let mut ans = 0;
    for k in 0..n {
        xorsum ^= arr[k];
        if freq.contains_key(&xorsum) {
            ans += freq[&xorsum] * k - idsum[&xorsum];
        }
        *freq.entry(xorsum).or_default() += 1;
        *idsum.entry(xorsum).or_default() += k +1;
    }
    ans as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
