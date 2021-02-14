/**
## 面试题 08.08. 有重复字符串的排列组合

有重复字符串的排列组合。编写一种方法，计算某字符串的所有排列组合。

示例1:
 输入：S = "qqe"
 输出：["eqq","qeq","qqe"]

示例2:
 输入：S = "ab"
 输出：["ab", "ba"]
提示:

字符都是英文字母。
字符串长度在[1, 9]之间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/permutation-ii-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn permutation(s: String) -> Vec<String> {
    if s.len() < 2 {
        return vec![];
    }
    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    let mut res: Vec<String> = Vec::new();
    let mut visited: Vec<bool> = vec![false; s.len()];
    let mut path = s.clone();

    dfs(0, &mut visited, &s, &mut res, &mut path);

    res
}

fn dfs(n: usize, visited: &mut Vec<bool>, s: &Vec<char>, res: &mut Vec<String>, path: &mut Vec<char>) {
    if n == visited.len() {
        let temp = path.clone().into_iter().collect();
        res.push(temp);
        return;
    }
    for i in 0..s.len() {
        if visited[i] {
            continue;
        }
        if i > 0 && s[i] == s[i-1] && !visited[i-1] {
            continue;
        }
        visited[i] = true;
        path[n] = s[i];
        dfs(n + 1, visited, s, res, path);
        visited[i] = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::permutation;

    #[test]
    fn it_works() {
        assert_eq!(permutation("qqe".to_string()), vec!["eqq".to_string(),"qeq".to_string(),"qqe".to_string()]);
    }
}
