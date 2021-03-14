/**
给你一个整数 num，请你找出同时满足下面全部要求的两个整数：

两数乘积等于  num + 1 或 num + 2
以绝对差进行度量，两数大小最接近
你可以按任意顺序返回这两个整数。

 

示例 1：

输入：num = 8
输出：[3,3]
解释：对于 num + 1 = 9，最接近的两个因数是 3 & 3；对于 num + 2 = 10, 最接近的两个因数是 2 & 5，因此返回 3 & 3 。
示例 2：

输入：num = 123
输出：[5,25]
示例 3：

输入：num = 999
输出：[40,25]
 

提示：

1 <= num <= 10^9

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/closest-divisors
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn closest_divisors(num: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0, 1000000000];
    for i in num+1..num+3 {
        let cur = divide(i);
        if (cur[0] - cur[1]).abs() < (ans[1] - ans[0]).abs() {
            ans = cur;
        }
    }
    ans
}

fn divide(n: i32) -> Vec<i32> {
    for i in (0..=(n as f64).sqrt() as i32).rev() {
        if n % i == 0 {
            return vec![i, n / i];
        }
    }
    vec![0, 1000000000]
}

#[cfg(test)]
mod tests {
    use crate::closest_divisors;

    #[test]
    fn it_works() {
        assert_eq!(closest_divisors(170967091), vec![10754,15898]);
    }
}
