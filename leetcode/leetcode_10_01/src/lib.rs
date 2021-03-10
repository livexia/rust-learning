/**
给定两个排序后的数组 A 和 B，其中 A 的末端有足够的缓冲空间容纳 B。 编写一个方法，将 B 合并入 A 并排序。

初始化 A 和 B 的元素数量分别为 m 和 n。

示例:
输入:
A = [1,2,3,0,0,0], m = 3
B = [2,5,6],       n = 3

输出: [1,2,2,3,5,6]

说明:
A.length == n + m

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sorted-merge-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn merge(a: &mut Vec<i32>, m: i32, b: &mut Vec<i32>, n: i32) {
    let mut pa = m - 1;
    let mut pb = n - 1;
    let mut tail = (n + m - 1) as usize;
    while pa >= 0 || pb >= 0 {
        let cur;
        if pa == -1 {
            cur = b[pb as usize];
            pb -= 1;
        } else if pb == -1 {
            cur = a[pa as usize];
            pa -= 1;
        } else if a[pa as usize] > b[pb as usize] {
            cur = a[pa as usize];
            pa -= 1;
        } else {
            cur = b[pb as usize];
            pb -= 1;
        }
        a[tail] = cur;
        if tail > 0 { tail -= 1 }
    }
}

#[cfg(test)]
mod tests {
    use crate::merge;

    #[test]
    fn it_works() {
        let mut a = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut b = vec![2, 5, 6];
        let n = 3;
        merge(&mut a, m, &mut b, n);
        assert_eq!(a, vec![1, 2, 2, 3, 5, 6]);
    }
}
