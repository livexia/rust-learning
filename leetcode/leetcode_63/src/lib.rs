/*
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？

网格中的障碍物和空位置分别用 1 和 0 来表示。

示例 1：
输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
输出：2
解释：
3x3 网格的正中间有一个障碍物。
从左上角到右下角一共有 2 条不同的路径：
1. 向右 -> 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右 -> 向右

示例 2：
输入：obstacleGrid = [[0,1],[0,0]]
输出：1

提示：

m == obstacleGrid.length
n == obstacleGrid[i].length
1 <= m, n <= 100
obstacleGrid[i][j] 为 0 或 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/unique-paths-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut path_grid = vec![vec![1; n]; m];
    
    for i in 0..m {
        for j in 0..n {
            if obstacle_grid[i][j] != 1 {
                if i == 0 && j > 0 {
                    path_grid[i][j] = path_grid[i][j - 1];
                } else if j == 0 && i > 0 {
                    path_grid[i][j] = path_grid[i - 1][j];
                } else if i > 0 && j > 0 {
                    path_grid[i][j] = path_grid[i][j - 1] + path_grid[i - 1][j];
                }
            } else {
                path_grid[i][j] = 0;
            }
        }
    }
    println!("{:?}", path_grid);
    path_grid[m-1][n-1]
}

#[test]
fn t1() {
    assert_eq!(2, unique_paths_with_obstacles(vec![
        vec![0,0,0],
        vec![0,1,0],
        vec![0,0,0]]));
}
#[test]
fn t2() {
    assert_eq!(0, unique_paths_with_obstacles(vec![
        vec![1],
        vec![0]]));
}
