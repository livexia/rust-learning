// union find
/*
在由 1 x 1 方格组成的 N x N 网格 grid 中，每个 1 x 1 方块由 /、\ 或空格构成。这些字符会将方块划分为一些共边的区域。

（请注意，反斜杠字符是转义的，因此 \ 用 "\\" 表示。）。

返回区域的数目。

示例 1：
输入：
[
  " /",
  "/ "
]
输出：2
解释：2x2 网格如下：

示例 2：
输入：
[
  " /",
  "  "
]
输出：1
解释：2x2 网格如下：

示例 3：
输入：
[
  "\\/",
  "/\\"
]
输出：4
解释：（回想一下，因为 \ 字符是转义的，所以 "\\/" 表示 \/，而 "/\\" 表示 /\。）
2x2 网格如下：

示例 4：
输入：
[
  "/\\",
  "\\/"
]
输出：5
解释：（回想一下，因为 \ 字符是转义的，所以 "/\\" 表示 /\，而 "\\/" 表示 \/。）
2x2 网格如下：

示例 5：
输入：
[
  "//",
  "/ "
]
输出：3
解释：2x2 网格如下：

提示：
1 <= grid.length == grid[0].length <= 30
grid[i][j] 是 '/'、'\'、或 ' '。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/regions-cut-by-slashes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/


fn main() {
    println!("Hello, world!");
    regions_by_slashes(vec!["\\/".to_string(), "/\\".to_string()]);
    regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]);
    regions_by_slashes(vec![" /".to_string(), "  ".to_string()]);
    regions_by_slashes(vec!["\\/\\ ".to_string()," /\\/".to_string()," \\/ ".to_string(),"/ / ".to_string()]);
}

pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let mut result = 1;

    let n = grid.len() + 1;
    let mut connections: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    for i in 0..n {
        for j in 0..n {
            connections[i].push((i, j));
        }
    }
    connections[0] = vec![(0, 0); n];
    connections[n-1] = vec![(0, 0); n];
    for i in 0..n {
        connections[i][0] = (0, 0);
        connections[i][n-1] = (0, 0);
    }
    println!("{:?}", connections);
    for i in 0..n-1 {
        for j in 0..n-1 {
            if grid[i].chars().nth(j).unwrap() == '/' {
                let t1 = find(&connections, i + 1, j);
                let t2 = find(&connections, i, j + 1);
                if t1 == t2 {
                    result += 1;
                } else {
                    connections[t1.0][t1.1] = connections[t2.0][t2.1]
                }
            } else if grid[i].chars().nth(j).unwrap() == '\\' {
                let t1 = find(&connections, i + 1, j + 1);
                let t2 = find(&connections, i, j);
                if t1 == t2 {
                    result += 1;
                } else {
                    connections[t1.0][t1.1] = connections[t2.0][t2.1]
                }
            }
        }
    }
    println!("{:?}", connections);
    println!("{:?}", result);
    result
}

fn find(connections: &Vec<Vec<(usize, usize)>>, i: usize, j: usize) -> (usize, usize) {
    let mut x = i;
    let mut y = j;
    while (x, y) != connections[x][y] {
        let t = connections[x][y];
        x = t.0;
        y = t.1;
    }
    return (x, y)
}