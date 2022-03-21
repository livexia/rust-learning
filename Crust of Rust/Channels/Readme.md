# **[Crust of Rust: Channels](https://youtu.be/b4mS5UPHh20)**

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时： 7h50min**

1. 2h26min，看完第一遍视频
2. 1h50min，完成基础实践
3. 3h34min，完成第二遍视频和笔记，完成加强实践，完成总结

## 总结

今天的学习内容非常多，看第一遍视频的时候也花了不少时间熟悉概念，基础实践花了大概两小时，反倒是第二遍的视频因为加快了速度，倒是没有花太多的时间，不过在完成第二遍的笔记之后，还是对学习的内容有了更多的理解，所以决定在继续完成我定下的加强实践，也就是实现 `sync_channel` 。直播主要介绍的内容就是 channel ，演示实现了一个简易版的 channel ，Jon 继续介绍了各种 channel 的实现，和各种不同使用场景的 channel 。不能说是完整深入的学习，但是经过这个学习，对 channel 的实现和作用有了更好的认知， channel 不再是一个黑盒，一旦后续有需要使用，我想我能够通过进一步的学习实现利用 `channel` 来完成任务。在补充参考链接的时候，我看到了 tokio 中的 channel ，发现之前自己都已经学过了，但是现在看却记不起自己学过了，而现在在看，channel 不是一个看起来很神奇的事物了，至少大致理解了内在的原理。

相比于之前的实践，今天我在实践中加上了很多的代码注释，在编写代码的过程中对逐渐新增的代码补充说明，增加代码的学习性。

## **内容**

1. 简单实现 std::sync::mpsc
2. 使用 std::sync::Mutex
    1. LockResult
    2. ****MutexGuard 也是智能指针，看定义可以发现和 Rc 的定义很相似。Q：那为什么还需要使用 Arc ？Q：Arc是用来对 Inner 进行动态引用的，Mutex 是对 Inner 里的 queue 进行访问控制的。****
3. 使用 std::sync::Arc
4. 使用 std::sync::Condvar 
5. Channel flavors 并不是不同的 channel 而是在运行时动态选择实现
    1. Synchronous send 能阻塞, 有限容量 Bounded 
        1. Mutex + Condvar + VecDeque
        2. Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
    2. Asynchronous send 不能阻塞，无限容量 UnBounded
        1. Mutex + Condvar + VecDeque 视频中的实践
        2. Mutex + Condvar + LinkedList 不会 resize
        3. Atomic linked list
        4. Atomic block linked list (crossbeam) 
    3. Rendezvous 容量为0的同步通道， 常用用于线程同步
    4. Oneshot 仅发送一次的通道，任意容量
6. Rust 存在很多的 channel 实现 `std::sync::mpsc` 、 `flume` 和 `crossbeam`。
7. 为 async/wait 实现的 channel 有所不同，可以看看 flume 和 crossbeam 的实现
8. 简单介绍了 channel benchmark

## **参考**

1. mpsc [https://doc.rust-lang.org/std/sync/mpsc](https://doc.rust-lang.org/std/sync/mpsc)
2. Arc [https://doc.rust-lang.org/std/sync/struct.Arc.html](https://doc.rust-lang.org/std/sync/struct.Arc.html)
3. Mutex [https://doc.rust-lang.org/std/sync/struct.Mutex.html](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
4. Condvar [https://doc.rust-lang.org/std/sync/struct.Condvar.html](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
5. [https://gist.github.com/jonhoo/935060885d0d832d463fda3c89e8259d](https://gist.github.com/jonhoo/935060885d0d832d463fda3c89e8259d)
6. parking_lot [https://crates.io/crates/parking_lot](https://crates.io/crates/parking_lot)
7. **[Extra Conditionals with Match Guards](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#extra-conditionals-with-match-guards)**
8. **[Comments](https://doc.rust-lang.org/reference/comments.html#comments)**
9. https://github.com/zesterer/flume
10. https://github.com/crossbeam-rs/crossbeam
11. [https://tokio.rs/tokio/tutorial/channels](https://tokio.rs/tokio/tutorial/channels)
12. [https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap)

## **实践**

### **实现 channel**

创建项目： cargo new —lib panama

基础定义：

`Sender` 是一个 Arc 存储内部的数据

```rust
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
```

 `Receiver` 和 `Sender` 几乎一致，但是为了对 `recv` 进行优化，从而增加了一个 `buffer` 字段。

```rust
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
    // add buffer to receiver,
    // because there is only one receiver, so it is ok to put buffer outside the Mutex
    // when there is data one the buffer, just pop from the buffer, no need to acquire the lock
    // when there is nothing on the buffer, acquire the lock, when there is data in the queue,
    // pop the first one, and swap the buffer and queue.
}
```

内层类型 `Shared` 存储一个新的类型由 `Mutex` 保护的 `Inner` 和用于在 `Sender` 和 `Receiver` 之间沟通的 `availability` 。`availability` 是 `Condvar` 类型，即保证没有数据时 `Receiver` 能够阻塞，又保证当没有 `Sender` 的时候 `recv()` 不会挂起。`availability` 不能在 Mutex 里，不能在持有锁的同时告知其他线程可以继续了，其他的线程并不能获取锁，将会产生死锁。

```rust
pub struct Shared<T> {
    inner: Mutex<Inner<T>>,
    availability: Condvar, // Condvar cna not be in Muttex, so need another wrapper for the queue
}
```

底层类型 `Inner` 是实际存储数据类型 `T` 的类型，包含一个队列和一个计数器，计数器用来记录 `Sender` 的数量，当 `Sender` 数量为 0 时告知 `Receiver` 可以返回 `None` 了。在存在多个共享的事物时，常见实践是定义一个 `Inner` ，存放实际的数据。使用 `VecDeque` 而不是 `Vec`， 实现 FIFO

```rust
pub struct Inner<T> {
    // counter need be inside the mutex
    // so there is another warpper for the queue and counter
    queue: VecDeque<T>,
    counter: usize,
}
```

函数 `channel` 并不复杂：

```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        counter: 1,
    };
    let shared = Arc::new(Shared {
        inner: Mutex::new(inner),
        availability: Condvar::new(),
    });
    (
        Sender {
            shared: Arc::clone(&shared), // 注意使用 Arc::clone 而不是 .clone
        },
        Receiver {
            shared: Arc::clone(&shared),
            buffer: VecDeque::default(), // add buffer for recv optimization
        },
    )
}
```

鉴于当前 `#[derive(Clone)]` 要求例如 `Sender<T>` 中的 `T` 是 `Clone` ，所以需要自己实现 `Clone`。在没有 `Sender` 的情况下，可能会导致 `Receiver` 阻塞，所以需要增加计数器，这个计数器需要在 Mutex 中，在 clone 时计数器加一，在 drop 的时减一。

`Clone` 实现

```rust
impl<T> Clone for Sender<T> {
    // implement Clone istead of using #[derive(Clone)]
    // #[derive(Clone)] require that the T is Clone, but we don't want T to be Clone
    fn clone(&self) -> Self {
        // when clone sender need to incremental the count by one
        // now we need acquire the lock to modify the counter
        let mut inner = self.shared.inner.lock().unwrap();
        inner.counter += 1;
        drop(inner); // drop inner
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}
```

`Drop` 实现

```rust
impl<T> Drop for Sender<T> {
    // now we drop the sender, we need to subtract the counter by one
    fn drop(&mut self) {
        // now we also need acquire the lock to modify the counter
        let mut inner = self.shared.inner.lock().unwrap();
        inner.counter -= 1;
        // decremental counter
        let was_last = inner.counter == 0;
        drop(inner);
        if was_last {
            // when there is no sender, notify receiver should be wake to return None
            self.shared.availability.notify_one();
        }
    }
}
```

`send` 时需要告知阻塞的线程可以继续允许了，通过 `availability` 就能实现。 `send` 实现：

```rust
impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutex
        let mut shared = self.shared.inner.lock().unwrap();
        shared.queue.push_back(value);

        self.shared.availability.notify_one();
        // after send notify the recv there is data on the queue, recvier can be wake up.
    }
}
```

recv 时需要取得锁，检查队列上是否有数据，如果有数据就返回数据，如果没有数据同时 Sender 的数量为 0 ，就要返回 None，而如果此时还有 Sender 那么就会将线程阻塞，等待 Sender 的通知再唤醒继续执行。

```rust
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        // we want when there is no data on the queue, recv is blocked
        // use &self instead of &mut self, because shared use Arc<Mutex<_>> interior mutability give by the Mutexs
        // because of the buffer, we need &mut self, to quick swap the buffer and the queue.
        if let Some(t) = self.buffer.pop_front() {
            // when there is data one the buffer, just pop from the buffer, no need to acquire the lock
            return Some(t);
        }

        // when there is data on tge queue, return the first value
        // if there is no data, drop the lock, then rerun the loop.
        let mut shared = self.shared.inner.lock().unwrap();
        loop {
            match shared.queue.pop_front() {
                Some(t) => {
                    ::std::mem::swap(&mut self.buffer, &mut shared.queue);
                    return Some(t);
                }
                None if shared.counter == 0 => return None,
                // when receiver wake up, and there is no sender, recv return None
                None => {
                    // drop(shared);
                    // when there is no data, locks will be continuously aquired and dropped.
                    // We need a way for the receiver to sleep when there is no data,
                    // and when there is more date on the queue, we need to wake up the receiver.
                    // we use Condvar see https://doc.rust-lang.org/std/sync/struct.Condvar.html

                    shared = self.shared.availability.wait(shared).unwrap();
                    // unwrap to ignore possible thread poison
                    // when there is no sender, this will be hang,
                    // so we need a counter to keep track of the number of senders.
                    // counter should be inside the mutex, or anohter atomic counter outside the mutex
                    // need another wrapper for the sender's counter
                }
            }
        }
    }
}
```

使用 RefCell<VecDeque<T>> 作为 buffer 的类型，保证 recv 不需要 &mut self 而仅需要 &self 

`Receiver`: `buffer: RefCell<VecDeque<T>>,`

recv() 中使用 ::std::mem::swap(&mut *self.buffer.borrow_mut(), &mut shared.queue); 进行内存替换，见 [https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap) [https://doc.rust-lang.org/src/core/cell.rs.html#814](https://doc.rust-lang.org/src/core/cell.rs.html#814)

```rust
// use std::mem::swap to quick swap the inner data with queue
// see: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.swap
// also see: https://doc.rust-lang.org/src/core/cell.rs.html#814
::std::mem::swap(&mut *self.buffer.borrow_mut(), &mut shared.queue);
```

### 加强实践：sync_channel

**注意我在这里的实现是我自己的自觉的，并不具有绝对参考性，推荐学习 std::sync::mpsc 和 flum 和 crossbeam 中的实现。**

对于简单的 sync_channel 只需要在增加一个 Condvar  `send_availability`，同时传入一个 bound。对于 bound 非0 的情况，只需要在 Sender 的时候判断当前队列的长度是否超过 bound，如果超过就对新的 `Condavar` 执行 `wait` 操作，当 `Receiver` 从队列从取出一个值的时候，使用 `send_availability` 对 `Sender` 进行通知（ `notify_all` ），将正在等待 send 的 Sender 进行唤醒，继续推入数据。

对于 `bound` 为 0 的情况，如果就按照上述的实现，会出现挂起，因为 `bound` 为 0 时，假设两个线程调用 `send` 和 `recv` ，那么 `send` 会等待数据被取出，而实际上 `recv` 又没有数据能去取得，所以会导致线程挂起。考虑单线程的情况，只有一个 Receiver 的时候，如果当前在 `send` ，而 `send` 又回阻塞线程，而又是单线程，所以当然会阻塞，但是在多线程中，实际上这应该就是一个 rendezvous channel，参见： [Examples](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html#examples-1) 。

在上面实现的基础上，对 `bound` 为 0 的情况进行额外的处理。当 `bound` 为 0 而且队列中也没有数据，那么允许将当前数据推入队列，同时唤醒 Receiver ，但是这个时候还要阻塞当前的 `Sender` 线程，于是只有当数据被取出时，`Sender` 才恢复执行。

```rust
if shared.bound == 0 && shared.queue.len() == 0 {
      shared.queue.push_back(value);
      self.shared.recv_availability.notify_one();
      let _ = self.shared.send_availability.wait(shared).unwrap();
      return;
  };
```

## **笔记**

### [0:00:00](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=0s) std::sync::mpsc 介绍

Multi-producer, single-consumer FIFO queue communication primitives. `mpsc` 是多生产者单消费者的先进先出队列类型。调用 `channel()` 会得到一个 `Sender` 和 `Receiver` ，可以 `clone()` `Sender` 但是无法 `clone()` `Receiver` 这就是为什么可以有多个生产者一个消费者。

`channel` 承载的 `T` 不一定需要是 `Send` 的，但是如果会将 `channel` 中的数据在多线程中传输，那么就要求 `T` 是 `Send` ，可见 [https://doc.rust-lang.org/stable/std/sync/mpsc/struct.Sender.html#impl-Send](https://doc.rust-lang.org/stable/std/sync/mpsc/struct.Sender.html#impl-Send)。如果 `channel` 不在多线程中使用，那么 `T` 只需要是 `Sized` ，而没有其他的限制。

### [0:08:20](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=500s) 实现自己的 `channel`

**基本定义**

函数 channel 返回一个 Sender 和 Receiver，需要定义两个类型 Sender 和 Receiver 。演示中的实践是使用 `std::sync::Mutex` 、 `std::sync::Arc` 和 `std::sync::Condvar` 来实现基础的 `channel` 。

`Mutex` 是一个锁，`lock` 方法能返回一个 `MutexGuard` ，会保证拥有 `MutexGuard` 的成员拥有唯一读写的能力。调用 `lock` 而无法获得锁会阻塞进程。

`Arc` 是原子引用计数，可以跨线程使用。利用 `Arc` 实现跨线程的对同一个队列的动态引用。

`Condvar` 可以对不同的线程进行告知，告知可用性。当 `Receiver` 在等待的数据的时候，如果 `Sender` 发送了数据，那么就需要存在一个机制能告知 `Receiver` 当前能进行接收。

`channel` 的内部应该是一个先进先出的队列，可以使用 `VecDeque` 。队列应该处于 `Mutex` 内部，同时 `Mutex` 也应该处于 `Arc` 中，实现 Sender 和 Receiver 能够同时拥有内部可变性的队列引用。

**实现 send 和 recv**

`Mutex` 的 `lock` 返回一个 `LockResult` ，可能在一个线程完成了操作但是发生了 panic ，这个时候其他线程取得锁，就会得到 `LockResult` ，告知当前线程，之前的线程结果可能有问题。参见：****[Poisoning](https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning)****

使用 `VecDeque` 而不是 `Vec` 保证能在较小的开销下实现先进先出队列，虽然 `VecDeque` 可能会存在 `resize` 的开销，但是仅仅是演示使用所以选择  `VecDeque` **。**

**recv 的预期和实现**

当 recv 调用时，预期出现的情况有两种，队列上有数据，那么直接返回数据即可，如果队列上没有数据，那么应该等待数据出现再取得数据返回。

仅仅使用 `Mutex` 无法实现 `recv` 的预期情况，所以需要引入 `Condvar` ，在内部类型中加入一个新字段，类型为Condvar。`Condvar` 不能和队列在同一个 `Mutex` 中，因为 `Condavar` 实际上是在告诉另一个线程可以醒来了，那么如果 `Condvar` 在 `Mutex` 中，那么这个时候这个线程一定是拥有 `Mutex` 的锁的，而唤醒另一个线程只会让另一个线程继续进入睡眠，因为锁并没有得到释放，这是一种死锁的情况。所以在一个线程中调用 `Condvar` 的唤醒函数（`notify_one` `notify_all`）前一定要先将锁进行释放。

所以在 `Sender` 当取得锁之后，并且在队列上推入新的数据，这个时候要将锁释放，然后利用 `Condvar` 的 `notify_one` 告诉 `Receiver` 可以开始工作了。

那么如何使得 `Receiver` 在没有收到数据的时候阻塞呢？需要使用 `Condvar` 的 `wait` 方法， `wait` 方法需要传入一个 `MutexGuard` ，`wait` 方法再取得这个 `MutexGuard` 后会将锁释放，然后阻塞当前线程。也就是说只有当前线程拥有这个锁，`Condvar` 才能阻塞当前线程，当 `Condvar` 变量收到通知时，阻塞的线程才会被唤醒，唤醒后当前线程自动的获得锁，这个时候再循环读取队列即。

`Condvar` 并不能保证线程不会因为其他原因醒来，利用 `loop` 就能保证及时在没有数据被唤醒后还能继续阻塞等待通知。

**为 `Sender` 实现 `Clone`**

如果使用 `#[derive(Clone)]`  ，因为这实际上是实现了 `impl<T: Clone> Clone for Sender<T> {}` ，这并不是预期发生的事情，因为这要求了内部的数据类型 `T` 必须实现 `Clone` ，而实际上 T 是在 `Arc` 中的， `Arc` 并不对内部数据类型是否实现 `Clone` 存在限制，所以这里不能使用 `#[derive(Clone)]` 而应该自己实现 `Clone` 。实现并不复杂，但是要注意对内部数据类型为 `Arc` 进行 `clone` 的时候，避免使用 `.clone()` 继续复制，而应该使用 `Rc::Clone(&inner)` 确保复制的是一个 `Arc` 而不是内部数据。参考：[https://stackoverflow.com/a/61950053](https://stackoverflow.com/a/61950053)

[0:40:28](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=2428s) **应对没有 Sender 的情况时，Receiver 会挂起的问题**

按照现有实现，当没有 `Sender` 的时候，`Receiver` 会阻塞的等待数据写入，而实际上现在已经不可能再有数据写入了，所以需要有一个计数器记录现有 `Sender` 的数量。原有 `Mutex` 仅确保队列的线程安全，实际上这个计数器应该也是需要在 `Mutex` 中保证线程安全，因为如果一个 `Sender` 在一个线程中新增，而在另一个线程中销毁另一个 `Sender` 这个时候，可能会出现两个线程同时尝试修改计数器，会导致计数器的不准确。将计数器和队列放在同一个 `Mutex` 中减少了 `Mutex` 的使用数量，所以需要再有一个一个内部的数据结构存放队列和计数器，这个内部数据结构再是整个处于 `Mutex` 中。

在复制（ `clone` ） `Sender` 的时候，取得锁，对计数器加一。在销毁（ `drop` ） `Sender` 的时候，取得锁，对计数器减一。要注意在对计数器操作后，要释放锁。当销毁的 `Sender` 是最后一个时，要通过前面设置的 `Condvar` 字段对 `Receiver` 进行唤醒，如果这个时候 `Receiver` 在等待新的数据，收到这个唤醒时 `Receiver` 知道没有更多的 `Sender` 了，而且也没有数据，所以就应该返回 `None` 。

在没有 Receiver 时，Sender 发送数据的行为是什么？如果希望 Sender 不会失败，那么就允许 Sender 能继续发送数据，如果不希望 Sender 能继续发送数据，那么需要而外的变量来记录 Receiver 的数量，然后 Sender 发送数据会失败，但是要注意这种设计下需要将发送的数据返回，以便用户能继续使用这个数据。参见：[https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send)

在当前实现中，每一个 Sender 之间都是同步的，而实际上我们只在乎 Sender 和 Receiver 之间是否同步。当前的实现并不高效，可以参见其它的实现，参考 `std::sync::mpsc` 、`flume` 和 `crossbeam` 。

使用 `VecDeque` 会存在 `resize` 开销的问题，可能会导致 `send` 和 `recv` 花费更多的时间。

[1:05:55](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=3955s) 对 recv 的优化：

因为只会存在一个 `Receiver` ，所以并不需要每次都取得锁。可以在 `Receiver` 中设置一个 `buffer` 。每次调用 `recv` 时，如果能从 `buffer` 中弹出数据则直接返回，如果 `buffer` 是空的，再取得锁，从队列中弹出顶部数据，然后将 `buffer` 和 队列的内存地址互换（ `std::mem::swap` ）。注意要使用内存互换，而不是重新分配、递归和赋值。这样优化减少了取得锁的开销，但是会增加内存的使用和分配。

### [0:58:37](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=3517s) 介绍同步 channel

`Sender` 和 `Receiver` 同步的 channel，参见 `SyncSender` [https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html](https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html) 。在同步 `channel` 中当 `channel` 的容量满了的时候，`send` 会阻塞当前线程直到 Receiver 重新接收了数据。

### [1:13:23](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=4403s) Channel flavors

主要是视频内容的简单记录，如果要详细了解可以见:

1. std::sync::mpsc [https://doc.rust-lang.org/src/std/sync/mpsc/mod.rs.html#158-234](https://doc.rust-lang.org/src/std/sync/mpsc/mod.rs.html#158-234) 
2. [crossbeam_channel](https://doc.servo.org/crossbeam_channel/index.html)::[flavors](https://doc.servo.org/crossbeam_channel/flavors/index.html)::[array](https://doc.servo.org/crossbeam_channel/flavors/array/index.html)::[Channel](https://doc.servo.org/crossbeam_channel/flavors/array/struct.Channel.html#) 
3. [https://docs.rs/flume/latest/flume/](https://docs.rs/flume/latest/flume/)

[1:22:32](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=4952s) 简单介绍不同的 Channel flavors 

- Synchronous channels: Channel where send() can block. Limited capacity.
    - Mutex + Condvar + VecDeque
    - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
- Asynchronous channels: Channel where send() cannot block. Unbounded.
    - Mutex + Condvar + VecDeque
    - Mutex + Condvar + LinkedList
    - Atomic linked list, linked list of T
    - crossbeam: Atomic block linked list, linked list of atomic VecDeque<T>
- Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
- Oneshot channels: Any capacity. In practice, only one call to send().

### [1:32:24](https://www.youtube.com/watch?v=b4mS5UPHh20&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=5&t=5544s) Future-aware channels

很难实现一个 `channel` 同时支持 async/wait 和线程阻塞，但是是可以做到的， `flume` 和 `crossbeam` 都支持。因为 async/await 环境中的不和谐，主要来自对于 I/O Trait 实现的不统一，channel 并不需要这些不和谐的问题，所以对于不同的 async/await 执行器理论上是可以使用每一种 channel 的。