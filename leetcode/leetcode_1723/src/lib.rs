/**
给你一个整数数组 jobs ，其中 jobs[i] 是完成第 i 项工作要花费的时间。

请你将这些工作分配给 k 位工人。所有工作都应该分配给工人，且每项工作只能分配给一位工人。工人的 工作时间 是完成分配给他们的所有工作花费时间的总和。请你设计一套最佳的工作分配方案，使工人的 最大工作时间 得以 最小化 。

返回分配方案中尽可能 最小 的 最大工作时间 。

示例 1：
输入：jobs = [3,2,3], k = 3
输出：3
解释：给每位工人分配一项工作，最大工作时间是 3 。

示例 2：
输入：jobs = [1,2,4,7,8], k = 2
输出：11
解释：按下述方式分配工作：
1 号工人：1、2、8（工作时间 = 1 + 2 + 8 = 11）
2 号工人：4、7（工作时间 = 4 + 7 = 11）
最大工作时间是 11 。
 
提示：

1 <= k <= jobs.length <= 12
1 <= jobs[i] <= 107

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/find-minimum-time-to-finish-all-jobs
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::HashSet;

pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut work_times = vec![0; k];

    backtrace(&jobs, &mut work_times, 0, k, i32::MAX)
}

fn backtrace(jobs: &Vec<i32>, work_times: &mut Vec<i32>, index: usize, k: usize, mut min_work_time: i32) -> i32 {
    if index >= jobs.len() {
        return min_work_time.min(*work_times.iter().max().unwrap())
    }

    let mut g = HashSet::new();

    for i in 0..k {
        if g.contains(&work_times[i]) {
            continue;
        }
        g.insert(work_times[i]);

        if work_times[i] + jobs[index] > min_work_time {
            continue;
        }

        work_times[i] += jobs[index];
        min_work_time = backtrace(jobs, work_times, index + 1, k, min_work_time);
        work_times[i] -= jobs[index]
    }

    min_work_time
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
