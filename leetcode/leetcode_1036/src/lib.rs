/**
在一个 10^6 x 10^6 的网格中，每个网格块的坐标为 (x, y)，其中 0 <= x, y < 10^6。

我们从源方格 source 开始出发，意图赶往目标方格 target。每次移动，我们都可以走到网格中在四个方向上相邻的方格，只要该方格不在给出的封锁列表 blocked 上。

只有在可以通过一系列的移动到达目标方格时才返回 true。否则，返回 false。

示例 1：
输入：blocked = [[0,1],[1,0]], source = [0,0], target = [0,2]
输出：false
解释：
从源方格无法到达目标方格，因为我们无法在网格中移动。

示例 2：
输入：blocked = [], source = [0,0], target = [999999,999999]
输出：true
解释：
因为没有方格被封锁，所以一定可以到达目标方格。
 
提示：

0 <= blocked.length <= 200
blocked[i].length == 2
0 <= blocked[i][j] < 10^6
source.length == target.length == 2
0 <= source[i][j], target[i][j] < 10^6
source != target

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/escape-a-large-maze
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashSet;

pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    if blocked.is_empty() {
        return true;
    }
    let blocked = blocked.iter()
        .fold(std::collections::HashSet::new(), 
        |mut v,  i| { 
            v.insert((i[0], i[1])); 
            v 
        });
    let source_to_target = bfs((source[0], source[1]), &blocked, (target[0], target[1]));
    let target_to_source = bfs((target[0], target[1]), &blocked, (source[0], source[1]));
    source_to_target && target_to_source
}

fn bfs(start: (i32, i32), blocked: &HashSet<(i32, i32)>, end: (i32, i32)) -> bool{
    let mut walked_steps = 1;
    let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let length = blocked.len();
    let mut walked = HashSet::new();
    walked.insert(start);
    let mut queue = vec![start];
    while !queue.is_empty() {
        if walked_steps > length as i32 * (length as i32 - 1) / 2 {
            return true;
        }
        let (x, y) = queue.pop().unwrap();
        for (dx, dy) in moves.iter() {
            let n_x = x + dx;
            let n_y = y + dy;
            if (n_x, n_y) == end {
                return true;
            }
            if 0 <= n_x && n_x < 1000000 && 0 <= n_y && n_y < 1000000  {
                if !blocked.contains(&(n_x, n_y)) && !walked.contains(&(n_x, n_y)) {
                    walked_steps += 1;
                    queue.push((n_x, n_y));
                    walked.insert((n_x, n_y));
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::is_escape_possible;

    #[test]
    fn it_works() {
        assert_eq!(is_escape_possible(vec![vec![999, 999]], vec![0, 0], vec![9999, 999]), true);
    }
}
