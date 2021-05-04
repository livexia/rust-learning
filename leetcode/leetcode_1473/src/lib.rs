/**
在一个小城市里，有 m 个房子排成一排，你需要给每个房子涂上 n 种颜色之一（颜色编号为 1 到 n ）。有的房子去年夏天已经涂过颜色了，所以这些房子不需要被重新涂色。

我们将连续相同颜色尽可能多的房子称为一个街区。（比方说 houses = [1,2,2,3,3,2,1,1] ，它包含 5 个街区  [{1}, {2,2}, {3,3}, {2}, {1,1}] 。）

给你一个数组 houses ，一个 m * n 的矩阵 cost 和一个整数 target ，其中：

houses[i]：是第 i 个房子的颜色，0 表示这个房子还没有被涂色。
cost[i][j]：是将第 i 个房子涂成颜色 j+1 的花费。
请你返回房子涂色方案的最小总花费，使得每个房子都被涂色后，恰好组成 target 个街区。如果没有可用的涂色方案，请返回 -1 。

示例 1：
输入：houses = [0,0,0,0,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
输出：9
解释：房子涂色方案为 [1,2,2,1,1]
此方案包含 target = 3 个街区，分别是 [{1}, {2,2}, {1,1}]。
涂色的总花费为 (1 + 1 + 1 + 1 + 5) = 9。

示例 2：
输入：houses = [0,2,1,2,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
输出：11
解释：有的房子已经被涂色了，在此基础上涂色方案为 [2,2,1,2,2]
此方案包含 target = 3 个街区，分别是 [{2,2}, {1}, {2,2}]。
给第一个和最后一个房子涂色的花费为 (10 + 1) = 11。

示例 3：
输入：houses = [0,0,0,0,0], cost = [[1,10],[10,1],[1,10],[10,1],[1,10]], m = 5, n = 2, target = 5
输出：5

示例 4：
输入：houses = [3,1,2,3], cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]], m = 4, n = 3, target = 3
输出：-1
解释：房子已经被涂色并组成了 4 个街区，分别是 [{3},{1},{2},{3}] ，无法形成 target = 3 个街区。
 
提示：

m == houses.length == cost.length
n == cost[i].length
1 <= m <= 100
1 <= n <= 20
1 <= target <= m
0 <= houses[i] <= n
1 <= cost[i][j] <= 10^4

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/paint-house-iii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    // 文中说houses是以颜色标号1开始，我们把它调整为0开始
    // -1表示未涂色
    // 为了方便后续匹配cost数组
    let mut houses = houses;
    for i in 0..houses.len() {
        houses[i] -= 1;
    }
    
    // 设置一个表示无限大的标记，用来初始化dp表，因为最后是求最小花费，这样会在后续比较中方便一些
    let INFTY = i32::MAX >> 1;
    
    // dp[i][j][k]的含义是：第i个房屋(0<=i<m)涂第j个颜色(0<=j<n)时属于第k个街区(0<=k<target)的最小花费
    // 注意这里index[0]代表第1个
    let mut dp = vec![vec![vec![INFTY; target as usize]; n as usize]; m as usize];
    
    // 我们从第1个房屋开始刷起(初始状态),也就是i=0时
    // 同样我们也知道，当i=0时，只能有一个街区(k=0)，所以k也一定为零
    if houses[0] != -1 {
        // 如果第一个房屋已着色，需要设置dp[0][j][0]为0，其它不变（保持无限大）
        // 这里j=house[0]代表它已着的颜色，已着当然就不用再着色了，所以花费为0
        dp[0][houses[0] as usize][0] = 0;
    } else {
        // 如果第一个房子未着色，这里我们需要把它着色
        // 我们不用管k，k保持0，因为只存在一个街区
        for j in 0..n as usize {
            dp[0][j][0] = cost[0][j];
        }
    }
    
    // 下面我们考虑后续的状态
    // 从第2个房子开始刷起
    for i in 1..m as usize {
        if houses[i] != -1 {
            // 如果当前房屋已被着色
            // 题中要求不能给已着色的房屋着色，所以对于dp[i][j][k]中j!=houses[i]的地方保持无限大
            // 只用考虑j=houses[i]时的情况
            // j=houses[i]时本轮cost为0
            
            for k in 0..target as usize {
                 // 这里具体考虑本次作为第k个街区时的最小花费
                for j0 in 0..n as usize {
                    // 这里我们需要进一步遍历前一个房屋的所有颜色
                    // 找到对于当前房屋所属的街区数k，对于前一个房屋不同颜色j0的情况下的最小值
                    
                    if houses[i] == j0 as i32 {
                        // 如果当前房屋和前一个房屋颜色相同(houses[i]=j0)
                        // 所以当前房屋所属街区与前一个房屋所属街区一样，即k与之前相同
                        dp[i][houses[i] as usize][k] = dp[i][houses[i] as usize][k].min(dp[i - 1][j0][k])
                    } else {
                        // 如果当前房屋和前一个房屋颜色不同(houses[i]!=j0)
                        // 则当前所属的街区比前一个多一，所以此时的状态是从dp[i - 1][j0][k - 1]转移而来
                        // 这里保证k > 0
                        // 因为已经与前一个不同，所以当前不可能是第一个(k=0)街区
                        if k > 0 {
                            // 这里是为了找到对于固定k-1，所有不同的j0的最小值
                            dp[i][houses[i] as usize][k] = dp[i][houses[i] as usize][k].min(dp[i - 1][j0][k - 1]);
                        }
                    }
                }
            }
        } else {
            // 这里表示当前房屋未被着色
            for j in 0..n as usize {
                // 给房屋着哪种颜色呢？
                for k in 0..target as usize {
                    // 刷这种颜色时当前房屋的街区数
                    for j0 in 0..n as usize {
                        // 前一个房屋的颜色状态
                        if j == j0 {
                            // 如果当前房屋要涂的颜色与前一房屋相同
                            // 表示当前房屋的街区数与前一房屋的街区数相同
                            dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j0][k])
                        } else {
                            // 如果当前房屋要涂的颜色与前一房屋不同
                            // 当前房屋街区数(k)要比前一房屋街区数(k-1)多1
                            // 这里需要找到前一房屋(i-1)中街区数为(k-1)的最小花费
                            // 同样这里要求k大于0（当前房屋所属街区与前一个房屋不同，所以k不可能为0）
                            if k > 0 {
                                dp[i][j][k] = dp[i][j][k].min(dp[i - 1][j0][k - 1])
                            }
                        }
                    }
                    
                    // 这里要加上着色花费
                    // 如果当前值为INFTY表示无意义就不管了
                    if dp[i][j][k] != INFTY {
                        dp[i][j][k] += cost[i][j];
                    }
                }
            }
        }
    }
    
    // 最后需要我们找到i = m - 1, k = target - 1时，不同j的最小值
    let mut ans = INFTY;
    for j in 0..n as usize {
        ans = ans.min(dp[m as usize - 1][j][target as usize - 1])
    }
    
    if ans == INFTY {
        return -1;
    }
    
    ans
}

// 作者：akson
// 链接：https://leetcode-cn.com/problems/paint-house-iii/solution/fei-chang-xiang-xi-de-zhu-shi-jie-shi-by-r6ar/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
