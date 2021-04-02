/**
给定一个直方图(也称柱状图)，假设有人从上面源源不断地倒水，最后直方图能存多少水量?直方图的宽度为 1。

上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的直方图，在这种情况下，可以接 6 个单位的水（蓝色部分表示水）。 感谢 Marcos 贡献此图。

示例:
输入: [0,1,0,2,1,0,1,3,2,1,2,1]
输出: 6

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/volume-of-histogram-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn trap(height: Vec<i32>) -> i32 {
    let sum: i32 = height.iter().sum();
    let n = height.len();
    if n == 0 { return 0 }
    let (mut left, mut right) = (0, n - 1);
    let (mut volume, mut high) = (0, 1);
    while left <= right {
        while left <= right && height[left] < high {
            left += 1
        }
        while left <= right && height[right] < high {
            right -= 1
        }
        volume += right - left + 1;
        high += 1
    }
    volume as i32 - sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
