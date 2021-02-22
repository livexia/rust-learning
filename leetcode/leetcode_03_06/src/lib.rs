/**
动物收容所。有家动物收容所只收容狗与猫，且严格遵守“先进先出”的原则。在收养该收容所的动物时，收养人只能收养所有动物中“最老”（由其进入收容所的时间长短而定）的动物，或者可以挑选猫或狗（同时必须收养此类动物中“最老”的）。换言之，收养人不能自由挑选想收养的对象。请创建适用于这个系统的数据结构，实现各种操作方法，比如enqueue、dequeueAny、dequeueDog和dequeueCat。允许使用Java内置的LinkedList数据结构。

enqueue方法有一个animal参数，animal[0]代表动物编号，animal[1]代表动物种类，其中 0 代表猫，1 代表狗。

dequeue*方法返回一个列表[动物编号, 动物种类]，若没有可以收养的动物，则返回[-1,-1]。

示例1:
 输入：
["AnimalShelf", "enqueue", "enqueue", "dequeueCat", "dequeueDog", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [], [], []]
 输出：
[null,null,null,[0,0],[-1,-1],[1,0]]

示例2:
 输入：
["AnimalShelf", "enqueue", "enqueue", "enqueue", "dequeueDog", "dequeueCat", "dequeueAny"]
[[], [[0, 0]], [[1, 0]], [[2, 1]], [], [], []]
 输出：
[null,null,null,null,[2,1],[0,0],[1,0]]
ß
说明:
收纳所的最大容量为20000

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/animal-shelter-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/

use std::collections::VecDeque;

struct AnimalShelf {
    cat_deque: VecDeque<i32>,
    dog_deque: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AnimalShelf {

    fn new() -> Self {
        AnimalShelf {
            cat_deque: VecDeque::new(),
            dog_deque: VecDeque::new(),
        }
    }
    
    fn enqueue(&mut self, animal: Vec<i32>) {
        if animal[1] == 0 {
            self.cat_deque.push_back(animal[0]);
        } else {
            self.dog_deque.push_back(animal[0]);
        }
    }
    
    fn dequeue_any(&mut self) -> Vec<i32> {
        if self.dog_deque.is_empty() && self.cat_deque.is_empty(){
            return vec![-1, -1];
        } 
        if self.cat_deque.front().or(Some(&999999999)) < self.dog_deque.front().or(Some(&999999999)) {
            return vec![self.cat_deque.pop_front().unwrap(), 0]
        } else {
            return vec![self.dog_deque.pop_front().unwrap(), 1]
        }
    }
    
    fn dequeue_dog(&mut self) -> Vec<i32> {
        if self.dog_deque.is_empty() {
            return vec![-1, -1];
        }
        return vec![self.dog_deque.pop_front().unwrap(), 1]
    }
    
    fn dequeue_cat(&mut self) -> Vec<i32> {
        if self.cat_deque.is_empty() {
            return vec![-1, -1];
        }
        return vec![self.cat_deque.pop_front().unwrap(), 0]
    }
}

/**
 * Your AnimalShelf object will be instantiated and called as such:
 * let obj = AnimalShelf::new();
 * obj.enqueue(animal);
 * let ret_2: Vec<i32> = obj.dequeue_any();
 * let ret_3: Vec<i32> = obj.dequeue_dog();
 * let ret_4: Vec<i32> = obj.dequeue_cat();
 */

#[cfg(test)]
mod tests {
    use crate::AnimalShelf;

    #[test]
    fn it_works() {
        let mut obj = AnimalShelf::new();
        obj.enqueue(vec![0, 0]);
        obj.enqueue(vec![1, 0]);
        let ret_2: Vec<i32> = obj.dequeue_cat();
        let ret_3: Vec<i32> = obj.dequeue_dog();
        let ret_4: Vec<i32> = obj.dequeue_any();
        obj.enqueue(vec![2, 1]);
        let ret_5: Vec<i32> = obj.dequeue_any();
        assert_eq!(ret_2, vec![0, 0]);
        assert_eq!(ret_3, vec![-1, -1]);
        assert_eq!(ret_4, vec![1, 0]);
        assert_eq!(ret_5, vec![2, 1]);
    }
}
