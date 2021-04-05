/**
给你两个二进制字符串，返回它们的和（用二进制表示）。

输入为 非空 字符串且只包含数字 1 和 0。

示例 1:
输入: a = "11", b = "1"
输出: "100"

示例 2:
输入: a = "1010", b = "1011"
输出: "10101"

提示：

每个字符串仅由字符 '0' 或 '1' 组成。
1 <= a.length, b.length <= 10^4
字符串如果不是 "0" ，就都不含前导零。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/add-binary
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn add_binary(a: String, b: String) -> String {
    let mut carry: u8 = 0;
    let mut answer = vec![];
    let mut a_bytes: Vec<u8> = a.bytes().collect();
    let mut b_bytes: Vec<u8> = b.bytes().collect();
    let n = a_bytes.len().max(b_bytes.len());
    for _ in 0..n {
        let c1 = a_bytes.pop().or(Some(b'0')).unwrap() - b'0';
        let c2 = b_bytes.pop().or(Some(b'0')).unwrap() - b'0';
        let sum = c1 + c2 + carry;
        answer.push(b'0' + sum % 2);
        carry = sum / 2;
    }
    if carry > 0 {
        answer.push(b'1');
    }
    answer.reverse();
    String::from_utf8(answer).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::add_binary;

    #[test]
    fn it_works() {
        assert_eq!(add_binary("1111".to_string(), "1111".to_string()), "11110".to_string());
    }
}
