/**
给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。

以上是柱状图的示例，其中每个柱子的宽度为 1，给定的高度为 [2,1,5,6,2,3]。

图中阴影部分为所能勾勒出的最大矩形面积，其面积为 10 个单位。

示例:

输入: [2,1,5,6,2,3]
输出: 10

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/largest-rectangle-in-histogram
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn largest_rectangle_area_brute_force(heights: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = heights.len();

    for i in 0..n {
        let mut width = heights[i];
        let mut area = width;
        for k in (0..i).rev() {
            width = width.min(heights[k]);
            area = area.max((i - k + 1) as i32 * width);
        }
        res = res.max(area);
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::largest_rectangle_area_brute_force;

    #[test]
    fn it_works() {
        assert_eq!(largest_rectangle_area_brute_force(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
