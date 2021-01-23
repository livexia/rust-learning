/*
用以太网线缆将 n 台计算机连接成一个网络，计算机的编号从 0 到 n-1。线缆用 connections 表示，其中 connections[i] = [a, b] 连接了计算机 a 和 b。

网络中的任何一台计算机都可以通过网络直接或者间接访问同一个网络中其他任意一台计算机。

给你这个计算机网络的初始布线 connections，你可以拔开任意两台直连计算机之间的线缆，并用它连接一对未直连的计算机。请你计算并返回使所有计算机都连通所需的最少操作次数。如果不可能，则返回 -1 。 

示例 1：
输入：n = 4, connections = [[0,1],[0,2],[1,2]]
输出：1
解释：拔下计算机 1 和 2 之间的线缆，并将它插到计算机 1 和 3 上。

示例 2：
输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
输出：2

示例 3：
输入：n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
输出：-1
解释：线缆数量不足。

示例 4：
输入：n = 5, connections = [[0,1],[0,2],[3,4],[2,3]]
输出：0

提示：

1 <= n <= 10^5
1 <= connections.length <= min(n*(n-1)/2, 10^5)
connections[i].length == 2
0 <= connections[i][0], connections[i][1] < n
connections[i][0] != connections[i][1]
没有重复的连接。
两台计算机不会通过多条线缆连接。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/number-of-operations-to-make-network-connected
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

fn main() {
    println!("Hello, world!");
    // let c = vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2]];
    // let n = 6;
    // println!("{}", make_connected(n, c));

    
    // let c = vec![vec![0,1],vec![0,2],vec![0,3],vec![1,2], vec![1,3]];
    // let n = 6;
    // println!("{}\n", make_connected(n, c));
    
    // let c = vec![vec![0,1],vec![0,2],vec![1,2]];
    // let n = 4;
    // println!("{}\n", make_connected(n, c));

    // let c = vec![vec![0,1],vec![0,2],vec![3,4], vec![2,3]];
    // let n = 5;
    // println!("{}\n", make_connected(n, c));

    // let c = vec![vec![0,1],vec![1,2],vec![2,3], vec![4,5], vec![6,5], vec![3,0]];
    // let n = 7;
    // println!("{}\n", make_connected(n, c));

    // let c = vec![vec![3,5],vec![8,12],vec![10,13],vec![2,4],vec![1,7],vec![6,11],vec![0,2],vec![1,5],vec![5,9],vec![1,3],vec![9,11],vec![2,12],vec![6,12],vec![4,10],vec![12,14],vec![3,6],vec![3,8]];
    // let n = 17;
    // println!("{}\n", make_connected(n, c));

    let c = vec![vec![17,51],vec![33,83],vec![53,62],vec![25,34],vec![35,90],vec![29,41],vec![14,53],vec![40,84],vec![41,64],vec![13,68],vec![44,85],vec![57,58],vec![50,74],vec![20,69],vec![15,62],vec![25,88],vec![4,56],vec![37,39],vec![30,62],vec![69,79],vec![33,85],vec![24,83],vec![35,77],vec![2,73],vec![6,28],vec![46,98],vec![11,82],vec![29,72],vec![67,71],vec![12,49],vec![42,56],vec![56,65],vec![40,70],vec![24,64],vec![29,51],vec![20,27],vec![45,88],vec![58,92],vec![60,99],vec![33,46],vec![19,69],vec![33,89],vec![54,82],vec![16,50],vec![35,73],vec![19,45],vec![19,72],vec![1,79],vec![27,80],vec![22,41],vec![52,61],vec![50,85],vec![27,45],vec![4,84],vec![11,96],vec![0,99],vec![29,94],vec![9,19],vec![66,99],vec![20,39],vec![16,85],vec![12,27],vec![16,67],vec![61,80],vec![67,83],vec![16,17],vec![24,27],vec![16,25],vec![41,79],vec![51,95],vec![46,47],vec![27,51],vec![31,44],vec![0,69],vec![61,63],vec![33,95],vec![17,88],vec![70,87],vec![40,42],vec![21,42],vec![67,77],vec![33,65],vec![3,25],vec![39,83],vec![34,40],vec![15,79],vec![30,90],vec![58,95],vec![45,56],vec![37,48],vec![24,91],vec![31,93],vec![83,90],vec![17,86],vec![61,65],vec![15,48],vec![34,56],vec![12,26],vec![39,98],vec![1,48],vec![21,76],vec![72,96],vec![30,69],vec![46,80],vec![6,29],vec![29,81],vec![22,77],vec![85,90],vec![79,83],vec![6,26],vec![33,57],vec![3,65],vec![63,84],vec![77,94],vec![26,90],vec![64,77],vec![0,3],vec![27,97],vec![66,89],vec![18,77],vec![27,43]];
    let n = 100;
    println!("{}\n", make_connected(n, c));
}

pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    if connections.len() < n as usize - 1 {
        return -1
    }
    let mut table = Vec::new();
    for i in 0..n {
        table.push(i as usize);
    }

    let mut extra = 0;
    let mut computers = 0;
    for i in 0..connections.len() {
        if union(connections[i][0] as usize, connections[i][1] as usize, &mut table) {
            extra += 1
        } else {
            computers += 1;
        }
    }
    n - computers - 1
}

fn find(x: usize, table: &mut Vec<usize>) -> usize {
    if x != table[x] {
        table[x] = find(table[x], table);
    }
    table[x]
}

fn union(x: usize, y: usize, table: &mut Vec<usize>) -> bool {
    let root_x = find(x, table);
    let root_y = find(y, table);
    if root_x == root_y {
        return true;
    } else {
        table[root_y] = root_x;
    }
    false
}