/**
给你一个整数数组 nums 和两个整数 k 和 t 。请你判断是否存在两个下标 i 和 j，使得 abs(nums[i] - nums[j]) <= t ，同时又满足 abs(i - j) <= k 。

如果存在则返回 true，不存在返回 false。

示例 1：
输入：nums = [1,2,3,1], k = 3, t = 0
输出：true

示例 2：
输入：nums = [1,0,1,1], k = 1, t = 2
输出：true

示例 3：
输入：nums = [1,5,9,1,5,9], k = 2, t = 3
输出：false

提示：
0 <= nums.length <= 2 * 104
-231 <= nums[i] <= 231 - 1
0 <= k <= 104
0 <= t <= 231 - 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/contains-duplicate-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut s: HashMap<i64, i64> = HashMap::new();
    let n = nums.len();
    if t < 0 { return  false; }
    for i in 0..n {
        let mut nth = (nums[i] / (t + 1)) as i64;
        if nums[i] < 0 {
            nth -= 1
        }
        if s.contains_key(&(nth as i64)) {
            return true
        }
        if s.contains_key(&(nth - 1)) && (nums[i] as i64 - s[&(nth - 1)]).abs() <= t as i64 {
            return true
        }
        if s.contains_key(&(nth + 1)) && (nums[i] as i64 - s[&(nth + 1)]).abs() <= t as i64 {
            return true
        }
        s.insert(nth, nums[i] as i64);
        if i >= k as usize {
            s.remove(&((nums[i - k as usize] / (t + 1)) as i64));
        }
    }
    false
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
