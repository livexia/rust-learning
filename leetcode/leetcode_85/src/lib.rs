/**
给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。

示例 1：
输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
输出：6
解释：最大矩形如上图所示。

示例 2：
输入：matrix = []
输出：0

示例 3：
输入：matrix = [["0"]]
输出：0

示例 4：
输入：matrix = [["1"]]
输出：1

示例 5：
输入：matrix = [["0","0"]]
输出：0

提示：

rows == matrix.length
cols == matrix[0].length
0 <= row, cols <= 200
matrix[i][j] 为 '0' 或 '1'

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximal-rectangle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

// 方法二在参考学习84题后再进行补充
pub fn maximal_rectangle_brute_force(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
        return 0;
    }
    let n = matrix[0].len();
    let mut left = vec![vec![0 as i32; n]; m];
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == '1' {
                left[i][j] = match j {
                    0 => 1,
                    _ => left[i][j-1] + 1
                }
            }
        }
    }


    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == '0' {
                continue;
            }
            let mut width = left[i][j];
            let mut area = width;
            if i > 0 {
                for k in (0..=i-1).rev() {
                    width = width.min(left[k][j]);
                    area = area.max((i - k + 1) as i32 * width)
                }
            }
            
            res = res.max(area);
        }
    }
    res
}

pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    if m == 0 {
        return 0;
    }
    let n = matrix[0].len();
    let mut left = vec![vec![0 as i32; n]; m];
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == '1' {
                left[i][j] = match j {
                    0 => 1,
                    _ => left[i][j-1] + 1
                }
            }
        }
    }
    let mut res = 0;
    for j in 0..n {
        let mut up = vec![-1; m];
        let mut down = vec![m as i32; m];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..m {
            while !stack.is_empty() && left[stack[stack.len() - 1]][j] >= left[i][j] {
                down[stack[stack.len() - 1]] = i as i32;
                stack.pop();
            }
            if !stack.is_empty() {
                up[i] = stack[stack.len() - 1] as i32;
            }
            stack.push(i);
        }

        for i in 0..m {
            res = res.max((down[i] - up[i] - 1)*left[i][j]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::maximal_rectangle;

    #[test]
    fn t1() {
        let matrix = vec![vec!['1','0','1','0','0'], vec!['1','0','1','1','1'], vec!['1','1','1','1','1'], vec!['1','0','0','1','0']];
        assert_eq!(maximal_rectangle(matrix), 6);
    }

    #[test]
    fn t2() {
        let matrix = vec![vec![]];
        assert_eq!(maximal_rectangle(matrix), 0);
    }

    #[test]
    fn t3() {
        let matrix = vec![vec!['0']];
        assert_eq!(maximal_rectangle(matrix), 0);
    }

    #[test]
    fn t4() {
        let matrix = vec![vec!['1']];
        assert_eq!(maximal_rectangle(matrix), 1);
    }

    #[test]
    fn t5() {
        let matrix = vec![vec!['0', '0']];
        assert_eq!(maximal_rectangle(matrix), 0);
    }
}
