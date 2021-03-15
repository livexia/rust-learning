/**
给定一个整数 n, 返回从 1 到 n 的字典顺序。

例如，

给定 n =13，返回 [1,10,11,12,13,2,3,4,5,6,7,8,9] 。

请尽可能的优化算法的时间复杂度和空间复杂度。 输入的数据 n 小于等于 5,000,000。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/lexicographical-numbers
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut answer  = vec![];
    let temp = 0;
    dfs(n, temp, &mut answer);
    answer
}

fn dfs(n: i32, mut temp: i32, answer: &mut Vec<i32>) {
    if temp > n { return; }
    for i in 0..=9 {
        let pre = temp;
        temp = temp * 10 + i;
        if temp == 0 { continue; }
        if temp > n { return; }
        answer.push(temp);
        dfs(n, temp, answer);
        temp = pre;
    }
}

#[cfg(test)]
mod tests {
    use crate::lexical_order;

    #[test]
    fn it_works() {
        assert_eq!(lexical_order(13), vec![1,10,11,12,13,2,3,4,5,6,7,8,9]);
    }
}
