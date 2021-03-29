/**
编写一个高效的算法来判断 m x n 矩阵中，是否存在一个目标值。该矩阵具有如下特性：

每行中的整数从左到右按升序排列。
每行的第一个整数大于前一行的最后一个整数。

示例 1：
输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
输出：true

示例 2：
输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
输出：false

提示：
m == matrix.length
n == matrix[i].length
1 <= m, n <= 100
-104 <= matrix[i][j], target <= 104

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/search-a-2d-matrix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // let flat_mattrix: Vec<i32> = matrix.into_iter().flatten().collect();
    // match flat_mattrix.binary_search(&target) {
    //     Ok(_) => true,
    //     Err(_) => false,
    // }
    let n = matrix[0].len();
    for row in matrix {
        if row[n-1] >= target {
            match row.binary_search(&target) {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::search_matrix;

    #[test]
    fn it_works() {
        assert_eq!(search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 
            13), true);
    }
}
