/**
给你一个 m x n 的矩阵 matrix 和一个整数 k ，找出并返回矩阵内部矩形区域的不超过 k 的最大数值和。

题目数据保证总会存在一个数值和不超过 k 的矩形区域。

示例 1：
输入：matrix = [[1,0,1],[0,-2,3]], k = 2
输出：2
解释：蓝色边框圈出来的矩形区域 [[0, 1], [-2, 3]] 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。

示例 2：
输入：matrix = [[2,2,-1]], k = 3
输出：3

提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 100
-100 <= matrix[i][j] <= 100
-105 <= k <= 105
 

进阶：如果行数远大于列数，该如何设计解决方案？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::BTreeSet;

pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut sum = vec![vec![0; m+ 1]; n];
    for i in 0..m {
        for j in 0..n {
            sum[j][i + 1] = sum[j][i] + matrix[i][j];
        }
    }
    let mut r = i32::MIN;
    let mut s = BTreeSet::new();
    for i in 0..m {
        for j in i..m {
            let mut x = 0;
            s.clear();
            s.insert(x);
            for l in 0..n {
                x += sum[l][j + 1] - sum[l][i];
                if let Some(z) = s.range(x - k..).next() {
                    r = r.max(x - z);
                }
                s.insert(x);
            }
        }
    }
    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
