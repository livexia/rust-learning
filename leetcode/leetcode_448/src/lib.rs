/**
给定一个范围在  1 ≤ a[i] ≤ n ( n = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。

找到所有在 [1, n] 范围之间没有出现在数组中的数字。

您能在不使用额外空间且时间复杂度为O(n)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。

示例:
输入:
[4,3,2,7,8,2,3,1]

输出:
[5,6]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = (0..nums.len()).map(|x| x as i32 + 1).collect(); // 最完美情况，+1符合题意[1, n]
    nums.iter().for_each(|i| res[*i as usize - 1] = 0); // 如果出现过，那么就把res对应位置置0，例如出现8则把index为7的元素置0
    res.into_iter().filter(|x| *x != 0).collect()   //利用filter删除所有出现的0
}

#[cfg(test)]
mod tests {
    use crate::find_disappeared_numbers;

    #[test]
    fn it_works() {
        assert_eq!(find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    }
}
