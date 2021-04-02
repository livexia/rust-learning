/**
墙壁上挂着一个圆形的飞镖靶。现在请你蒙着眼睛向靶上投掷飞镖。

投掷到墙上的飞镖用二维平面上的点坐标数组表示。飞镖靶的半径为 r 。

请返回能够落在 任意 半径为 r 的圆形靶内或靶上的最大飞镖数。

示例 1：
输入：points = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
输出：4
解释：如果圆形的飞镖靶的圆心为 (0,0) ，半径为 2 ，所有的飞镖都落在靶上，此时落在靶上的飞镖数最大，值为 4 。

示例 2：
输入：points = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
输出：5
解释：如果圆形的飞镖靶的圆心为 (0,4) ，半径为 5 ，则除了 (7,8) 之外的飞镖都落在靶上，此时落在靶上的飞镖数最大，值为 5 。

示例 3：
输入：points = [[-2,0],[2,0],[0,2],[0,-2]], r = 1
输出：1

示例 4：
输入：points = [[1,2],[3,5],[1,-1],[2,3],[4,1],[1,3]], r = 2
输出：4
 

提示：
1 <= points.length <= 100
points[i].length == 2
-10^4 <= points[i][0], points[i][1] <= 10^4
1 <= r <= 5000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn num_points(points: Vec<Vec<i32>>, r: i32) -> i32 {
    let n = points.len();
    let mut ans = 0;
    let r = r as f64;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                let mut count = 0;
                for k in 0..n {
                    let a = Point{ x: points[i][0] as f64, y: points[i][1] as f64 };
                    let b = Point{ x: points[k][0] as f64, y: points[k][1] as f64 };
                    let temp = dist(&a, &b);
                    if temp <= r { count+= 1 }
                }
                ans = ans.max(count);
            } else {
                let a = Point{ x: points[i][0] as f64, y: points[i][1] as f64 };
                let b = Point{ x: points[j][0] as f64, y: points[j][1] as f64 };
                let d = dist(&a, &b);
                if d/2.0 > r { continue; }

                let res = center(&a, &b, r);
                let mut count = 0;
                for k in 0..n {
                    let c = Point{ x: points[k][0] as f64, y: points[k][1] as f64 };
                    let temp = dist(&res, &c);
                    if temp <= r { count += 1 }
                }
                ans = ans.max(count);
            }
        }
    }
    ans
}

struct Point {
    x: f64,
    y: f64
}

fn dist(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn mid(a: &Point, b: &Point) -> Point {
    Point { x: (a.x + b.x) / 2.0, y: (a.y + b.y) / 2.0 } 
}

fn center(a: &Point, b: &Point, r: f64) -> Point {
    let mid = mid(a, b);
    let d = dist(a, &mid);
    let h = (r.powi(2) - d.powi(2)).sqrt();
    let ba = Point { x: b.x -a.x, y: b.y - a.y };
    let mut hd = Point{ y: ba.x, x: -ba.y };
    let len = dist(&hd, &Point{ x: 0.0, y: 0.0 });
    hd.x /= len;
    hd.y /= len;
    hd.x *= h;
    hd.y *= h;
    Point { x: hd.x + mid.x, y: hd.y + mid.y }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
