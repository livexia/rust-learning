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

pub fn largest_rectangle_area_brute_force2(heights: Vec<i32>) -> i32 {
    let mut res = 0;
    let n = heights.len();

    for i in 0..n {
        let mut left = i;
        let mut right  = i;
        let height = heights[i];

        while left > 0 && heights[left - 1] >= height {
            left -= 1;
        }
        while right < n - 1 && heights[right + 1] >= height {
            right += 1;
        }
        res = res.max((right - left + 1) as i32 * height);
    }
    res
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut left = vec![-1; n];
    let mut right = vec![n as i32; n];
    let mut ans = 0;

    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
            stack.pop();
        }
        if !stack.is_empty() {
            left[i] = stack[stack.len() - 1] as i32;
        }
        stack.push(i);
    }
    
    let mut stack: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
            stack.pop();
        }
        if !stack.is_empty() {
            right[i] = stack[stack.len() - 1] as i32;
        }
        stack.push(i);
    }
    for i in 0..n {
        ans = ans.max((right[i] - left[i] - 1) * heights[i])
    }
    ans
}

pub fn largest_rectangle_area2(heights: Vec<i32>) -> i32 {
    let n = heights.len();
    let mut left = vec![-1; n];
    let mut right = vec![n as i32; n];
    let mut ans = 0;

    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        while !stack.is_empty() && heights[stack[stack.len() - 1]] >= heights[i] {
            right[stack[stack.len() - 1]] = i as i32;
            stack.pop();
        }
        if !stack.is_empty() {
            left[i] = stack[stack.len() - 1] as i32;
        }
        stack.push(i);
    }
    for i in 0..n {
        ans = ans.max((right[i] - left[i] - 1) * heights[i])
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::largest_rectangle_area2;

    #[test]
    fn it_works() {
        assert_eq!(largest_rectangle_area2(vec![6, 7, 5, 2, 4, 5, 9, 3]), 16);
        assert_eq!(largest_rectangle_area2(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_area2(vec![2, 4]), 4);
    }
}
