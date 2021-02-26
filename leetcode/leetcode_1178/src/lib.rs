/**
外国友人仿照中国字谜设计了一个英文版猜字谜小游戏，请你来猜猜看吧。

字谜的迷面 puzzle 按字符串形式给出，如果一个单词 word 符合下面两个条件，那么它就可以算作谜底：

单词 word 中包含谜面 puzzle 的第一个字母。
单词 word 中的每一个字母都可以在谜面 puzzle 中找到。
例如，如果字谜的谜面是 "abcdefg"，那么可以作为谜底的单词有 "faced", "cabbage", 和 "baggage"；而 "beefed"（不含字母 "a"）以及 "based"（其中的 "s" 没有出现在谜面中）。
返回一个答案数组 answer，数组中的每个元素 answer[i] 是在给出的单词列表 words 中可以作为字谜迷面 puzzles[i] 所对应的谜底的单词数目。

示例：
输入：
words = ["aaaa","asas","able","ability","actt","actor","access"], 
puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
输出：[1,1,3,2,4,0]
解释：
1 个单词可以作为 "aboveyz" 的谜底 : "aaaa" 
1 个单词可以作为 "abrodyz" 的谜底 : "aaaa"
3 个单词可以作为 "abslute" 的谜底 : "aaaa", "asas", "able"
2 个单词可以作为 "absoryz" 的谜底 : "aaaa", "asas"
4 个单词可以作为 "actresz" 的谜底 : "aaaa", "asas", "actt", "access"
没有单词可以作为 "gaswxyz" 的谜底，因为列表中的单词都不含字母 'g'。
 

提示：
1 <= words.length <= 10^5
4 <= words[i].length <= 50
1 <= puzzles.length <= 10^4
puzzles[i].length == 7
words[i][j], puzzles[i][j] 都是小写英文字母。
每个 puzzles[i] 所包含的字符都不重复。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-valid-words-for-each-puzzle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res = Vec::new();

    let mut freqs = HashMap::new();
    for word in words {
        let mut pos = 0;
        for c in word.chars() {
            pos |= 1 << (c as usize - 'a' as usize);
        }
        *freqs.entry(pos).or_insert(0) += 1;
    }

    for puzzle in puzzles {
        let mut count = 0;
        let mut pos = 0;
        let head = puzzle.chars().nth(0).unwrap();
        for c in puzzle.chars().skip(1) {
            pos |= 1 << (c as usize - 'a' as usize)
        }
        let mut sub = pos;
        loop {
            sub = (sub - 1) & pos;
            let mask = sub + (1 << (head as usize - 'a' as usize));
            count += freqs.get(&mask).or(Some(&0)).unwrap();
            if sub == pos {
                break;
            }
        }
        res.push(count)
    }

    res
}


#[cfg(test)]
mod tests {
    use crate::find_num_of_valid_words as f1;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*])
    }

    #[test]
    fn it_works() {
        assert_eq!(
            f1(vec_of_strings!["aaaa","asas","able","ability","actt","actor","access"],
                vec_of_strings!["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]), 
            vec![1,1,3,2,4,0]);
    }
}
