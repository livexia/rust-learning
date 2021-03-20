/**
给定一个正整数 x，我们将会写出一个形如 x (op1) x (op2) x (op3) x ... 的表达式，其中每个运算符 op1，op2，… 可以是加、减、乘、除（+，-，*，或是 /）之一。例如，对于 x = 3，我们可以写出表达式 3 * 3 / 3 + 3 - 3，该式的值为 3 。

在写这样的表达式时，我们需要遵守下面的惯例：

除运算符（/）返回有理数。
任何地方都没有括号。
我们使用通常的操作顺序：乘法和除法发生在加法和减法之前。
不允许使用一元否定运算符（-）。例如，“x - x” 是一个有效的表达式，因为它只使用减法，但是 “-x + x” 不是，因为它使用了否定运算符。 
我们希望编写一个能使表达式等于给定的目标值 target 且运算符最少的表达式。返回所用运算符的最少数量。

 

示例 1：

输入：x = 3, target = 19
输出：5
解释：3 * 3 + 3 * 3 + 3 / 3 。表达式包含 5 个运算符。
示例 2：

输入：x = 5, target = 501
输出：8
解释：5 * 5 * 5 * 5 - 5 * 5 * 5 + 5 / 5 。表达式包含 8 个运算符。
示例 3：

输入：x = 100, target = 100000000
输出：3
解释：100 * 100 * 100 * 100 。表达式包含 3 个运算符。
 

提示：

2 <= x <= 100
1 <= target <= 2 * 10^8


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/least-operators-to-express-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
    let mut memo = HashMap::new();
    dp(0, target, x, &mut memo) - 1
}

fn dp(i: i32, target: i32, x: i32, memo: &mut HashMap<String, i32>) -> i32 {
    let code = format!("{}#{}", i, target);
    if memo.contains_key(&code) {
        return memo[&code];
    }
    let ans;
    if target == 0 {
        ans = 0;
    } else if target                                                                                                                                                                                                                                                                             == 1 {
        ans = cost(i);
    } else if i >= 39 {
        ans = target + 1;
    } else {
        let t = target / x;
        let r = target % x;
        ans = (r * cost(i) + dp(i+1, t, x, memo)).min(
            (x-r) * cost(i) + dp(i+1, t + 1, x, memo)
        )
    }
    memo.insert(code, ans);
    ans
}

fn cost(x: i32) -> i32 {
    match x > 0 {
        true => x,
        false => 2
    }
}

#[cfg(test)]
mod tests {
    use crate::least_ops_express_target;

    #[test]
    fn it_works() {
        assert_eq!(least_ops_express_target(5, 501), 8);
        assert_eq!(least_ops_express_target(2, 1125082), 84);
    }
}
