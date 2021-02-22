/**
还记得童话《卖火柴的小女孩》吗？现在，你知道小女孩有多少根火柴，请找出一种能使用所有火柴拼成一个正方形的方法。不能折断火柴，可以把火柴连接起来，并且每根火柴都要用到。

输入为小女孩拥有火柴的数目，每根火柴用其长度表示。输出即为是否能用所有的火柴拼成正方形。

示例 1:
输入: [1,1,2,2,2]
输出: true

解释: 能拼成一个边长为2的正方形，每边两根火柴。

示例 2:
输入: [3,3,3,3,4]
输出: false

解释: 不能用所有火柴拼成一个正方形。
注意:
给定的火柴长度和在 0 到 10^9之间。
火柴数组的长度不超过15。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/matchsticks-to-square
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn makesquare(nums: Vec<i32>) -> bool {
    let count = nums.len();
    if count < 4 { return false };
    let total_len = nums.clone().into_iter().sum::<i32>();
    let side = total_len / 4;
    if side * 4 != total_len { return false; };
    let mut nums = nums;
    nums.sort();
    nums.reverse();

    let mut sums = vec![0; 4];
    return dfs(&nums, 0, &mut sums, side);
}

pub fn dfs(nums: &Vec<i32>, index: usize, sums: &mut Vec<i32>, side: i32) -> bool{
    if index == nums.len() {
        return sums[0] == sums[1] && sums[1] == sums[2] && sums[2] == sums[3];
    }
    let check = nums[index];
    for i in 0..4 {
        if sums[i] + check <= side {
            sums[i] += check;
            if dfs(nums, index+1, sums, side) {
                return true;
            }
            sums[i] -= check;
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
