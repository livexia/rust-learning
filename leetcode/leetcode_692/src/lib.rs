/**
给一非空的单词列表，返回前 k 个出现次数最多的单词。

返回的答案应该按单词出现频率由高到低排序。如果不同的单词有相同出现频率，按字母顺序排序。

示例 1：
输入: ["i", "love", "leetcode", "i", "love", "coding"], k = 2
输出: ["i", "love"]
解析: "i" 和 "love" 为出现次数最多的两个单词，均为2次。
    注意，按字母顺序 "i" 在 "love" 之前。
 

示例 2：

输入: ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
输出: ["the", "is", "sunny", "day"]
解析: "the", "is", "sunny" 和 "day" 是出现次数最多的四个单词，
    出现次数依次为 4, 3, 2 和 1 次。
 
注意：
假定 k 总为有效值， 1 ≤ k ≤ 集合元素数。
输入的单词均由小写字母组成。
 
扩展练习：
尝试以 O(n log k) 时间复杂度和 O(n) 空间复杂度解决。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/top-k-frequent-words
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut cnt = HashMap::new();
    for word in words {
        *cnt.entry(word).or_insert(0) += 1;
    }
    let mut ret: Vec<(String, i32)> = cnt.into_iter().collect();
    ret.sort_by(|a, b| if a.1 == b.1 {
        a.0.cmp(&b.0)
    } else {
        b.1.cmp(&a.1)
    });
    ret.into_iter().map(|(s, _)| s).take(k as usize).collect()
}   

#[cfg(test)]
mod tests {
    use crate::top_k_frequent;

    #[test]
    fn t1() {
        let words = ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"].iter().map(|s| s.to_string()).collect();
        let ret: Vec<String> = ["the", "is", "sunny", "day"].iter().map(|s| s.to_string()).collect();
        assert_eq!(top_k_frequent(words, 4), ret);
    }

    #[test]
    fn t2() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"].iter().map(|s| s.to_string()).collect();
        let ret: Vec<String> = ["i", "love"].iter().map(|s| s.to_string()).collect();
        assert_eq!(top_k_frequent(words, 2), ret);
    }

    #[test]
    fn t3() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"].iter().map(|s| s.to_string()).collect();
        let ret: Vec<String> = ["i"].iter().map(|s| s.to_string()).collect();
        assert_eq!(top_k_frequent(words, 1), ret);
    }
}
