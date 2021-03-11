/**
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

整数除法仅保留整数部分。

示例 1：
输入：s = "3+2*2"
输出：7

示例 2：
输入：s = " 3/2 "
输出：1

示例 3：
输入：s = " 3+5 / 2 "
输出：5

提示：

1 <= s.length <= 3 * 105
s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
s 表示一个 有效表达式
表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内
题目数据保证答案是一个 32-bit 整数

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/basic-calculator-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn calculate(s: String) -> i32 {
    let mut chars: Vec<char> = s.chars().collect();
    chars.extend(&['+', '0']);
    let mut stack = vec![];
    let mut num = 0;
    let mut op = '+';
    for c in chars {
        if c.is_numeric() {
            num = 10 * num + c.to_digit(10).unwrap() as i32;
        }
        if "-+*/".contains(c) {
            if op == '-' {
                stack.push(-num);
            } else if op == '+' {
                stack.push(num);
            } else if op == '*' {
                let num1  = stack.pop().unwrap();
                stack.push(num1 * num)
            } else if op == '/' {
                let num1  = stack.pop().unwrap();
                stack.push(num1 / num)
            }
            op = c;
            num = 0;
        }
    }
    stack.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        assert_eq!(calculate("3+5*10/2".to_string()), 28);
    }
}
