/**
给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。

在杨辉三角中，每个数是它左上方和右上方的数的和。

示例:
输入: 3
输出: [1,3,3,1]

进阶：
你可以优化你的算法到 O(k) 空间复杂度吗？

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/pascals-triangle-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

// pub fn get_row(row_index: i32) -> Vec<i32> {
//     let mut res: Vec<i32> = Vec::new();
//     res.push(1);
//     for i in 1..=row_index {
//         res.push((res[i as usize - 1] as i128 * (row_index - i + 1) as i128 / i as i128) as i32 )
//     }
//     res
// }

pub fn get_row(row_index: i32) -> Vec<i32> {
    let row_index: usize = row_index as usize + 1;
    let mut res: Vec<i32> = vec![1; row_index];

    for i in 2..row_index {
        for j in (1..i).rev() {
            res[j] += res[j-1]
        }
    }
    
    res
}

#[cfg(test)]
mod tests {
    use crate::get_row;

    #[test]
    fn it_works() {
        assert_eq!(get_row(30), vec![1,30,435,4060,27405,142506,593775,2035800,5852925,14307150,30045015,54627300,86493225,119759850,145422675,155117520,145422675,119759850,86493225,54627300,30045015,14307150,5852925,2035800,593775,142506,27405,4060,435,30,1]);
    }

    #[test]
    fn t2() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    }
}
