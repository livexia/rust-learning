use std::usize;

/*
设计一个算法，找出数组中最小的k个数。以任意顺序返回这k个数均可。

示例：

输入： arr = [1,3,5,7,2,4,6,8], k = 4
输出： [1,2,3,4]
提示：

0 <= len(arr) <= 100000
0 <= k <= min(100000, len(arr))

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/smallest-k-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
fn main() {
    println!("Hello, world!");
    println!("{:?}", smallest_k(vec![1, 3, 5, 7, 2, 4, 6, 8], 4))
}

pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
    let len = arr.len();
    if len == 0 as usize || k == 0 { return vec![]; };
    quick_select(&mut arr, 0,  len - 1, k as usize);
    let mut result = Vec::new();
    for i in 0..k {
        result.push(arr[i as usize])
    }
    result
}

fn quick_select(arr: &mut Vec<i32>, start: usize, end: usize, k: usize) {
    println!("{:?}, start: {}, end: {}", arr, start, end);
    if start >= end {
        return
    }
    
    let pivot = arr[end];
    let mut i = start;
    for j in start..end {
        if arr[j] < pivot {
            println!("swap {:?}, start: {}, end: {}", arr, j, i);
            if i != j {
                arr.swap(j, i);
            }
            i += 1
        }
    }
    arr.swap(i, end);
    if i < k {
        quick_select(arr, i + 1, end, k);
    } else {
        quick_select(arr, start, i - 1, k)
    }
}