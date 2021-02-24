/**
给定一个二进制矩阵 A，我们想先水平翻转图像，然后反转图像并返回结果。

水平翻转图片就是将图片的每一行都进行翻转，即逆序。例如，水平翻转 [1, 1, 0] 的结果是 [0, 1, 1]。

反转图片的意思是图片中的 0 全部被 1 替换， 1 全部被 0 替换。例如，反转 [0, 1, 1] 的结果是 [1, 0, 0]。

示例 1:
输入: [[1,1,0],[1,0,1],[0,0,0]]
输出: [[1,0,0],[0,1,0],[1,1,1]]
解释: 首先翻转每一行: [[0,1,1],[1,0,1],[0,0,0]]；
     然后反转图片: [[1,0,0],[0,1,0],[1,1,1]]

示例 2:
输入: [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
输出: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
解释: 首先翻转每一行: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]]；
     然后反转图片: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]

说明:
1 <= A.length = A[0].length <= 20
0 <= A[i][j] <= 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/flipping-an-image
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn flip_and_invert_image_brute(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter().map(|x| x.iter().rev().map(|x| x ^ 1).collect()).collect()
}

pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut a = a;

    for i in 0..n {
        let (mut left, mut right) = (0, n - 1);
            while left < right {
                if a[i][left] == a[i][right] {
                    a[i][left] ^= 1;
                    a[i][right] ^= 1;
                }
                left += 1;
                right -= 1;
            }
            if left == right {
                a[i][left] ^= 1;
            }
    }
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
