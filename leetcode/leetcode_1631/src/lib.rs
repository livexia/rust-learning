/*
你准备参加一场远足活动。给你一个二维 rows x columns 的地图 heights ，其中 heights[row][col] 表示格子 (row, col) 的高度。一开始你在最左上角的格子 (0, 0) ，且你希望去最右下角的格子 (rows-1, columns-1) （注意下标从 0 开始编号）。你每次可以往 上，下，左，右 四个方向之一移动，你想要找到耗费 体力 最小的一条路径。
一条路径耗费的 体力值 是路径上相邻格子之间 高度差绝对值 的 最大值 决定的。
请你返回从左上角走到右下角的最小 体力消耗值 。

示例 1：
输入：heights = [[1,2,2],[3,8,2],[5,3,5]]
输出：2
解释：路径 [1,3,5,3,5] 连续格子的差值绝对值最大为 2 。
这条路径比路径 [1,2,2,2,5] 更优，因为另一条路径差值最大值为 3 。

示例 2：
输入：heights = [[1,2,3],[3,8,4],[5,3,5]]
输出：1
解释：路径 [1,2,3,4,5] 的相邻格子差值绝对值最大为 1 ，比路径 [1,3,5,3,5] 更优。

示例 3：
输入：heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
输出：0
解释：上图所示路径不需要消耗任何体力。

提示：

rows == heights.length
columns == heights[i].length
1 <= rows, columns <= 100
1 <= heights[i][j] <= 106

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/path-with-minimum-effort
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let row = heights.len();
    let col = heights[0].len();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    let mut table: Vec<usize> = Vec::new();
    for i in 0..(row*col) {
        table.push(i);
    }
    for i in 0..row {
        for j in 0..col {
            let next_row = i + 1;
            let next_col = j;
            if next_row < row && next_col < col {
                edges.push((
                    i * col + j, 
                    next_row * col + next_col, 
                    (heights[i][j] - heights[next_row][next_col]).abs() as usize))
            }
            let next_row = i;
            let next_col = j + 1;
            if next_row < row && next_col < col {
                edges.push((
                    i * col + j, 
                    next_row * col + next_col, 
                    (heights[i][j] - heights[next_row][next_col]).abs() as usize))
            }
        }
    }
    edges.sort_by(|a, b| a.2.cmp(&b.2));
    println!("{:?}", edges);
    
    let mut max_effort: usize = 0;
    for edge in edges {
        if find(&mut table, 0) == find(&mut table, row * col - 1) {
            break;
        }
        let (cur_point, next_point, effort) = edge;
        if find(&mut table, cur_point) != find(&mut table, next_point) {
            union(&mut table, cur_point, next_point);
            max_effort = effort.max(max_effort)
        }
    }
    max_effort as i32
}

fn find(table: &mut Vec<usize>, i: usize) -> usize {
    if table[i] != i {
        table[i] = find(table, table[i])
    }
    table[i]
}

fn union(table: &mut Vec<usize>, i: usize, j: usize) {
    let root_i = find(table, i);
    let root_j = find(table, j);
    table[root_i] = root_j;
}

#[test]
fn test1() {
    assert_eq!(2, minimum_effort_path(vec![
        vec![1,2,2],
        vec![3,8,2],
        vec![5,3,5]
    ]));
}


#[test]
fn test2() {
    assert_eq!(1, minimum_effort_path(vec![
        vec![1,2,3],
        vec![3,8,4],
        vec![5,3,5]
    ]));
}
