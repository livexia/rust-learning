/**
今天，书店老板有一家店打算试营业 customers.length 分钟。每分钟都有一些顾客（customers[i]）会进入书店，所有这些顾客都会在那一分钟结束后离开。

在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。 当书店老板生气时，那一分钟的顾客就会不满意，不生气则他们是满意的。

书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 X 分钟不生气，但却只能使用一次。

请你返回这一天营业下来，最多有多少客户能够感到满意的数量。

示例：
输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
输出：16
解释：
书店老板在最后 3 分钟保持冷静。
感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.

提示：
1 <= X <= customers.length == grumpy.length <= 20000
0 <= customers[i] <= 1000
0 <= grumpy[i] <= 1

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/grumpy-bookstore-owner
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut res = 0;
    let mut right = 0;
    let mut count = 0;
    let l = customers.len();
    for i in 0..l {
        if grumpy[i] == 0 {
            res += customers[i];
        }
    }
    let mut sum = res;

    while right < l {
        if grumpy[right] == 1 {
            count += 1;
            sum += customers[right];
        } else if count != 0 {
            count += 1;
        }
        if count as i32 > x {
            count -= 1;
            if grumpy[right - count] == 1 {
                sum -= customers[right - count];
            }
        }
        res = res.max(sum);
        right += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::max_satisfied;

    #[test]
    fn t1() {
        assert_eq!(max_satisfied(vec![1,0,1,2,1,1,7,5], vec![0,1,0,1,0,1,0,1], 3), 16);
    }

    #[test]
    fn t2() {
        assert_eq!(max_satisfied(vec![2,6,7,9], vec![0,0,1,1], 1), 17);
    }
}
