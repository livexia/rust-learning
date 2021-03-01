/**
在本问题中，有根树指满足以下条件的 有向 图。该树只有一个根节点，所有其他节点都是该根节点的后继。该树除了根节点之外的每一个节点都有且只有一个父节点，而根节点没有父节点。

输入一个有向图，该图由一个有着 n 个节点（节点值不重复，从 1 到 n）的树及一条附加的有向边构成。附加的边包含在 1 到 n 中的两个不同顶点间，这条附加的边不属于树中已存在的边。

结果图是一个以边组成的二维数组 edges 。 每个元素是一对 [ui, vi]，用以表示 有向 图中连接顶点 ui 和顶点 vi 的边，其中 ui 是 vi 的一个父节点。

返回一条能删除的边，使得剩下的图是有 n 个节点的有根树。若有多个答案，返回最后出现在给定二维数组的答案。

示例 1：
输入：edges = [[1,2],[1,3],[2,3]]
输出：[2,3]

示例 2：
输入：edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
输出：[4,1]
 
提示：
n == edges.length
3 <= n <= 1000
edges[i].length == 2
1 <= ui, vi <= n

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/redundant-connection-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let length = edges.len();
    let mut parent: Vec<usize> = (0..=length).collect();
    let mut ancestor: Vec<usize> = (0..=length).collect();

    let mut conflict = length + 1;
    let mut cycle = length + 1;

    for i in 0..length {
        let edge = &edges[i];
        let node1 = edge[0] as usize;
        let node2 = edge[1] as usize;
        if parent[node2] != node2 {
            conflict = i;
        } else {
            parent[node2] = node1;
            if find(&mut ancestor, node1) == find(&mut ancestor, node2) {
                cycle = i;
            } else {
                union(&mut ancestor, node1, node2)
            }
        }
    }
    println!("{}, {}", conflict, cycle);
    if conflict > length {
        return edges[cycle].clone();
    } else {
        let conflict_edge = edges[conflict].clone();
        match cycle > length {
            true => return conflict_edge,
            _ => return vec![parent[conflict_edge[1] as usize] as i32, conflict_edge[1]]
        }
    }
}

fn union(ancestor: &mut Vec<usize>, i: usize, j: usize) {
    let root_i = find(ancestor, i);
    let root_j = find(ancestor, j);
    if root_i != root_j {
        ancestor[root_i] = root_j;
    }
}

fn find(ancestor: &mut Vec<usize>, i: usize) -> usize {
    if ancestor[i] != i {
        ancestor[i] = find(ancestor, ancestor[i]);
    }
    ancestor[i]
}

#[cfg(test)]
mod tests {
    use crate::find_redundant_directed_connection;

    #[test]
    fn t1() {
        assert_eq!(find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), vec![2, 3]);
    }
    #[test]
    fn t2() {
        assert_eq!(find_redundant_directed_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]]), vec![4, 1]);
    }
}
