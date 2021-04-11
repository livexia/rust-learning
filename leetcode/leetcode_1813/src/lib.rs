use std::mem::swap;

/**
一个句子是由一些单词与它们之间的单个空格组成，且句子的开头和结尾没有多余空格。比方说，"Hello World" ，"HELLO" ，"hello world hello world" 都是句子。每个单词都 只 包含大写和小写英文字母。

如果两个句子 sentence1 和 sentence2 ，可以通过往其中一个句子插入一个任意的句子（可以是空句子）而得到另一个句子，那么我们称这两个句子是 相似的 。比方说，sentence1 = "Hello my name is Jane" 且 sentence2 = "Hello Jane" ，我们可以往 sentence2 中 "Hello" 和 "Jane" 之间插入 "my name is" 得到 sentence1 。

给你两个句子 sentence1 和 sentence2 ，如果 sentence1 和 sentence2 是相似的，请你返回 true ，否则返回 false 。

示例 1：
输入：sentence1 = "My name is Haley", sentence2 = "My Haley"
输出：true
解释：可以往 sentence2 中 "My" 和 "Haley" 之间插入 "name is" ，得到 sentence1 。

示例 2：
输入：sentence1 = "of", sentence2 = "A lot of words"
输出：false
解释：没法往这两个句子中的一个句子只插入一个句子就得到另一个句子。

示例 3：
输入：sentence1 = "Eating right now", sentence2 = "Eating"
输出：true
解释：可以往 sentence2 的结尾插入 "right now" 得到 sentence1 。

示例 4：
输入：sentence1 = "Luky", sentence2 = "Lucccky"
输出：false

提示：
1 <= sentence1.length, sentence2.length <= 100
sentence1 和 sentence2 都只包含大小写英文字母和空格。
sentence1 和 sentence2 中的单词都只由单个空格隔开。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sentence-similarity-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    if sentence1 == sentence2 { return true }
    let mut s1: Vec<&str> = sentence1.split(" ").collect();
    let mut s2: Vec<&str> = sentence2.split(" ").collect();
    if s1.len() > s2.len() {
        swap(&mut s1, &mut s2);
    }
    let (mut left, mut right) = (0, s1.len());
    while left < right {
        if s1[left] != s2[left] && s1.last() != s2.last() {
            return false
        }
        if s1[left] == s2[left] {
            left += 1;
        }
        if s1.last() == s2.last() {
            s1.pop();
            s2.pop();
            right -= 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::are_sentences_similar;

    #[test]
    fn t1() {
        let sentence1 = "Eating right now".to_string();
        let sentence2 = "Eating".to_string();
        assert!(are_sentences_similar(sentence1, sentence2));
    }

    #[test]
    fn t2() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "My Haley".to_string();
        assert!(are_sentences_similar(sentence1, sentence2));
    }

    #[test]
    fn t3() {
        let sentence1 = "My name is Haley".to_string();
        let sentence2 = "name".to_string();
        assert!(!are_sentences_similar(sentence1, sentence2));
    }

    #[test]
    fn t4() {
        let sentence1 = "BE Pu g Y k UMEkkF WfU U akwRv re J Qgw zGU tA UE Z vJGqJ p nJ ipESxr nQRkFMx H qQqEjO luX ZB".to_string();
        let sentence2 = "BE Pu g Y k UMEkkF WfU U akwRv J Qgw tA UE Z vJGqJ p nJ ipESxr nQRkFMx H qQqEjO luX ZB".to_string();
        assert!(!are_sentences_similar(sentence1, sentence2));
    }
}
