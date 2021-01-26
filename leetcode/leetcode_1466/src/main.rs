/*
n 座城市，从 0 到 n-1 编号，其间共有 n-1 条路线。因此，要想在两座不同城市之间旅行只有唯一一条路线可供选择（路线网形成一颗树）。去年，交通运输部决定重新规划路线，以改变交通拥堵的状况。

路线用 connections 表示，其中 connections[i] = [a, b] 表示从城市 a 到 b 的一条有向路线。

今年，城市 0 将会举办一场大型比赛，很多游客都想前往城市 0 。

请你帮助重新规划路线方向，使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。

题目数据 保证 每个城市在重新规划路线方向后都能到达城市 0 。

示例 1：
输入：n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
输出：3
解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。

示例 2：
输入：n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
输出：2
解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。

示例 3：
输入：n = 3, connections = [[1,0],[2,0]]
输出：0
 
提示：
2 <= n <= 5 * 10^4
connections.length == n-1
connections[i].length == 2
0 <= connections[i][0], connections[i][1] <= n-1
connections[i][0] != connections[i][1]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

fn main() {
    println!("Hello, world!");
    println!("{:?}", min_reorder(6, vec![vec![0,1],vec![1,3],vec![2,3],vec![4,0],vec![4,5]]));
    println!("{:?}", min_reorder(5, vec![vec![1,0],vec![1,2],vec![3,2],vec![3,4]]));
    println!("{:?}", min_reorder(5, vec![vec![4, 3],vec![2, 3],vec![1, 2],vec![1, 0]]));
}

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut v = Vec::new();
    let mut res = 0;
    for i in 0..n as usize {
        v.push(i);
    }
    let mut ret = 1;
    while ret != 0 {
        ret = 0;
        for i in &connections {
            let root_i = find(&mut v, i[0] as usize);
            let root_j = find(&mut v, i[1] as usize);
            if root_i == 0 && root_j != 0{
                v[root_j] = v[root_i];
                res += 1;
                ret = 1;
            } else if root_i != 0 {
                if root_j == 0 {
                    v[root_i] = v[root_j];
                } else {
                    ret = 1;
                }
            }
            println!("{:?}", v);
        }
    }
    res
}

fn find(v: &mut Vec<usize>, i: usize) -> usize {
    if v[i] != i {
        v[i] = find(v, v[i]);
    }
    v[i]
}