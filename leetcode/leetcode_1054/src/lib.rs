/**
在一个仓库里，有一排条形码，其中第 i 个条形码为 barcodes[i]。

请你重新排列这些条形码，使其中两个相邻的条形码 不能 相等。 你可以返回任何满足该要求的答案，此题保证存在答案。

示例 1：
输入：[1,1,1,2,2,2]
输出：[2,1,2,1,2,1]

示例 2：
输入：[1,1,1,1,2,2,3,3]
输出：[1,3,1,3,2,1,2,1]
 
提示：

1 <= barcodes.length <= 10000
1 <= barcodes[i] <= 10000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/distant-barcodes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
    let mut counter = HashMap::new();
    let mut barcodes = barcodes;
    barcodes.sort();
    let n = barcodes.len();
    let mut res = vec![0; n];
    for i in barcodes.clone() {
        *counter.entry(i).or_insert(0) += 1;
    }
    barcodes.sort_by(|a, b| counter[b].cmp(&counter[a]));
    barcodes.reverse();
    println!("{:?}", barcodes);
    println!("{:?}", counter);
    for i in (0..n).step_by(2) {
        res[i] = barcodes.pop().unwrap();
    }
    for i in (1..n).step_by(2) {
        res[i] = barcodes.pop().unwrap();
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::rearrange_barcodes;

    #[test]
    fn t1() {
        assert_eq!(rearrange_barcodes(vec![1,1,1,1,2,2,3,3]), vec![1,3,1,3,2,1,2,1]);
    }

    #[test]
    fn t2() {
        assert_eq!(rearrange_barcodes(vec![1]), vec![1]);
    }

    #[test]
    fn t3() {
        assert_eq!(rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]), vec![1, 2, 1, 2, 1, 2]);
    }

    #[test]
    fn t4() {
        assert_eq!(rearrange_barcodes(vec![1, 1, 1, 2, 2]), vec![1, 2, 1, 2, 1]);
    }

    #[test]
    fn t5() {
        assert_eq!(rearrange_barcodes(vec![7,7,7,8,5,7,5,5,5,8]), vec![5,7,5,7,5,7,5,8,7,8]);
    }
}
