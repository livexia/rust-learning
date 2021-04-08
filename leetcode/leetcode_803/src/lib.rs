/**
有一个 m x n 的二元网格，其中 1 表示砖块，0 表示空白。砖块 稳定（不会掉落）的前提是：

一块砖直接连接到网格的顶部，或者
至少有一块相邻（4 个方向之一）砖块 稳定 不会掉落时
给你一个数组 hits ，这是需要依次消除砖块的位置。每当消除 hits[i] = (rowi, coli) 位置上的砖块时，对应位置的砖块（若存在）会消失，然后其他的砖块可能因为这一消除操作而掉落。一旦砖块掉落，它会立即从网格中消失（即，它不会落在其他稳定的砖块上）。

返回一个数组 result ，其中 result[i] 表示第 i 次消除操作对应掉落的砖块数目。

注意，消除可能指向是没有砖块的空白位置，如果发生这种情况，则没有砖块掉落。

示例 1：
输入：grid = [[1,0,0,0],[1,1,1,0]], hits = [[1,0]]
输出：[2]
解释：
网格开始为：
[[1,0,0,0]，
 [1,1,1,0]]
消除 (1,0) 处加粗的砖块，得到网格：
[[1,0,0,0]
 [0,1,1,0]]
两个加粗的砖不再稳定，因为它们不再与顶部相连，也不再与另一个稳定的砖相邻，因此它们将掉落。得到网格：
[[1,0,0,0],
 [0,0,0,0]]
因此，结果为 [2] 。

示例 2：
输入：grid = [[1,0,0,0],[1,1,0,0]], hits = [[1,1],[1,0]]
输出：[0,0]
解释：
网格开始为：
[[1,0,0,0],
 [1,1,0,0]]
消除 (1,1) 处加粗的砖块，得到网格：
[[1,0,0,0],
 [1,0,0,0]]
剩下的砖都很稳定，所以不会掉落。网格保持不变：
[[1,0,0,0], 
 [1,0,0,0]]
接下来消除 (1,0) 处加粗的砖块，得到网格：
[[1,0,0,0],
 [0,0,0,0]]
剩下的砖块仍然是稳定的，所以不会有砖块掉落。
因此，结果为 [0,0] 。

提示：

m == grid.length
n == grid[i].length
1 <= m, n <= 200
grid[i][j] 为 0 或 1
1 <= hits.length <= 4 * 104
hits[i].length == 2
0 <= xi <= m - 1
0 <= yi <= n - 1
所有 (xi, yi) 互不相同


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/bricks-falling-when-hit
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub mod dsu {
    use std::collections::HashSet;
    use std::mem::swap;

    pub struct DSU {
        pub data: Vec<i32>,
        pub set: HashSet<usize>,
    }

    impl DSU {
        pub fn new(i: usize) -> Self {
            DSU {
                data: vec![-1i32; i],
                set: HashSet::new()
            }
        }

        pub fn show_info(&self) {
            println!(
                "root nums: {:?}\nroots: {:?}\ndata: {:?}",
                self.set.len(), self.set, self.data
            )
        }

        pub fn merge(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.root(x);
            y = self.root(y);
            if x != y {
                if self.data[y] < self.data[x] {
                    swap(&mut x, &mut y);
                }
                if self.data[y] != -1 {
                    self.set.remove(&y);
                }
                if self.data[x] == -1 {
                    self.set.insert(x);
                }
                self.data[x] += self.data[y];
                self.data[y] = x as i32;
            }
            x != y
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.data[x] < 0 {
                x
            } else {
                self.data[x] = self.root(self.data[x] as usize) as i32;
                self.data[x] as usize
            }
        }

        pub fn size(&mut self, x: usize) -> usize {
            let u = self.root(x);
            (-self.data[u]) as usize
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn fields(&self) -> usize {
            self.set.len()
        }

        pub fn roots(&self) -> HashSet<usize> {
            self.set.clone()
        }
    }
}

pub use crate::dsu::*;
fn gt(x: i32, y: i32) -> usize {
    x as usize * 200 + y as usize + 1
}

pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
    let mut grid = grid;
    let n = grid[0].len() as i32;
    grid.insert(0, vec![1; n as usize]);
    let m = grid.len() as i32;
    let mut hits = hits;
    for hit in hits.iter_mut() {
        hit[0] += 1;
        if grid[hit[0] as usize][hit[1] as usize] == 0 {
            hit[0] = 0;
            hit[1] = 0;
        } else {
            grid[hit[0] as usize][hit[1] as usize] = 0;
        }
    }
    let mut union_find = DSU::new(40500);
    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == 1 {
                if i > 0 && grid[i as usize - 1][j as usize] == 1 {
                    union_find.merge(gt(i, j), gt(i - 1, j));
                }
                if j > 0 && grid[i as usize][j as usize - 1] == 1 {
                    union_find.merge(gt(i, j), gt(i, j - 1));
                }
            } 
        }
    }
    let direcs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
    let mut ret = vec![];
    for hit in hits.iter().rev() {
        let temp = union_find.size(1);
        for direc in &direcs {
            let n_x = hit[0] + direc[0];
            let n_y = hit[1] + direc[1];
            if n_x >= 0 && n_x < m && n_y >= 0 && n_y < n 
                && grid[n_x as usize][n_y as usize] == 1 {
                    union_find.merge(gt(n_x, n_y), gt(hit[0], hit[1]));
                }
        }
        if union_find.same(1, gt(hit[0], hit[1])) {
            ret.push((union_find.size(1) as i32 - temp as i32 - 1).max(0))
        } else {
            ret.push(0);
        }
        grid[hit[0] as usize][hit[1] as usize] = 1;
    }
    ret.reverse();
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
