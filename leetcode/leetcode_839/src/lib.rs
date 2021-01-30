/*
如果交换字符串 X 中的两个不同位置的字母，使得它和字符串 Y 相等，那么称 X 和 Y 两个字符串相似。如果这两个字符串本身是相等的，那它们也是相似的。

例如，"tars" 和 "rats" 是相似的 (交换 0 与 2 的位置)； "rats" 和 "arts" 也是相似的，但是 "star" 不与 "tars"，"rats"，或 "arts" 相似。

总之，它们通过相似性形成了两个关联组：{"tars", "rats", "arts"} 和 {"star"}。注意，"tars" 和 "arts" 是在同一组中，即使它们并不相似。形式上，对每个组而言，要确定一个单词在组中，只需要这个词和该组中至少一个单词相似。

给你一个字符串列表 strs。列表中的每个字符串都是 strs 中其它所有字符串的一个字母异位词。请问 strs 中有多少个相似字符串组？

示例 1：
输入：strs = ["tars","rats","arts","star"]
输出：2

示例 2：
输入：strs = ["omv","ovm"]
输出：1

提示：
1 <= strs.length <= 100
1 <= strs[i].length <= 1000
sum(strs[i].length) <= 2 * 104
strs[i] 只包含小写字母。
strs 中的所有单词都具有相同的长度，且是彼此的字母异位词。

备注：字母异位词（anagram），一种把某个字符串的字母的位置（顺序）加以改换所形成的新词。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/similar-string-groups
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let mut connections = Vec::new();
    for i in 0..strs.len() {
        connections.push(i);
    }

    let l = strs.len();
    for i in 0..l {
        for j in i+1..l {
            let root_i = find(&mut connections, i);
            let root_j = find(&mut connections, j);
            if root_i == root_j {
                continue;
            }
            if is_sim(&strs[i], &strs[j]) {
                connections[root_i] = root_j
            }
        }
    }
    println!("{:?}", connections);
    let mut res = 0;
    for i in 0..l {
        if connections[i] == i {
            res += 1;
        }
    }
    res
}

fn find(connections: &mut Vec<usize>, i: usize) -> usize {
    if connections[i] != i {
        connections[i] = find(connections, connections[i])
    }
    connections[i]
}

fn is_sim(str1: &str, str2: &str) -> bool {
    let mut count = 0;
    for i in 0..str1.len() {
        if str1.chars().nth(i) != str2.chars().nth(i) {
            count += 1;
            if count > 2 {
                return  false;
            }
        }
    }
    return true
}

#[test]
fn t1() {
    let strs = vec![
        "tars".into(),
        "rats".into(),
        "arts".into(),
        "star".into()
    ];
    assert_eq!(2, num_similar_groups(strs));
}

#[test]
fn t2() {
    let strs = vec![
        "omv".into(),
        "ovm".into()
    ];
    assert_eq!(1, num_similar_groups(strs));
}

#[test]
fn t3() {
    let strs = vec![
        "blw".into(),
        "bwl".into(),
        "wlb".into(),
    ];
    assert_eq!(1, num_similar_groups(strs));
}