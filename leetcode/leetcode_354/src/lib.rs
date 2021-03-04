/**
给定一些标记了宽度和高度的信封，宽度和高度以整数对形式 (w, h) 出现。当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。

请计算最多能有多少个信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。

说明:
不允许旋转信封。

示例:
输入: envelopes = [[5,4],[6,4],[6,7],[2,3]]
输出: 3 
解释: 最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/russian-doll-envelopes
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_by(
        |a, b| 
        if a[0] != b[0] { a[0].cmp(&b[0]) } else { b[1].cmp(&a[1]) } );
    let mut piles = 0;
    let length = envelopes.len();
    let mut top = vec![0; length];
    for i in 0..length {
        let poker = envelopes[i][1];
        let mut left = 0;
        let mut right = piles;
        while left < right {
            let mid = (left + right) / 2;
            if top[mid] >= poker {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == piles {
            piles += 1;
        }
        top[left] = poker;
    }
    piles as i32
}

#[cfg(test)]
mod tests {
    use crate::max_envelopes;

    #[test]
    fn it_works() {
        assert_eq!(max_envelopes(vec![vec![5,4],vec![6,4],vec![6,7],vec![2,3]]), 3);
    }
}
