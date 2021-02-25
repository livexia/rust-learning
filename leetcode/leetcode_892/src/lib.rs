/**
给你一个 n * n 的网格 grid ，上面放置着一些 1 x 1 x 1 的正方体。

每个值 v = grid[i][j] 表示 v 个正方体叠放在对应单元格 (i, j) 上。

放置好正方体后，任何直接相邻的正方体都会互相粘在一起，形成一些不规则的三维形体。

请你返回最终这些形体的总表面积。

注意：每个形体的底面也需要计入表面积中。

示例 1：
输入：grid = [[2]]
输出：10

示例 2：
输入：grid = [[1,2],[3,4]]
输出：34

示例 3：
输入：grid = [[1,0],[0,2]]
输出：16

示例 4：
输入：grid = [[1,1,1],[1,0,1],[1,1,1]]
输出：32

示例 5：
输入：grid = [[2,2,2],[2,1,2],[2,2,2]]
输出：46
 
提示：

n == grid.length
n == grid[i].length
1 <= n <= 50
0 <= grid[i][j] <= 50

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/surface-area-of-3d-shapes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut res = 0;

    for i in 0..n {
        for j in 0..n {
            // if grid[i][j] != 0 { res += 2 };
            // if j + 1 < n && grid[i][j] > grid[i][j+1] { res += grid[i][j] - grid[i][j+1] };
            // if j > 0 && grid[i][j] > grid[i][j-1] { res += grid[i][j] - grid[i][j-1] };
            // if i + 1 < n && grid[i][j] > grid[i+1][j] { res += grid[i][j] - grid[i+1][j] };
            // if i > 0 && grid[i][j] > grid[i-1][j] { res += grid[i][j] - grid[i-1][j] };
            // if i == 0 { res += grid[i][j] };
            // if j == 0 { res += grid[i][j] };
            // if i == n - 1 { res += grid[i][j] };
            // if j == n - 1 { res += grid[i][j] };
            if grid[i][j] > 0 {
                res += grid[i][j] * 2 + 1;
                if i > 0 {
                    res -= grid[i-1][j].min(grid[i][j]);
                }
                if j > 0 {
                    res -= grid[i][j-1].min(grid[i][j]);
                }
            }
        }
    }
    res *= 2;
    res
}

#[cfg(test)]
mod tests {
    use crate::surface_area;

    #[test]
    fn it_works() {
        // assert_eq!(surface_area(vec![vec![2]]), 10);
        // assert_eq!(surface_area(vec![vec![1]]), 6);
        assert_eq!(surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    }
}
