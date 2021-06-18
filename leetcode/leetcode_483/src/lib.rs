use std::intrinsics::log10f64;

/**
对于给定的整数 n, 如果n的k（k>=2）进制数的所有数位全为1，则称 k（k>=2）是 n 的一个好进制。

以字符串的形式给出 n, 以字符串的形式返回 n 的最小好进制。

示例 1：
输入："13"
输出："3"
解释：13 的 3 进制是 111。

示例 2：
输入："4681"
输出："8"
解释：4681 的 8 进制是 11111。

示例 3：
输入："1000000000000000000"
输出："999999999999999999"
解释：1000000000000000000 的 999999999999999999 进制是 11。
 
提示：
n的取值范围是 [3, 10^18]。
输入总是有效且没有前导 0。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/smallest-good-base
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn smallest_good_base(n: String) -> String {
    let n = n.parse::<i64>().unwrap();
    let max = ((n as f64).log10() / 2f64.log10()).floor() as i64;
    for m in (2..=max).rev() {
        let k = (n as f64).powf(1.0 / m as f64) as i64;
        let mut mul = 1;
        let mut sum = 1;
        for i in 0..m {
            mul *= k;
            sum += mul;
        }
        if sum == n {
            return k.to_string();
        }
    }
    (n - 1).to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
