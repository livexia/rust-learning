/*
给你一个按升序排序的整数数组 num（可能包含重复数字），请你将它们分割成一个或多个长度至少为 3 的子序列，其中每个子序列都由连续整数组成。

如果可以完成上述分割，则返回 true ；否则，返回 false 。

示例 1：
输入: [1,2,3,3,4,5]
输出: True
解释:
你可以分割出这样两个连续子序列 : 
1, 2, 3
3, 4, 5

示例 2：
输入: [1,2,3,3,4,4,5,5]
输出: True
解释:
你可以分割出这样两个连续子序列 : 
1, 2, 3, 4, 5
3, 4, 5

示例 3：
输入: [1,2,3,4,4,5]
输出: False

提示：

1 <= nums.length <= 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/split-array-into-consecutive-subsequences
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;
pub fn is_possible(nums: Vec<i32>) -> bool {
    if nums.len() < 3 {
        return false;
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in nums.iter() {
        *map.entry(*i).or_insert(0) += 1;
    }
    let mut tails: HashMap<i32, i32> = HashMap::new();
    for i in nums.iter() {
        if map[i] == 0 {
            continue;
        }
        if *tails.get(&(*i-1)).unwrap_or(&0) > 0 {
            *map.entry(*i).or_insert(0) -= 1;
            *tails.entry(*i-1).or_insert(0) -= 1;
            *tails.entry(*i).or_insert(0) += 1;
        } else if *map.get(&(*i+1)).unwrap_or(&0) > 0 && *map.get(&(*i+2)).unwrap_or(&0) > 0 {
            for j in *i..*i+3 {
                *map.entry(j).or_insert(0) -= 1;
            }
            *tails.entry(*i+2).or_insert(0) += 1;
        } else {
            return false;
        }
    }
    return true;
}

#[test]
fn t1() {
    assert_eq!(true, is_possible(vec![1, 2, 3, 3, 4, 5]));
}
