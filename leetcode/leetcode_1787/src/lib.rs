/**
给你一个整数数组 nums​​​ 和一个整数 k​​​​​ 。区间 [left, right]（left <= right）的 异或结果 是对下标位于 left 和 right（包括 left 和 right ）之间所有元素进行 XOR 运算的结果：nums[left] XOR nums[left+1] XOR ... XOR nums[right] 。

返回数组中 要更改的最小元素数 ，以使所有长度为 k 的区间异或结果等于零。

示例 1：
输入：nums = [1,2,0,3,0], k = 1
输出：3
解释：将数组 [1,2,0,3,0] 修改为 [0,0,0,0,0]

示例 2：
输入：nums = [3,4,5,2,1,7,3,4,7], k = 3
输出：3
解释：将数组 [3,4,5,2,1,7,3,4,7] 修改为 [3,4,7,3,4,7,3,4,7]

示例 3：
输入：nums = [1,2,4,1,2,5,1,2,6], k = 3
输出：3
解释：将数组[1,2,4,1,2,5,1,2,6] 修改为 [1,2,3,1,2,3,1,2,3]
 
提示：
1 <= k <= nums.length <= 2000
​​​​​​0 <= nums[i] < 210

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/make-the-xor-of-all-segments-equal-to-zero
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let maxx = 1 << 10;
    let infty = i32::MAX / 2;
    let n = nums.len();
    let mut f = vec![infty; maxx as usize];
    f[0] = 0;

    for i in 0..k as usize {
        let mut cnt = std::collections::HashMap::new();
        let mut size = 0;
        for j in (i..n).step_by(k as usize) {
            *cnt.entry(nums[j]).or_insert(0) += 1;
            size += 1;
        }

        let &t2min = f.iter().min().unwrap();
        let mut g = vec![t2min; maxx as usize];
        for mask in 0..maxx {
            for (&x, &countx) in &cnt {
                g[mask] = g[mask].min(f[mask ^ x as usize] - countx)
            }
        }
        for j in 0..maxx {
            g[j] += size
        }
        f = g
    }
    f[0] as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
