/*
几张卡牌 排成一行，每张卡牌都有一个对应的点数。点数由整数数组 cardPoints 给出。

每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 k 张卡牌。

你的点数就是你拿到手中的所有卡牌的点数之和。

给你一个整数数组 cardPoints 和整数 k，请你返回可以获得的最大点数。

示例 1：

输入：cardPoints = [1,2,3,4,5,6,1], k = 3
输出：12
解释：第一次行动，不管拿哪张牌，你的点数总是 1 。但是，先拿最右边的卡牌将会最大化你的可获得点数。最优策略是拿右边的三张牌，最终点数为 1 + 6 + 5 = 12 。

示例 2：
输入：cardPoints = [2,2,2], k = 2
输出：4
解释：无论你拿起哪两张卡牌，可获得的点数总是 4 。

示例 3：
输入：cardPoints = [9,7,7,9,7,7,9], k = 7
输出：55
解释：你必须拿起所有卡牌，可以获得的点数为所有卡牌的点数之和。

示例 4：
输入：cardPoints = [1,1000,1], k = 1
输出：1
解释：你无法拿到中间那张卡牌，所以可以获得的最大点数为 1 。 

示例 5：
输入：cardPoints = [1,79,80,1,1,1,200,1], k = 3
输出：202

提示：
1 <= cardPoints.length <= 10^5
1 <= cardPoints[i] <= 10^4
1 <= k <= cardPoints.length

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/maximum-points-you-can-obtain-from-cards
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let total_sum: i32 = card_points.iter().sum();

    if k as usize == card_points.len() {
        return total_sum
    }

    let mut left: usize = 0;
    let mut right: usize = card_points.len() - k as usize;
    let mut window_sum: i32 = card_points[left..right].iter().sum();
    println!("{}", window_sum);
    let mut max = total_sum - window_sum;
    
    while right < card_points.len() {
        window_sum += card_points[right] - card_points[left];
        right += 1;
        left += 1;
        max = max.max(total_sum - window_sum);
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::max_score;

    #[test]
    fn t1() {
        assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    }

    #[test]
    fn t2() {
        assert_eq!(max_score(vec![1,2,3,4,5,6,1], 3), 12);
    }

    #[test]
    fn t3() {
        assert_eq!(max_score(vec![9,7,7,9,7,7,9], 7), 55);
    }

    #[test]
    fn t4() {
        assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
    }

    #[test]
    fn t5() {
        assert_eq!(max_score(vec![1,79,80,1,1,1,200,1], 3), 202);
    }

    #[test]
    fn t6() {
        assert_eq!(max_score(vec![100,40,17,9,73,75], 3), 248);
    }
}
