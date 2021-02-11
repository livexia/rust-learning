use std::usize;

/**
给出一个二维整数网格 grid，网格中的每个值表示该位置处的网格块的颜色。

只有当两个网格块的颜色相同，而且在四个方向中任意一个方向上相邻时，它们属于同一连通分量。

连通分量的边界是指连通分量中的所有与不在分量中的正方形相邻（四个方向上）的所有正方形，或者在网格的边界上（第一行/列或最后一行/列）的所有正方形。

给出位于 (r0, c0) 的网格块和颜色 color，使用指定颜色 color 为所给网格块的连通分量的边界进行着色，并返回最终的网格 grid 。

示例 1：
输入：grid = [[1,1],[1,2]], r0 = 0, c0 = 0, color = 3
输出：[[3, 3], [3, 2]]

示例 2：
输入：grid = [[1,2,2],[2,3,2]], r0 = 0, c0 = 1, color = 3
输出：[[1, 3, 3], [2, 3, 3]]

示例 3：
输入：grid = [[1,1,1],[1,1,1],[1,1,1]], r0 = 1, c0 = 1, color = 2
输出：[[2, 2, 2], [2, 1, 2], [2, 2, 2]]

提示：
1 <= grid.length <= 50
1 <= grid[0].length <= 50
1 <= grid[i][j] <= 1000
0 <= r0 < grid.length
0 <= c0 < grid[0].length
1 <= color <= 1000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/coloring-a-border
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn color_border(grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
    if grid.len() == 0 {
        return vec![vec![]];
    }
    let mut visited = Vec::new();
    visited.push((r0 as usize, c0 as usize));
    let target = grid[r0 as usize][c0 as usize];

    let mut grid = grid;
    dfs(&mut grid, r0 as usize, c0 as usize, color, &mut visited, target);
    grid
}

fn dfs(grid:  &mut Vec<Vec<i32>>,x: usize, y: usize, color: i32, visited: &mut Vec<(usize, usize)>, target: i32) {
    let pos: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let m = grid.len();
    let n = grid[0].len();
    for (i, j) in pos {
        let a = x as i32 + i;
        let b = y as i32 + j;

        if a >= 0 && a < m as i32 && b >= 0 && b < n as i32 {
            let a = a as usize;
            let b = b as usize;
            if !visited.contains(&(a, b)) {
                if grid[a][b] == target {
                    visited.push((a, b));
                    dfs(grid, a, b, color, visited, target);
                } else {
                    grid[x][y] = color;
                }
            }
        } else {
            grid[x][y] = color
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
