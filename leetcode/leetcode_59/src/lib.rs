/**
给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。

示例 1：
输入：n = 3
输出：[[1,2,3],[8,9,4],[7,6,5]]

示例 2：
输入：n = 1
输出：[[1]]

提示：

1 <= n <= 20

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/spiral-matrix-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut answer = vec![vec![0; n]; n];
    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;
    let mut cur = 1;
    while top <= right && left <= right {
        for j in left..=right {
            answer[top][j] = cur;
            cur += 1;
        }
        for i in top+1..=bottom {
            answer[i][right] = cur;
            cur += 1;
        }
        if left < right && top < bottom {
            for j in (left+1..right).rev() {
                answer[bottom][j] = cur;
                cur += 1;
            }
            for i in (top+1..=bottom).rev() {
                answer[i][left] = cur;
                cur += 1;
            }
        }
        left += 1;
        if right > 0 { right -= 1 };
        top += 1;
        if bottom > 0 { bottom -= 1 };
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::generate_matrix;

    #[test]
    fn it_works() {
        assert_eq!(generate_matrix(3), vec![vec![]]);
    }
}
