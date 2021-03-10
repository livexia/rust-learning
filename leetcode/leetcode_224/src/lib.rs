/**
实现一个基本的计算器来计算一个简单的字符串表达式 s 的值。

示例 1：
输入：s = "1 + 1"
输出：2

示例 2：
输入：s = " 2-1 + 2 "
输出：3

示例 3：
输入：s = "(1+(4+5+2)-3)+(6+8)"
输出：23

提示：
1 <= s.length <= 3 * 105
s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
s 表示一个有效的表达式

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/basic-calculator
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn calculate(s: String) -> i32 {
    let chars: Vec<char> = s.trim().chars().collect();
    let mut stack = vec![];
    let mut num = 0;
    let mut sign = 1;
    let mut answer = 0;
    for c in chars {
        if c.is_numeric() {
            num = 10 * num + c.to_digit(10).unwrap() as i32;
        } else if c == '-' || c == '+' {
            answer += sign * num;
            num = 0;
            if c == '+' { sign = 1 } else { sign = -1 };
        } else if c == '(' {
            stack.push(answer);
            stack.push(sign);
            answer = 0;
            sign = 1;
        } else if c == ')' {
            answer += sign * num;
            num = 0;
            answer *= stack.pop().unwrap();
            answer += stack.pop().unwrap();
        }
    }
    answer + sign * num
}

#[cfg(test)]
mod tests {
    use crate::calculate;

    #[test]
    fn it_works() {
        assert_eq!(calculate(" 2-(1 + 2) ".to_string()), -1);
    }
}
