/**

给定一个非负整数 num。对于 0 ≤ i ≤ num 范围中的每个数字 i ，计算其二进制数中的 1 的数目并将它们作为数组返回。

示例 1:
输入: 2
输出: [0,1,1]

示例 2:
输入: 5
输出: [0,1,1,2,1,2]
进阶:

给出时间复杂度为O(n*sizeof(integer))的解答非常容易。但你可以在线性时间O(n)内用一趟扫描做到吗？
要求算法的空间复杂度为O(n)。
你能进一步完善解法吗？要求在C++或任何其他语言中不使用任何内置函数（如 C++ 中的 __builtin_popcount）来执行此操作。
*/

pub fn count_bits(num: i32) -> Vec<i32> {
    let mut res = vec![0; num as usize + 1];
    for i in 1..=num as usize {
        if i % 2 == 1 {
            res[i] = res[i-1] + 1;
        } else {
            res[i] = res[i/2];
        }
    }
    // (1..=num as usize).fold(vec![0], |mut v, i| { v.push(&v[i & (i - 1)] + 1); v })
    res
}

#[cfg(test)]
mod tests {
    use crate::count_bits;

    #[test]
    fn it_works() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
