/**
搜索旋转数组。给定一个排序后的数组，包含n个整数，但这个数组已被旋转过很多次了，次数不详。请编写代码找出数组中的某个元素，假设数组元素原先是按升序排列的。若有多个相同元素，返回索引值最小的一个。

示例1:
 输入: arr = [15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], target = 5
 输出: 8（元素5在该数组中的索引）

示例2:
 输入：arr = [15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], target = 11
 输出：-1 （没有找到）

提示:
arr 长度范围在[1, 1000000]之间

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/search-rotate-array-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

pub fn search_loop_over(arr: Vec<i32>, target: i32) -> i32 {
    for i in 0..arr.len() {
        if arr[i] == target {
            return i as i32;
        }
    }
    -1
}

pub fn search(arr: Vec<i32>, target: i32) -> i32 {
    let len = arr.len();

    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        let mid = (right + left) / 2;

        if arr[left] == target {
            return left as i32;
        } else if arr[mid] == target {
            right = mid;
        } else if arr[mid] == arr[right] {
            right -= 1;
        } else if arr[mid] < arr[right] {
            if arr[mid] < target && target <= arr[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        } else if arr[mid] > arr[right] {
            if arr[mid] > target && target >= arr[left] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
    }
    if arr[left] == target {
        return left as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn t1() {
        assert_eq!(search(vec![15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], 5), 8);
    }

    #[test]
    fn t2() {
        assert_eq!(search(vec![15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], 11), -1);
    }

    #[test]
    fn t3() {
        assert_eq!(search(vec![1,1,1,1,1,2,1,1,1], 2), 5);
    }
}
