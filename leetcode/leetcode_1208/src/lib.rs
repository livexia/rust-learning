/*
给你两个长度相同的字符串，s 和 t。

将 s 中的第 i 个字符变到 t 中的第 i 个字符需要 |s[i] - t[i]| 的开销（开销可能为 0），也就是两个字符的 ASCII 码值的差的绝对值。

用于变更字符串的最大预算是 maxCost。在转化字符串时，总开销应当小于等于该预算，这也意味着字符串的转化可能是不完全的。

如果你可以将 s 的子字符串转化为它在 t 中对应的子字符串，则返回可以转化的最大长度。

如果 s 中没有子字符串可以转化成 t 中对应的子字符串，则返回 0。

示例 1：
输入：s = "abcd", t = "bcdf", cost = 3
输出：3
解释：s 中的 "abc" 可以变为 "bcd"。开销为 3，所以最大长度为 3。

示例 2：
输入：s = "abcd", t = "cdef", cost = 3
输出：1
解释：s 中的任一字符要想变成 t 中对应的字符，其开销都是 2。因此，最大长度为 1。

示例 3：
输入：s = "abcd", t = "acde", cost = 0
输出：1
解释：你无法作出任何改动，所以最大长度为 1。

提示：
1 <= s.length, t.length <= 10^5
0 <= maxCost <= 10^6
s 和 t 都只含小写英文字母。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/get-equal-substrings-within-budget
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let mut expenses = Vec::new();
    for (a, b) in s.chars().zip(t.chars()) {
        expenses.push((a as i32 - b as i32).abs())
    }
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut cost = 0;
    let mut max = 0;

    while right < expenses.len() {
        cost += expenses[right];
        while cost > max_cost {
            cost -= expenses[left];
            left += 1;
        }
        right += 1;
        max = max.max(right - left);
    }

    max as i32
}

/*
当框内总消耗小于消耗阈值时，延展右侧窗口。
当框内纵消耗大于消耗阈值时，当前长度定为最大值；右侧移动一格，左侧也随之移动一格，维持窗口为最大符合条件的大小。
直到可以继续扩充时，才扩充右侧大小，所以在整个滑动过程中，窗口的大小是依据合法性只增不减，且直到数据末尾时的窗口大小为所有子串中的最大合法窗口，且右指针与n重合，窗口大小为n-l。
使用窗口合法非递减的性质，利用左右指针保证了算法的正确性。妙绝妙绝。
https://leetcode-cn.com/problems/get-equal-substrings-within-budget/comments/771699
*/

pub fn equal_substring2(s: String, t: String, max_cost: i32) -> i32 {
    let mut expenses = Vec::new();
    for (a, b) in s.chars().zip(t.chars()) {
        expenses.push((a as i32 - b as i32).abs())
    }
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut cost = 0;

    while right < expenses.len() {
        cost += expenses[right];
        right += 1;
        if cost > max_cost {
            cost -= expenses[left];
            left += 1;
        }
    }

    (right - left) as i32
}

#[test]
fn t1() {
    assert_eq!(equal_substring("abcd".to_string(), "bcdf".to_string(), 3), 3);
}

#[test]
fn t2() {
    assert_eq!(equal_substring("abcd".to_string(), "cdef".to_string(), 3), 1);
}