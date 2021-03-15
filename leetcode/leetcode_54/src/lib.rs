/**
给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。

示例 1：
输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[1,2,3,6,9,8,7,4,5]

示例 2：
输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
输出：[1,2,3,4,8,12,11,10,9,5,6,7]
 
提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 10
-100 <= matrix[i][j] <= 100

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/spiral-matrix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left = 0;
    let mut right = matrix[0].len() - 1;
    let mut top = 0;
    let mut bottom = matrix.len() - 1;
    let mut answer = vec![];

    while left <= right && top <= bottom {
        for j in left..=right {
            answer.push(matrix[top][j])
        }
        for i in top+1..=bottom {
            answer.push(matrix[i][right])
        }
        if left < right && top < bottom {
            for j in (left+1..right).rev() {
                answer.push(matrix[bottom][j])
            }
            for i in (top+1..=bottom).rev() {
                answer.push(matrix[i][left])
            }
        }
        left += 1;
        if right > 0 {
            right -= 1;
        }
        top += 1;
        if bottom > 0 {
            bottom -= 1;
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use crate::spiral_order;

    #[test]
    fn t1() {
        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        assert_eq!(spiral_order(matrix), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }

    #[test]
    fn t2() {
        let matrix = vec![vec![1]];
        assert_eq!(spiral_order(matrix), vec![1]);
    }

    #[test]
    fn t3() {
        let matrix = vec![vec![1,2], vec![3,4]];
        assert_eq!(spiral_order(matrix), vec![1,2,4,3]);
    }
}
