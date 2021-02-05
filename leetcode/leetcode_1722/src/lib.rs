/*
给你两个整数数组 source 和 target ，长度都是 n 。还有一个数组 allowedSwaps ，其中每个 allowedSwaps[i] = [ai, bi] 表示你可以交换数组 source 中下标为 ai 和 bi（下标从 0 开始）的两个元素。注意，你可以按 任意 顺序 多次 交换一对特定下标指向的元素。

相同长度的两个数组 source 和 target 间的 汉明距离 是元素不同的下标数量。形式上，其值等于满足 source[i] != target[i] （下标从 0 开始）的下标 i（0 <= i <= n-1）的数量。

在对数组 source 执行 任意 数量的交换操作后，返回 source 和 target 间的 最小汉明距离 。

示例 1：
输入：source = [1,2,3,4], target = [2,1,4,5], allowedSwaps = [[0,1],[2,3]]
输出：1
解释：source 可以按下述方式转换：
- 交换下标 0 和 1 指向的元素：source = [2,1,3,4]
- 交换下标 2 和 3 指向的元素：source = [2,1,4,3]
source 和 target 间的汉明距离是 1 ，二者有 1 处元素不同，在下标 3 。

示例 2：
输入：source = [1,2,3,4], target = [1,3,2,4], allowedSwaps = []
输出：2
解释：不能对 source 执行交换操作。
source 和 target 间的汉明距离是 2 ，二者有 2 处元素不同，在下标 1 和下标 2 。

示例 3：
输入：source = [5,1,2,4,3], target = [1,5,4,2,3], allowedSwaps = [[0,4],[4,2],[1,3],[1,4]]
输出：0

提示：
n == source.length == target.length
1 <= n <= 105
1 <= source[i], target[i] <= 105
0 <= allowedSwaps.length <= 105
allowedSwaps[i].length == 2
0 <= ai, bi <= n - 1
ai != bi

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimize-hamming-distance-after-swap-operations
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashMap;

pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
    let mut connections = Vec::new();

    let mut indices = std::collections::HashMap::new();
    for i in 0..source.len() {
        connections.push(i);
        indices.entry(source[i]).or_insert(Vec::new()).push(i);
    }
    for edge in allowed_swaps {
        union(&mut connections, edge[0] as usize, edge[1] as usize);
    }
    let mut res = 0;
    let mut used = vec![false; source.len()];
    for i in 0..source.len() {
        let connected = indices.get(&target[i]);
        if connected.is_none() {
            res += 1;
            continue;
        }
        let mut flag = 1;
        for &j in connected.unwrap() {
            if !used[j] && find(&mut connections, i) == find(&mut connections, j) {
                used[j] = true;
                flag = 0;
                break;
            }
        }
        res += flag;
    }
    res

}

fn find(connections: &mut Vec<usize>, x: usize) -> usize {
    if connections[x] != x {
        connections[x] = find(connections, connections[x])
    }
    connections[x]
}

fn union(connections: &mut Vec<usize>, x: usize, y: usize) {
    let root_x = find(connections, x);
    let root_y = find(connections, y);
    if root_x != root_y {
        connections[root_x] = root_y;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
