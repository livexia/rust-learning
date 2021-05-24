/**
有台奇怪的打印机有以下两个特殊要求：

打印机每次只能打印由 同一个字符 组成的序列。
每次可以在任意起始和结束位置打印新字符，并且会覆盖掉原来已有的字符。
给你一个字符串 s ，你的任务是计算这个打印机打印它需要的最少打印次数。

示例 1：
输入：s = "aaabbb"
输出：2
解释：首先打印 "aaa" 然后打印 "bbb"。

示例 2：
输入：s = "aba"
输出：2
解释：首先打印 "aaa" 然后在第二个位置打印 "b" 覆盖掉原来的字符 'a'。
 
提示：
1 <= s.length <= 100
s 由小写英文字母组成

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/strange-printer
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn strange_printer(s: String) -> i32 {
    let bs = s.as_bytes();
    let n = s.len();
    let mut f = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        f[i][i] = 1;
        for j in i + 1..n {
            if bs[i] == bs[j] {
                f[i][j] = f[i][j - 1];
            } else {
                let mut minn = i32::MAX;
                for k in i..j {
                    minn = minn.min(f[i][k] + f[k + 1][j])
                }
                f[i][j] = minn;
            }
        }
    }
    f[0][n-1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
