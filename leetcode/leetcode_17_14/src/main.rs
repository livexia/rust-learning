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

pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
    if arr.len() == 0 as usize { return vec![]; };
    let mut result = Vec::new();
    for i in 0..arr.len() {
        if result.len() < k as usize {
            result.push(arr[i])
        }
        if result.len() == k as usize && i + 1 < arr.len() {
            let m = max(&result);
            if arr[i+1] < m[0] {
                result.remove(m[1] as usize);
                println!("OK, {:?}", result);
            }
        }
    }
    result
}

pub fn max(a: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![a[0], 0];
    for i in 0..a.len() {
        if a[i] > result[0] {
            result[0] = a[i];
            result[1] = i as i32;
        }
    }
    result
}