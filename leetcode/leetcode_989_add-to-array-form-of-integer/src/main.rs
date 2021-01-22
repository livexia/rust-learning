use std::vec;

/*
对于非负整数 X 而言，X 的数组形式是每位数字按从左到右的顺序形成的数组。例如，如果 X = 1231，那么其数组形式为 [1,2,3,1]。

给定非负整数 X 的数组形式 A，返回整数 X+K 的数组形式。

示例 1：

输入：A = [1,2,0,0], K = 34
输出：[1,2,3,4]
解释：1200 + 34 = 1234
示例 2：

输入：A = [2,7,4], K = 181
输出：[4,5,5]
解释：274 + 181 = 455
示例 3：

输入：A = [2,1,5], K = 806
输出：[1,0,2,1]
解释：215 + 806 = 1021
示例 4：

输入：A = [9,9,9,9,9,9,9,9,9,9], K = 1
输出：[1,0,0,0,0,0,0,0,0,0,0]
解释：9999999999 + 1 = 10000000000
 

提示：

1 <= A.length <= 10000
0 <= A[i] <= 9
0 <= K <= 10000
如果 A.length > 1，那么 A[0] != 0

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/add-to-array-form-of-integer
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

fn main() {
    println!("Hello, world!");
    // let test1 = vec![1, 2, 3, 4];
    // // test1
    // println!("{:?}", add_to_array_form(test1, 10000000));
    let test2 = vec![1,2,0,0];
    // test1
    println!("{:?}", add_to_array_form(test2, 0));

}

pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result_vec: Vec<i32> = Vec::new();
    let mut k_vec: Vec<i32> = Vec::new();

    let mut _k = k;
    loop {
        k_vec.push(_k % 10);
        _k = _k / 10;
        if _k == 0 {
            break;
        }
    }
    k_vec.reverse();
    let k_max = k_vec.len() - 1;
    let a_max = a.len() - 1;
    let mut count = 0;
    let mut carry = 0;
    while count <= a_max || count <= k_max {
        let mut result = carry;
        if count <= k_max {
            result += k_vec[k_max - count];
        } 
        if count <= a_max {
            result += a[a_max - count];
        }
        carry = result / 10;
        result_vec.push(result % 10);
        count += 1;
    }
    if carry == 1 {
        result_vec.push(carry)
    }
    result_vec.reverse();
    result_vec
}