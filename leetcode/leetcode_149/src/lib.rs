/**
给你一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点。求最多有多少个点在同一条直线上。

示例 1：
输入：points = [[1,1],[2,2],[3,3]]
输出：3

示例 2：
输入：points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
输出：4

提示：
1 <= points.length <= 300
points[i].length == 2
-104 <= xi, yi <= 104
points 中的所有点 互不相同

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/max-points-on-a-line
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if b != 0 { gcd(b, a % b) } else { a }
    }
    let n = points.len();
    if n < 3 {
        return n as i32
    }
    let mut res =0;
    for (i, p1) in points.iter().enumerate() {
        let mut count = HashMap::new();
        for j in (i + 1)..n {
            let mut dx = p1[0] - points[j][0];
            let mut dy = p1[1] - points[j][1];
            if dx == 0 {
                dy = 1;
            } else if dy == 0 {
                dx = 1;
            } else {
                if dy < 0 {
                    dx = -dx;
                    dy = -dy;
                }
                let gdc_xy = gcd(dx, dy);
                dx /= gdc_xy;
                dy /= gdc_xy;
            }
            *count.entry(dy + 20001 * dx).or_insert(0) += 1;
        }
        res = res.max(*count.values().max().unwrap_or(&0) + 1);
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
