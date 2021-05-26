/**
给出一个字符串 s（仅含有小写英文字母和括号）。

请你按照从括号内到外的顺序，逐层反转每对匹配括号中的字符串，并返回最终的结果。

注意，您的结果中 不应 包含任何括号。

示例 1：
输入：s = "(abcd)"
输出："dcba"

示例 2：
输入：s = "(u(love)i)"
输出："iloveu"

示例 3：
输入：s = "(ed(et(oc))el)"
输出："leetcode"

示例 4：
输入：s = "a(bcdefghijkl(mno)p)q"
输出："apmnolkjihgfedcbq"

提示：

0 <= s.length <= 2000
s 中只有小写英文字母和括号
我们确保所有括号都是成对出现的

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reverse-substrings-between-each-pair-of-parentheses
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn reverse_parentheses(s: String) -> String {
    let bs = s.as_bytes();
    let n = s.len();
    let mut pair = vec![0; n];
    let mut stack = std::collections::VecDeque::new();
    for i in 0..n {
        if bs[i] == b'(' {
            stack.push_back(i)
        } else if bs[i] == b')' {
            if let Some(j) = stack.pop_back() {
                pair[i] = j;
                pair[j] = i;
            }

        }
    }

    let mut res = String::new();
    let mut index = 0;
    let mut step = 1;
    while index < n {
        if bs[index] == b'(' || bs[index] == b')' {
            index = pair[index];
            step = -step;
        } else {
            res.push(bs[index] as char)
        }
        index = (index as i32 + step) as usize;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
