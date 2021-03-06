/**
给你一个由若干 0 和 1 组成的二维网格 grid ，其中 0 表示水，而 1 表示陆地。岛屿由水平方向或竖直方向上相邻的 1 （陆地）连接形成。

如果 恰好只有一座岛屿 ，则认为陆地是 连通的 ；否则，陆地就是 分离的 。

一天内，可以将任何单个陆地单元（1）更改为水单元（0）。

返回使陆地分离的最少天数。

示例 1：
输入：grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]
输出：2
解释：至少需要 2 天才能得到分离的陆地。
将陆地 grid[1][1] 和 grid[0][2] 更改为水，得到两个分离的岛屿。

示例 2：
输入：grid = [[1,1]]
输出：2
解释：如果网格中都是水，也认为是分离的 ([[1,1]] -> [[0,0]])，0 岛屿。

示例 3：
输入：grid = [[1,0,1,0]]
输出：0

示例 4：
输入：grid = [[1,1,0,1,1],
             [1,1,1,1,1],
             [1,1,0,1,1],
             [1,1,0,1,1]]
输出：1

示例 5：
输入：grid = [[1,1,0,1,1],
             [1,1,1,1,1],
             [1,1,0,1,1],
             [1,1,1,1,1]]
输出：2

提示：
1 <= grid.length, grid[i].length <= 30
grid[i][j] 为 0 或 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-number-of-days-to-disconnect-island
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let m = grid.len();
    let n = grid[0].len();
    if count_land(&mut grid, m, n) != 1 {
        return 0;
    }
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                if count_land(&mut grid, m, n) != 1 {
                    return 1;
                }
                grid[i][j] = 1;
            }
        }
    }
    2
}

fn count_land(grid: &mut Vec<Vec<i32>>, m: usize, n: usize) -> i32 {
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                count += 1;
                dfs(i, j, grid, m, n);
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 2 {
                grid[i][j] = 1;
            }
        }
    }
    count
}

fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<i32>>, m: usize, n: usize) {
    let offset = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    grid[x][y] = 2;
    let (x, y) = (x as i32, y as i32);
    for i in 0..4 {
        let nx = x + offset[i].0;
        let ny = y + offset[i].1;
        if nx < 0 || ny < 0 || nx >= m as i32 || ny >= n as i32 || grid[nx as usize][ny as usize] != 1 {
            continue;
        }
        dfs(nx as usize, ny as usize, grid, m, n)
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
