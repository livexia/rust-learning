/**
你的面前有一堵矩形的、由 n 行砖块组成的砖墙。这些砖块高度相同（也就是一个单位高）但是宽度不同。每一行砖块的宽度之和应该相等。

你现在要画一条 自顶向下 的、穿过 最少 砖块的垂线。如果你画的线只是从砖块的边缘经过，就不算穿过这块砖。你不能沿着墙的两个垂直边缘之一画线，这样显然是没有穿过一块砖的。

给你一个二维数组 wall ，该数组包含这堵墙的相关信息。其中，wall[i] 是一个代表从左至右每块砖的宽度的数组。你需要找出怎样画才能使这条线 穿过的砖块数量最少 ，并且返回 穿过的砖块数量 。

示例 1：
输入：wall = [[1,2,2,1],[3,1,2],[1,3,2],[2,4],[3,1,2],[1,3,1,1]]
输出：2

示例 2：
输入：wall = [[1],[1],[1]]
输出：3

提示：
n == wall.length
1 <= n <= 104
1 <= wall[i].length <= 104
1 <= sum(wall[i].length) <= 2 * 104
对于每一行 i ，sum(wall[i]) 应当是相同的
1 <= wall[i][j] <= 231 - 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/brick-wall
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    let mut right_edge = HashMap::new();
    let h = wall.len() as i32;
    for row in wall {
        let mut sum = 0;
        let n = row.len();
        for i in 0..n - 1 {
            sum += row[i];
            *right_edge.entry(sum).or_insert(0) += 1;
        }
    }
    let mut max = 0;
    for (_, v) in right_edge {
        max = max.max(v)
    }
    h - max
}

#[cfg(test)]
mod tests {
    use crate::least_bricks;

    #[test]
    fn it_works() {
        let wall = vec![
            vec![1, 2, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 4],
            vec![3, 1, 2],
            vec![1, 3, 1, 1],
        ];

        assert_eq!(least_bricks(wall), 2);
    }
}
