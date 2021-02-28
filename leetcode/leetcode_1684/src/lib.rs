/**
给你一个由不同字符组成的字符串 allowed 和一个字符串数组 words 。如果一个字符串的每一个字符都在 allowed 中，就称这个字符串是 一致字符串 。

请你返回 words 数组中 一致字符串 的数目。

示例 1：
输入：allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
输出：2
解释：字符串 "aaab" 和 "baa" 都是一致字符串，因为它们只包含字符 'a' 和 'b' 。

示例 2：
输入：allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
输出：7
解释：所有字符串都是一致的。

示例 3：
输入：allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
输出：4
解释：字符串 "cc"，"acd"，"ac" 和 "d" 是一致字符串。

提示：

1 <= words.length <= 104
1 <= allowed.length <= 26
1 <= words[i].length <= 10
allowed 中的字符 互不相同 。
words[i] 和 allowed 只包含小写英文字母。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/count-the-number-of-consistent-strings
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let allowed = allowed.bytes().fold(0, |ac, c| ac | (1 << (c - b'a')));
    println!("{:b}", allowed);
    
    words.iter()
     .filter(|word| word.bytes().all(|c| allowed & (1 << (c - b'a')) != 0))
     .count() as i32
}

#[cfg(test)]
mod tests {
    use crate::count_consistent_strings;

    macro_rules! vec_of_strings {
        ($($x:expr), *) => (vec![$($x.to_string()),*]);
    }
    #[test]
    fn t1() {
        assert_eq!(count_consistent_strings("ac".to_string(), vec_of_strings!["cc"]), 1);
    }

    #[test]
    fn t2() {
        assert_eq!(count_consistent_strings("cad".to_string(), vec_of_strings!["cc","acd","b","ba","bac","bad","ac","d"]), 4);
    }
}
