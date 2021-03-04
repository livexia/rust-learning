/**
在二维平面上计算出两个由直线构成的矩形重叠后形成的总面积。

每个矩形由其左下顶点和右上顶点坐标表示，如图所示。

示例:
输入: -3, 0, 3, 4, 0, -1, 9, 2
输出: 45
说明: 假设矩形面积不会超出 int 的范围。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/rectangle-area
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    let area1 = compute_single_area(a, b, c, d);
    let area2 = compute_single_area(e, f, g, h);
    if c <= e || h <= b || g <= a || d <=f {
        return area1 + area2;
    }
    let (x1, y1) = (a.max(e), b.max(f));
    let (x2, y2) = (c.min(g), d.min(h));
    area1 - compute_single_area(x1, y1, x2, y2) + area2
}

fn compute_single_area(a: i32, b: i32, c: i32, d: i32) -> i32 {
    return (c-a) * (d-b)
}

#[cfg(test)]
mod tests {
    use crate::compute_area;

    #[test]
    fn it_works() {
        assert_eq!(compute_area(-3, 0, 3, 4, 0, -1 ,9, 2), 45);
    }
}
