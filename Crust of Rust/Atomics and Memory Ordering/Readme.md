# [Crust of Rust: Atomics and Memory Ordering](https://youtu.be/rMGWeSjctlY)

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时： 13h**

1. 2h50min，这个知识点过于丰富了，第一遍视频只能让我了解一个大概，很多的内容只能听到，但是无法理解。
2. 4h40min，只是简单的完成了视频中的自旋锁的实践，阅读了 Rust 中关于 Atomic、Ordering 的内容，阅读了 Nomicon 中 Atomics 的部分。本来计划进行的 C++ 、LLVM 和更多博客文章的阅读被我搁置了。因为这部分过于丰富，有一点不知如何下手，感觉通过这短短的时间无法掌握所有的内容，反而产生了拖延（2天），那么只好先搁置了，否则这个系列又将被搁置。
3. 5h30min，完成第二遍视频，对部分材料进行了重新阅读。整个部分花费了 5 天，在这种复杂和困难的点上，我的拖延更加明显，达到这个理解我自己认为已经可以接受了。

## 总结

大致理解 Atomics 是为了解决多线程中的同步问题的一种原子类型。而内存模型和内存排序（ Ordering ）则是为了解决，在语义的正确执行、编译器的优化重排序和硬件执行中的重排序三者要求下能够在多线程中实现程序正确快速的运行。

在实践和后续的简单阅读学习中，我也大致了解了再 Rust 中不同的 Ordering 的不同限制，在 Jon 的直播中和 The Rust Nomicon 中也详细说明了这几种不同 Ordering 错误使用时可能出现的问题，这个部分本来是我学习的目标，也就是如何正确的使用 Atomics 编写高速的程序，但是在 Jon 直播中最后的一个例子却让我有些费解，于是我尝试在阅读和学习中寻找到更多的解释，但是很可惜我还是只能懂个大概。

因为没有能够完成对这个部分内容的深入学习，如果说之前的学习我学到了60%，那么我想关于这个部分的内容我只有学到 30% ，这个系列并不是希望我能完全的理解某一个概念，我现在暂时还没有那样的耐心和环境，我想暂时只能达到这个地步了。这是断断续续在两天内完成实践和部分阅读学习的时候写下的，这两天间因为拖延所以并没有实现高效的学习，自己也不能下定决心在这个地方投入 100 小时，所以只好已这样的进度选择继续进行第二遍视频内容的记录和学习，再次印证自己的学习。

这个部分的内容太复杂了，暂时我还无法完全的理解，目前虽然完成了第二遍的视频，有很多内容得到了印证，很多材料完成了重新的阅读，但是我不认为短期内能够有很好的收获，所以这一部分的内容只能暂时如此，如果后续有需要使用 Atomic ，那么再进行深入的学习可能是更好的选择。

## **内容**

1. 为什么需要 Atomic 类型，什么是 Atomic 类型
2. Rust 的内存模型，官方没有给出确定的内存模型
    1. Rust 依赖于 LLVM 所以 Rust 的内存模型和 C11 的内存模型类似，参考 std::memory_order
3. AtomicUsize
    1. load
    2. store
    3. 额外的参数 Ordering
    4. compare_and_swap
    5. compare_exchange
4. CPU 指令允许安全修改和访问
5. non_exhaustive
6. Atomic 和 Mutex 的不同
7. lock free 但是不一定是 wait free
8. 包含一个 UnsafeCell
9.  实现一个 Mutex 
    1. spinlock 不要使用 spinlock 
10. 为 Mutex 实现 Sync 
11. Ordering
    1. Relaxed 不保证执行顺序
    2. Release
    3. Acquire
    4. AcqRel
    5. SeqCst 这个非常非常复杂
    6. 这其中的测试并不一定，ARM 和 x86 具体的实现就存在差异，导致可能失败的错误在某一个平台保成功。
12. fetch_ 方法
13. modification value （CPU）
14. ThreadSanitizer
15. loom
16. Memory  Barrier

## **参考**

1. std::sync::atomic [https://doc.rust-lang.org/std/sync/atomic/](https://doc.rust-lang.org/std/sync/atomic/)
2. AtomicUsize [https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicUsize.html)
3. The Rustonomicon [https://doc.rust-lang.org/nomicon/atomics.html](https://doc.rust-lang.org/nomicon/atomics.html)
4. Ordering [https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
5. non_exhaustive [https://doc.rust-lang.org/reference/attributes/type_system.html](https://doc.rust-lang.org/reference/attributes/type_system.html)
6. The Rust Reference [Memory model](https://doc.rust-lang.org/reference/memory-model.html#memory-model)
7. Spinlocks Considered Harmful [https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html)
8. MESI protocol [https://en.wikipedia.org/wiki/MESI_protocol](https://en.wikipedia.org/wiki/MESI_protocol)
9. C++ Reference [std::memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order) 
    1. Order of evaluation [https://en.cppreference.com/w/cpp/language/eval_order](https://en.cppreference.com/w/cpp/language/eval_order)
10. [LLVM Atomic Instructions and Concurrency Guide](https://llvm.org/docs/Atomics.html)
11. Jeff Preshing has an incredible blog (and I think some videos) on this. He's a C developer, but as you said that doesn't matter.
    1. [https://preshing.com/20141024/my-multicore-talk-at-cppcon-2014/](https://preshing.com/20141024/my-multicore-talk-at-cppcon-2014/)
    2. [https://preshing.com/20120625/memory-ordering-at-compile-time/](https://preshing.com/20120625/memory-ordering-at-compile-time/)
    3. [https://www.airs.com/blog/archives/79](https://www.airs.com/blog/archives/79)
12. ThreadSanitizer [https://github.com/google/sanitizers/wiki/ThreadSanitizerAlgorithm](https://github.com/google/sanitizers/wiki/ThreadSanitizerAlgorithm)
13. loom https://github.com/tokio-rs/loom
    1. [http://plrg.eecs.uci.edu/publications/toplas16.pdf](http://plrg.eecs.uci.edu/publications/toplas16.pdf)
    2. [https://docs.rs/loom/latest/loom/](https://docs.rs/loom/latest/loom/)
14. compiler_fence [https://doc.rust-lang.org/std/sync/atomic/fn.compiler_fence.html](https://doc.rust-lang.org/std/sync/atomic/fn.compiler_fence.html)
15. fence [https://doc.rust-lang.org/std/sync/atomic/fn.fence.html](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html)
16. volatile [https://doc.rust-lang.org/std/ptr/fn.read_volatile.html](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)
    1. Relationship with volatile [https://en.cppreference.com/w/cpp/atomic/memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order)
17. 大话计算机 第六章

## **实践**

主要就是实现了 Mutex ，但是这个 Mutex 不同于标准库中的 Mutex ，本质上这是一个自旋锁。具体的实现在笔记的部分都进行了详细的说明，为了减少冗余，在此不重复。

## 学习内容

### Rust 中的原子类型

链接1：[https://doc.rust-lang.org/std/sync/atomic/index.html](https://doc.rust-lang.org/std/sync/atomic/index.html)

原子类型提供了一种基础类型用作于线程间的共享内存的通信，并且往往是其他并发类型的基础。当正确的使用原子类型和原子类型的操作，可以实现在线程间的同步。

在 C++ 中每一个原子类型的操作，都需要一个 Ordering 参数，表示这个操作的限制强度（memory barrier），这些 Ordering 和 C++ 20 的 atomic ordering 相同，具体见后文。

虽然 Rust 中的原子类型是安全在线程中共享，但并不提供具体的在不同线程中分享的机制，所以在具体使用中多和 `Arc` 共同使用。

原子类型保证是 [lock-free](https://en.wikipedia.org/wiki/Non-blocking_algorithm) 但是并不一定 wait-free ，在很多的方法的实现中，可能使用了 `compare-and-swap` 循环，那么就是需要等待的。

原子类型的实现和可用是依赖于平台的，并不是所有在这个模块中的所有原子类型都是所有平台上通用的。

链接2：The Rustonomicon [https://doc.rust-lang.org/nomicon/atomics.html](https://doc.rust-lang.org/nomicon/atomics.html)

下列的内容，基本上是我一边阅读内容，一边进行简单的翻译，有很多的名词翻译并不一定是正确的。

C++ 的内存模型，实际上是尝试弥合我们想要的语义、编译器需要的优化和硬件想要的的不一致混乱间的差距。我们想要能够编写出能够完全按照我们所说执行的程序，同时这个程序也要很快，在这个过程中，这三者间的差距就很重要了。

**编译器的重排序**

编译器希望通过一系列的复杂的转变，减少程序中的数据依赖和消灭死代码。特别的，编译器可能积极的修改事件的顺序，或者删除部分事件。

```rust
x = 1;
y = 3;
x = 2;
```

对于这个程序，编译器可能觉得，如果按照下面的代码执行那么就最好了。

```rust
x = 2;
y = 3;
```

这不仅仅使得两个语句（事件）的顺序完全相反了，同时也删除了一个语句。对于一个单线程的程序来说，编译器这样的转变是无感的，当所有语句执行之后，所有的状态都是完全一致的。但是如果我们的程序是多线程的，实际上我们有可能需要依赖于 `x` 赋值与 1 直到 `y` 得到赋值。我们希望编译器能进行这些优化，因为这些优化能提升性能，但是又希望程序不会因为编译器的优化而导致结果的错误。

**硬件的重排序**

即便是编译器能完全根据场景进行完全正确的优化，我们的硬件仍然可能引发问题。这个问题来自于 CPU 的内存层次结构。的确在硬件中的某个位置有一个全局的共享内存空间，但是这个空间对于 CPU 来说，显得十分遥远和缓慢。每个 CPU 可能还是希望能够在本地的缓存中进行工作，而只有在本地缓存中不存在所需要的数据时才去共享内存空间中取得。

如果每一次 CPU 从缓存中读取数据都需要和内存中的数据进行对比，那么这个缓存也就发挥不了作用了。这就导致了硬件无法保证在一个线程中的这些事件的顺序是和另外其他线程中的事件顺序相同。如果我们想要保证它们拥有相同的顺序，我们必须使用特殊的指令，来告知 CPU 不要那么聪明。

考虑以下是编译器给出的一种逻辑：

```rust
initial state: x = 0, y = 1

THREAD 1        THREAD2
y = 3;          if x == 1 {
x = 1;              y *= 2;
                }
```

在理想情况中，这个程序存在两种最终状态：

- `y =  3` ：（线程 2 在线程 1 完成前进行了检查）
- `y =  6` ：（线程 2 在线程 1 完成后进行了检查）

然而这里存在有硬件导致的第三种可能状态：

- `y = 2` ：（线程 2 看见了 `x = 1` ，但是没有看见 `y = 3`，然后利用 `y *= 2` 对 `y = 3` 进行了覆盖写）

对于不同的 CPU 类型，提供了不同的保证，大致上有两类 CPU 硬件，一种是强排序的，另一种是弱排序的。大部分常见的 x86/64 提供了强排序保证，而 ARM 则提供了弱排序保证。这将导致两种结果：

- 在强排序的硬件中要求强有序的执行可能会有更加小或者免费的开销，因为硬件已经提供了强保证，而弱排序的保证可能只在弱序硬件上产生性能优势。
- ？在强排序的硬件中要求弱有序的执行，可能会导致程序的正常运行，但是严格来说程序并不是严格正确的。如果可能，应该在弱序硬件上测试并发算法。

**数据访问**

C++ 的内存模型通过运行我们表达我们程序的因果关系，来弥合这个差异。一般来说，这个因果关系是通过在不同部分程序和运行这些程序的线程中建立一种 *happens before*（没有发现很好的中文翻译，可以翻译成：发生在前 ？）的关系。这允许编译器和硬件在没有建立这种 happens-before 关系时对程序进行更加积极的优化，而当这样的关系建立的生活更加小心的进行优化。沟通这些关系的方法是通过数据访问（ *data accesses* ）和原子访问（ *atomic accesses* ）。

数据访问是编程世界中的基础，数据访问本质上是不同步的，编译器可以自由的积极的对它们进行优化。如果一个程序是单线程的，编译器会自由的对数据访问进行重新排序。硬件也可以自由的将数据访问中改变数据的变化，随意的和不连贯的将这种改变传递给其他线程。更加重要的是，数据访问是数据竞争的方式。数据的访问对于编译器和硬件来说太友好了，它们提供了一种用来编写同步代码的糟糕语义。

**仅仅使用数据访问来编写正确的同步代码，简直是不可能的。**

我们通过原子访问来告知硬件和编译器我们的程序是多线程的。每一个原子访问可以通过一个 ordering 标记来指明它和其他的访问之间建立什么样的关系。实践中，也就是告诉编译器和硬件它们不能做某些事情。对于编译器来说，这主要是围绕在指令的重新排序。对于硬件来说，这主要是围绕着如何将线程写入的内容传播给其他的线程。Rust 暴露的 ordering 有：

- Sequentially Consistent (SeqCst)
- Release
- Acquire
- Relaxed
- Acqrel （ Nomicon 中没有提及）

**按序一致性 Sequentially Consistent (SeqCst)**

按序一致性是这其中最强大的，包含所有其他 ordering 的限制。也就是说一个按序一致性的操作是不允许被重新排序的：一个线程中所有发生在 SeqCst 访问之前和之后的所有访问都保持着原有顺序。一个不存在数据竞争，使用按序一致性原子访问和数据访问的程序，有着非常好的特效，即有一个所有线程都同意的程序指令的单一全局执行。这种执行方式也很好理解，每个线程的单独执行的交错，也就是对于每一个线程来说，指令执行的顺序都是一致的。而这个保证在其他较弱的 ordering 中是不存在的。

这种对开发者友好的按序执行并不是免费的，即便是在强序平台上，也往往涉及到给出内存屏障（ memory fences ）。

在实际中，对于程序的正确性往往是不必要的。但是如果你对于其他的内存一致性（ memory ordering ）并不具有信心，那么按序一致性绝对是正确的选择。运行慢一点的程序要好过运行错误的程序。同时在实现中将原子操作降级也是微不足道的，只需要简单的将 `SeqCst` 改为 `Relaxed` ，当然这并不保证这样的改变是正确的。

**取得-释放 Acquire-Release**

Acquire 和 Release 大部分情况下都是配合使用的。它们的名字暗示了使用情景：它们完全适用于获取和释放锁，并确保关键部分不重叠。

 Acquire 访问，确保了之后的所有访问都保持在之后。然而发生在 Acquire 的之前的操作，可以自由的重新排序到 Acquire 之后。类似的一个 Release 的访问确保了每一个之前的访问都保持在之前，然而发生在 Release 之后的访问可以自由的重新排序到 Release 之前。

当线程 A 在内存中释放（ Release ）了一个位置，随后线程 B 从内存中获取（ Acquire ）同一位置时，因果关系就建立了。 每一个在 A 线程的释放（ Release ）前的写入（包括非原子和松散的原子写入），都将会被 B 的获取（ Acquire ）之后观察到。然而，这不会于任何其他线程简历关系。类似的，如果 A 和 B 访问了内存中的不同位置，那么将不会有因果关系被建立。

因此基本的释放和获取一致性（ Acquire-Release ）的使用很简单：你获取（ Acquire ）一个内存位置以开始关键部分，然后释放（ Release ）该位置以结束关键部分。例如一个简单的自旋锁如下：

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    let lock = Arc::new(AtomicBool::new(false)); // value answers "am I locked?"

    // ... distribute lock to threads somehow ...

    // Try to acquire the lock by setting it to true
    while lock.compare_and_swap(false, true, Ordering::Acquire) { }
    // broke out of the loop, so we successfully acquired the lock!

    // ... scary data accesses ...

    // ok we're done, release the lock
    lock.store(false, Ordering::Release);
}
```

在这个示例代码中，在 `Acquire` 和 `Release` 中的任何代码都将不会被重新排序。

在强序平台中，大部分的访问都有释放和获取语义，这使得释放和获取通常是免费的。在弱序平台中往往不是如此。

**松散 Relaxed**

松散的访问是最不严格的。这些访问可以自由的进行重新排序，并不提供任何的 happens-before 关系。尽管如此松散的操作仍是原子操作。所以它们不会被视为数据访问，对它们进行的任何读、修改和写操作都是原子的。松散的操作适合哪些希望一定能发生的，但是并不关心其他方面的问题。例如，如果不使用计数器来同步任何其他访问，多个线程是可以安全的利用松散的 `fetch_add` 增加一个计数器。

在强平台中使用松散一致性的访问很少有好处，因为这些平台通常提供 Acquire-Release 语义。然而，在弱序的平台上，松散的操作可能更便宜。

**AcqRel** 

链接：[https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.AcqRel](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.AcqRel)

拥有 Acquire 和 Release 全部效果，操作中是 loads 的部分会使用 Acquire ，是 stores 的部分就会使用 Release 。对于 compare_and_swap 来说，很可能这个操作中最后没有发生任何的存储操作，在这种情况下最后可能就只有 Acquire 的效果。然而 AcqRel 无论在任何场景下绝对不会执行 Relaxed 访问。

这个一致性限制，只适用于哪些将读取和写入合并的操作。

### 内存顺序 std::memory_order （ C++ 20，TODO）

Rust 的 Reference 中对于内存模型并没有明确定义，但是现在 Rust 编译器的后端还是 LLVM 的，所以实际上使用的内存模型是和 C/C++ 类似的， Jon 在直播中也是使用了 C++ 的文档进行部分说明。这里大部分的学习资料都是来自于 C++ 、 LLVM 的，包括别人推荐的关于这个话题的较好文章。~~不确定在我看完 Rust Nomicon 之后这里还会进行多少记录。~~**没有能够看完这个部分的内容，留待后续学习吧，这个部分的内容并不轻松，已经出现拖延的情况了，只好暂时搁置，好在还有第二遍视频，看看能不能看完第二遍视频再补充这个部分的内容。**

在视频大概一小时的部分， Jon 提及了部分名词，这些名词从未出现在 Rust 相关的文档中，所以我完整的完成了这个部分内容的阅读，因为和 C++ 直接相关，而且文档中的说明多是正式严格的叙述，我的记录仅仅只是我的理解。

****Modification order 修改序列：对于原子类型，存在一个修改序列，所有对任何特定原子变量的所有修改都是按照一定的顺序进行发生，对于这个序列的确定，在文档中涉及了四个要求，分别对应写-写、读-读、读-写和写-读四种不同基础场景的序列安排。****

1. C++ Reference [std::memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order) 
2. [LLVM Atomic Instructions and Concurrency Guide](https://llvm.org/docs/Atomics.html#llvm-atomic-instructions-and-concurrency-guide)
3. Jeff Preshing has an incredible blog (and I think some videos) on this. He's a C developer, but as you said that doesn't matter.
    1. [https://preshing.com/20141024/my-multicore-talk-at-cppcon-2014/](https://preshing.com/20141024/my-multicore-talk-at-cppcon-2014/)
    2. [https://preshing.com/20120625/memory-ordering-at-compile-time/](https://preshing.com/20120625/memory-ordering-at-compile-time/)

### MESI protocol

链接： [https://en.wikipedia.org/wiki/MESI_protocol](https://en.wikipedia.org/wiki/MESI_protocol) [https://zh.wikipedia.org/wiki/MESI协议](https://zh.wikipedia.org/wiki/MESI%E5%8D%8F%E8%AE%AE)

别名：Illinois protocol

Invalidate-based [cache coherence protocol](https://en.wikipedia.org/wiki/Cache_coherence) , support [write-back caches](https://en.wikipedia.org/wiki/Write-back_cache).

四种状态：cache 中由额外的两位进行标记

- Modified(M) 已修改，是唯一的拷贝，缓存行已经被修改，是脏（dirty）的，缓存的值与主存的值不同，缓存经过写回后状态变为共享。是唯一的拷贝。
- Exclusive(E) 独占，缓存行是在当前缓存中，也是唯一的拷贝，是干净（clean）的，缓存的值和主存的值相同，当读取操作时，状态能转变为共享，或者当写入时，状态会变为已修改。
- Shared(S) 共享，缓存行可能是存储在其他的缓存中，是干净（clean）的，缓存的值和主存的值相同，缓存行可以在任意时刻修改为丢弃状态。
- Invalid(I) 无效，缓存行是无效的。数据是无用的。

维基百科上的内容主要是描述了这四个状态在什么情况中会互相转换，参考资料中也有更多的内容，我只是简单的了解了一下，但是并不是彻底了解。

为什么学习和提及这个协议呢？因为在 Jon 的直播中，关于对于 `Atomic` 类型的 `compare_exchange` 开销较大时提及了这个协议，我的理解是对于使用 MESI 协议的 CPU ，`compare_exchange` 实际上需要将缓存行改编为 Modified 或 Exclusive ，这是开销较大的需要 CPU 间的协同，而对于 `load` 和 `store` 的两个原子操作，实际上 load 只需要缓存行是 Shared 这是开销较小的，大概率不需要 CPU 间的协同，这也是为什么使用 `compare_exchange` 的循环，替代 `load` 的循环和 `store` ，程序执行效率下降的原因。

更多的参考：

1. [https://www.youtube.com/watch?v=-p9tfMMu1PE](https://www.youtube.com/watch?v=-p9tfMMu1PE)
2. [https://www.cs.utexas.edu/~pingali/CS377P/2018sp/lectures/mesi.pdf](https://www.cs.utexas.edu/~pingali/CS377P/2018sp/lectures/mesi.pdf)

### 不应该使用 Spinlock

主要是一篇博客的内容，[Spinlocks Considered Harmful](https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html) 。在这篇文章中详细的说明了，为什么不应该使用 Spinlock 。建议感兴趣的人，直接去阅读原文，这里的记录并不准确，只是自己的阅读记录。

要点：

1. 在 Rust 中，对于希望 `#[no_std]` ，然后无法使用 `Mutex` ，于是 `Mutex` 就被各种 Spinlock 进行替换了。
2. Spinlock 在操作系统看来，和一个正在进行大量计算的线程没有任何不同，是属于用户态的操作。
3. Mutex 是阻塞的系统调用，系统调用开销较大，生产环境中的 Mutex 往往会进行几次的 spin ，然后再系统调用，希望 Mutex 会在较短时间内进行释放。
4. Spinlock 在系统看起来是一个良好的、繁忙的系统线程，所以大概率会被优先调度，那么其他的 Spinlock 线程可能会被搁置。而如果有更高优先级的线程，那么 Spinlock 线程很可能会被进一步搁置，而在 CPU 核数不足的情况下，很可能其他的线程会被不断的调度。
5. 即使是没有操作系统中，可能不需要关系线程调度的问题，但是要考虑 CPU 中断的情况，那么就有可能会出现死锁的情况，这比存在操作系统更糟糕。

更多的链接：

1. https://github.com/matklad/spin-of-death

## **笔记**

### [0:00:00](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=0s) Introduction

主要的内容是针对 Rust 中的原子类型和 Rust 中的内存模型，虽然主要的学习要点都是围绕着 Rust 进行，但是涉及到内存模型的内容实际上和其他语言是相通的。

### [0:02:03](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=123s) What are atomics?

为什么需要原子类型，在多线程程序中使用原始类型的时候，只有部分具体的方式能实现安全的访问这些类型，可能会引发数据竞争，即便能够完全避免数据竞争，编译器和硬件也有可能导致具体执行中，不同线程间的数据同步行为不一致。原子类型在编译器生成最后的机器指令时，是和原始类型的指令不同的，指令中包含了更多的信息，这些信息包含了如何在多个线程间保持一定的读写一致性。

### [0:05:26](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=326s) The Memory Model

Rust 的 Reference 中并没有明确的定义，Rust 中的具体内存模型是什么，这看来很不合理。因为 Rust 的后端编译器是 LLVM ，所以 Rust 的内存模型基本上就是 C++ 的内存模型。

### [0:07:33](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=453s) AtomicUsize

对于原子类型 AtomicUsize 来说，和原始类型 usize 不同的是，AtomicUsize 存在不同的方式，使用上和 usize 存在着很大的差别。原子类型并不是内在分享的，原子类型默认是在栈上分配的，在大部分情况中可以通过使用 Box 或者更加常见的使用 Arc 将原子类型分配在堆上，这将允许将一个原子类型共享到多个线程中。大部分的原子类型的操作和 usize 操作不同之处在于，可以通过不可变共享引用来对原子类型进行修改。

 `load` 和 `store` ，`load` 方法会将 `AtomicUsize` 中的值进行读取，`store` 方法则会将一个新值放入 `AtomicUsize`， `swap` 方法类似于同时进行 `load` 和 `store` 。这些方法都要求一个额外的参数 `order` ，`Ordering` 在 Rust 中是一个枚举类型，其中每一个 `Ordering` 都确保了，这些操作执行时可能的情况。

 `compre_and_swap` 和 `compare_exchange` 和 `compare_exchange_weak` ，这几个操作互相有所类似，大致上都是同时进行 `load` 和有条件的 `store` ，相比于 `load` 和 `store` ，这些操作是一个原子操作，使用 `load` 和 `store` 的时候，可能存在其他线程在这两个操作中进行运行，而 `compre_and_swap` 则是一个原子操作，不可能在其中被其他线程插入。

还有一系列的 `fetch_` 操作，在一个原子操作中 `load` 内在值，然后进行修改，这些操作与 `compare_exchange` 不同，并不存在任何条件预设。

### [0:12:23](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=743s) Questions so far

在部分平台中，原始类型 `usize` ，默认就存在原子类型的保证，在这些平台上可以最后编译器可以进行优化，在这些平台上使用 `AtomicUsize` 可能在大多数情况中都是没有开销的，对于通用标准库来说，这并不是设计者所主要思考的问题。 

Rust 中的 `Ordering` 枚举类型中存在一个关键字 `non_exhaustive` ，这个关键字表示对于这个枚举类型，可能未来还会存在新增。具体就 `Ordering` 而言，可能未来会新增 `Consumer` ，存在于 C++ 中。

不同架构对不同的 Ordering 有着不同的实现，不同架构上对这些 Ordering 的保证有所不同。

`Mutex` 和 `Atomic` 的不同，对于 `Atomic` 来说，不存在获取锁的过程， `Atomic` 更加高效。 `Atomic` 的操作理论上是 `lock-free` 的，但是不一定是 `wait-free` 的，在不同架构上部分操作可能是需要等待的。

在 Rust 中 `Atomic` 底层还是基于 `UnsafeCell` ，所以可以通过不可变引用进行修改。在 Rust 中，考虑最为底层的情况，`UnsafeCell` 是唯一的方式能够通过不可变引用进行修改。

### [0:20:20](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=1220s) Implementing a (bad) Mutex 实现一个错误的 Mutex

利用 AtomicBool 和 UnsafeCell 实现一个 Mutex ，这个 Mutex 并不是一个标准库中的 Mutex ，而是一个自旋锁。通过检查 AtomicBool 的值，判断是否能够进行写入，如果能进行写入，就通过 UnsafeCell 进行写入。

Mutex 定义：

```rust
pub struct Mutex<T> {
    lock: AtomicBool,
    v: UnsafeCell<T>,
}
```

因为实现的 `Mutex` 需要在多个线程中进行使用，所以需要实现 `Sync` ，因为 `T` 是内部的数据 ， `T` 可能会在多个线程中传递，所以也要求 `T` 是 `Send`。

```rust
unsafe impl<T> Sync for Mutex<T> where T: Send {}
```

设定不同的 `bool` 值，表示锁的状态：

```rust
const UNLOCKED: bool = false;
const LOCKED: bool = true;
```

取得锁：暂时使用 `Ordering::Relaxed` 作为原子操作的参数。

```rust
pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.lock.load(Ordering::Relaxed) != UNLOCKED {}
        self.lock.store(LOCKED, Ordering::Relaxed);
        let ret = f(unsafe { &mut *self.v.get() });
        // change ordering to Release to make sure all access before release the lock
        self.lock.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
```

### [0:27:39](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=1659s) Our Mutex works! 看似正常工作的 Mutex

暂时先不考虑 Ordering 的选择问题，这个代码实际上存在很严重的问题。

考虑以下的测试：

```rust
fn test_load_and_store() {
    use std::thread::spawn;
    // use Box to create a sendable data
    let m: &'static _ = Box::leak(Box::new(Mutex::new(0)));

    let handles: Vec<_> = (0..1000)
        .map(|_| {
            spawn(move || {
                for _ in 0..1000 {
                    m.with_lock(|v| *v += 1)
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
    assert_ne!(m.with_lock(|v| *v), 1000 * 1000);
}
```

这个测试中，首先利用 `Box::leak(Box::new(Mutex::new(0)))` 创建一个可以在线程中共享的指针。测试中创建了 1000 个线程，每个线程中对 `Mutex` 中的值进行 1000 次加一。如果代码完全正确，那么最后 `Mutex` 中的值应该是 `1000*1000` ，在 Jon 直播的演示中，这个代码的确运行正确，但是这个代码的逻辑是存在问题的。

考虑当前 `Mutex` 中的锁处于解锁状态，这个时候两个线程同时尝试进行加一的操作，两个线程同时看见当前解锁状态，然后同时进行获取锁，然后同时进行加一操作，这会导致内部的值实际上并没有在两个线程间进行同步，两个线程的写入很可能存在覆盖。所以理论上最后的结果会小于预期的结果。但是在 Jon 的硬件下计算机架构和性能足够强，所以并没有发生这种错误。我自己的环境是 Mac M1，这个硬件是基于 ARM 的，所以在我的测试中，错误是很有可能发生的，实际上得到的值是小于预期的。

硬件和操作系统也有可能在任何时刻进行线程调度，调度就有可能导致产生线程交错，即使是在单线程的情况中，这个代码的逻辑也是有可能出现问题。

Rust 中不存在默认的 Ordering 。

### [0:39:42](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=2382s) compare_exchange

利用 `compare_exchange` 替换 `load` 和 `store` ，因为当前实现的逻辑中，取得锁的操作实际上是分开的两个原子操作 `load` 和 `store` ，因为线程交错的原因，这两个原子操作中可能存在其他的线程的原子操作，导致出现错误。所以可以使用是一个原子操作的 `compare_exchange` 进行修改。

`compare_exchange` 如果当前原子类型中的值和传入的当前值相同，将存入新值。在这个实现中，也就是，如果当前的 `AtomicBool` 的值是 `UNLOCKED` 那么将值修改为 `LOCKED` 。因为 `compare_exchange` 是在循环中，如果值进行了修改，那么就会返回 `Ok` ，循环结束，如果没有进行修改，那么就会返回 `Err` ，循环继续。无论 `compare_exchange` 是否成功，在返回的结果中，总是包含，在 `compare_exchange` 读取到的值，这对于计数器来说是重要的，但是在当前的场景中，并不重要。

具体实现：

```rust
pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .lock
            .compare_exchange(UNLOCKED, LOCKED, Relaxed, Relaxed)
            .is_err()
				{}
        // Safety: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.v.get() });
        // change ordering to Release to make sure all access before release the lock
        self.lock.store(UNLOCKED, Relaxed);
        ret
    }
```

### [0:44:54](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=2694s) 缓解 compare_exchange 中的竞争

`compare_exchange` 是一个较为昂贵的方法/指令，每一个执行这个指令的线程，都尝试获取原子类型的独享内存，这会增加很多的竞争。考虑在多核的情况中，一个 CPU 获取了这个独享内存，检查完发现并不满足条件，然后其他 CPU 就会继续尝试获取独享内存。每个 CPU 间就存在一种协调，这种协调是十分昂贵的。具体可见 MESI 协议，缓存的一致性导致独享状态的协调开销巨大。

解决方法1：在 `compare_exchange` 的循环内再增加一个循环，这个内循环中不断的通过 `load` 方法检查状态，再进行外层循环的判断。因为 `load` 实际上并不需要独享内存，只需要共享内存即可，这样就能减小开销。

```rust
while self
    .lock
    .compare_exchange(UNLOCKED, LOCKED, Relaxed, Relaxed)
    .is_err()
{
    while self.lock.load(Relaxed) == LOCKED {}
}
```

`Mutex` 中会进行阻塞，而 `compare_exchange` 并不会阻塞。`compare_exchange` 常见的用于循环中。

### [0:50:43](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=3043s) 利用 compare_exchange_weak 再进行优化

`compare_exchange` 只有在当前值和传入值不同才允许失败，`compare_exchange_weak` 允许其他情况下失败，在不同的架构中，`compare_exchange` 可能并不一定是一个指令，例如在 ARM 中，实际上这个方法是由两个指令（ `LDREX` `STREX` ）组合完成的，执行这两个指令可能会在中间失败，但是由于 `compare_exchange` 只允许在传入值和当前值不同情况下才失败，所以 `compare_exchange` 对于 ARM 的实现，其实是这个两个指令的再次循环，所以我们的这个实现，如果在 ARM 平台上执行，实际上最后是一个嵌套的循环。这就是为什么会有 `compare_exchange_weak` ，因为允许更加广泛的失败，所以这就不需要进行潜逃的循环。通常的推荐实践是，如果已经在循环中使用了 `compare_exchange` 那么最好替换为 `compare_exchange_weak` 。当只在确定只允许当前值和传入值不同才失败的情况下才使用 `compare_exchange` 。

```rust
while self
    .lock
    .compare_exchange_weak(UNLOCKED, LOCKED, Relaxed, Relaxed)
    .is_err()
{
    while self.lock.load(Relaxed) == LOCKED {}
}
```

### [0:57:02](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=3422s) Ordering::Relaxed 松散

即使利用 `compare_exchange_weak` 替换了 `load` 和 `store` ，这个代码的实现还是不正确的。在现有的代码中，涉及到的所有原子操作中的 `order` 参数，使用的都是 `Relaxed` ，这就是问题所在。当使用 Relaxed 的时候，除了操作仍旧是原子的，但是不存在更多的保证。

使用 Relaxed 可能出现的问题，参考如下代码：

```rust
#[test]
fn too_relaxed() {
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        let r1 = y.load(Relaxed); // A
        x.store(r1, Relaxed); // B
        r1
    });
    let t2 = spawn(move || {
        let r2 = x.load(Relaxed); // C
        y.store(42, Relaxed); // D
        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    // r1 == r2 == 42
    assert_eq!(r1, 42);
    assert_eq!(r2, 42);
}
```

两个 `AtomicUsize` 的变量 `x` 和 `y` ，初始都为 0 。线程 1 中，从变量 `y` 中取值为 `r1` ，然后将 `r1` 放入 `x` ，再返回 `r1` 。线程 `t2` 中，从变量 `x` 中取值为 `r2` ，然后将 `42` 放入 `y` 中，再返回 `r2` 。理论上最后返回的 `r1` 和 `r2` 不应该都为 42 。但是实际上使用 Relaxed 作为 `load` 和 `store` 的 `order` ，r1 和 r2 最后都为 42 的情况是被允许的，也是有可能发生的，虽然在实际测试中很大概率并不会发生，但是这是有可能的。

虽然线程 `t1` 中 A 的确是在线性顺序上是先于 B 的，在线程 `t2` 中 C 也是先于 D 的，但是因为使用了 Relaxed ，那么并没有任何的保证， D 对 y 的修改，不会发生在 A 之前（ modification order ），B 对 `x` 的修改不会发生在 C 之前（ modification order ）。D 中对 `y` 的副作用（ side-efect ）对线程 `t1` 中的 A 的 `load` 来说是可见的，而 B 对 `x` 的副作用对线程 `t2` 中的 C 的 `load` 来说是可见的。也就是：

- `y` 的 modification order 是：0 42
- `x` 的 modification order 是：0 42

因为是 Relaxed 的，所以对于 C 来说，`x` 的 load 操作就有可能看见 42 ，同理对于 A 来说， `y` 的`load` 也有可能看见 42 。实际来说，这可能是因为编译对代码的重排，或者运行时的重排，使得在线程 `t2` 中 D 先于 C 完成。实际上从程序员的角度来说，如果这两个线程的内容很长，甚至这两个线程都不是处于同一个文件中，那么实际上在线程 t2 中，交换 C 和 D 的执行顺序对于线程 t2 来说，实际上并没有任何问题。

`Relaxed` 适用于对计数器的加一，因为这要求能进行原子操作，但是并不在乎是否这个加一是同步的。

### [1:12:13](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=4333s) Ordering::Acquire/Release 取得-释放

在我们的实践中，在 `compare_exchange_weak` 和 `store` 的操作中都使用了 `Relaxed` ，根据前面的测试用例，有可能出现，实际上对 `Mutex` 内部数据修改的语句，很可能会发生在取得锁前，或者放弃锁之后，因为就编译器或硬件来说，修改内部数据只和内部数据有关，和原子类型的操作无关，那么很可能为了性能就进行了优化。所以在这里需要使用 `Acquire` 和 `Release` 。`Acquire` 和 `Release` 的语义和标准库中的 Mutex 取得锁和释放锁的语义类似，实际上它们的功能也类似。

`Release` 是对于 `store` 而言的，如果使用了 `Release` ，那么所有发生在 `store` 前的任何原子或非原子的访问，都会保持在 `store` 之前。这些访问，对于同一原子使用 `load` 和 `Acquire` 都可见的。

`Acquire` 是对于 `load` 而言的，如果使用了 `Acquire` ，那么所有发生在 load 后的任何原子或非原子的访问，都会保持在 `load` 之后。这些访问，对于同一原子使用 `store` 和 `Release` 都可见的。

在我们的 Mutex 中，对应的使用`Acquire` 和 `Release` 就保证了一种 happens-before 的关系，也就保证了在 `compare_exchange_weak` 解锁和 `store` 重新上锁的过程中的所有代码都不会被重排，在这过程中所有访问的结果都会被其他线程所观察到。

```rust
while self
    .lock
    .compare_exchange_weak(UNLOCKED, LOCKED, Acquire, Relaxed)
    .is_err()
{
    // add a layer of loops to prevent each attempt to gain exclusive access to memory
    // this will run much quicker now.
    while self.lock.load(Relaxed) == LOCKED {}
}
// Safety: we hold the lock, therefore we can create a mutable reference.
let ret = f(unsafe { &mut *self.v.get() });
// change ordering to Release to make sure all access before release the lock
self.lock.store(UNLOCKED, Release);
ret
```

`AcqRel` 同时存在 `Acquire` 和 `Release` 的限制，例如对于 `compare_exchange_weak` 来说，其中 `load` 的部分就会 `Acquire` ，而 `store` 来说就是 `Release` ，但是在我们的实践中，这个并不重要。`AcqRel` 常用与 `fetch_` 中，在这些操作中，并不存在关键部分，不同于我们的实践中，解锁和重新上锁间存在需要执行任务的关键部分。

`compare_exchange_weak` 还有最后一个参数，也就是最后一个 `order` ，这个指明当 `compare_exchange_weak` 失败的时候，需要以怎样的要求运行代码，在我们的情况中，我们并不关心这一点，所以可以使用 `Relaxed` ，实际上在我们的实现中，为了更好的性能，最好是使用 `Relaxed` 。

不同的平台可能有着不同的内存模型要求，在 x86 的平台上，因为平台是强序的，也就是平台基本能保证每一个 `load` 和 `store` 都是 `Acquire` 和 `Release` 的，所以这样的代码实际运行中可能不会有问题。但是这种保证，并不是适用于所有的平台，这对测试提出了很高的要求，简单的多次运行测试是效率不足的。

### [1:26:00](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=5160s) fetch_ 方法

对于一个原子类型，可能存在很多的 `fetch_` 方法，`fetch_` 更加上层，通过告知如何计算原子类型的值，会返回原有的值。这些方法并不关心原有的值是什么，所以这些方法是一定会成功的。这些方法也是单一的原子操作。

`fetch_update` 较为不同，接受一个函数闭包，相比于 `fetch_add` ，实际上的实现内部是一个 `compare_exchange` 循环，`fetch_add` 很可能存在架构中就有指令，性能会更好。

> All atomic types in this module are guaranteed to be [lock-free](https://en.wikipedia.org/wiki/Non-blocking_algorithm) if they’re available. This means they don’t internally acquire a global mutex. Atomic types and operations are not guaranteed to be wait-free. This means that operations like `fetch_or` may be implemented with a compare-and-swap loop.
> 

参考：[https://doc.rust-lang.org/std/sync/atomic/index.html](https://doc.rust-lang.org/std/sync/atomic/index.html)

可以使用 `fetch_add` 生成每个线程的唯一标识。相比于使用 `Mutex` 会有更好的性能。

### ? [1:34:07](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=5647s) Ordering::SeqCst 顺序一致性

`SeqCst` 是最为严格的 `Ordering` 。考虑以下代码：

```rust
fn seq_cst() {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = spawn(move || x.store(true, Release));
    let _ty = spawn(move || y.store(true, Release));
    let t1 = spawn(move || {
        while !x.load(Acquire) {}
        if y.load(Acquire) {
            z.fetch_add(1, Relaxed);
        }
    });
    let t2 = spawn(move || {
        while !y.load(Acquire) {}
        if x.load(Acquire) {
            z.fetch_add(1, Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let z = z.load(SeqCst);
}
```

最后 z 的值有多少种可能呢？

`z == 2` 如果线程以 `tx` 、`ty` 、 `t1` 、 `t2` 的顺序执行，最后 `z` 的值就是 2 。

`z == 1` 如果线程以 `tx` 、 `t1` 、`ty` 、 `t2` 的顺序执行，最后 `z` 的值就是 1。

`z == 0` 是最复杂，首先考虑线程调度是否有可能：

- 线程 `tx` 一定会在 `t1` 前完成，即使 `t1` 先开始，也会循环等待 `tx` ，同理 `ty` 一定会在 `t2` 前。
- 可能有的线程运行顺序：
- ty t2 tx t1 ：t1 会使得 z 加一
- ty tx t2 t1 或 tx ty t2 t1 ：最后 t1 还是会使得 z 加一
- tx t1 ty t2 或 tx ty t1 t2 或 ty tx t1 t2 ：无论是哪一种顺序，最后的 t2 还是会使得 z 加一

看起来通过线程调度 `z` 是不可能为 0 的，但是实际上是有可能。考虑 x 和 y 的 modification order 。

- `x` 的 modification order 是：`false` `true`
- `y` 的 modification order 是：`false` `true`

考虑 `t1` ，对于 `t1` 中的 `x.load(Acquire)` ，也在 `tx` 线程中使用了 `x.store(true, Release)` ，那么对于 `t1` 的 `x.load(Acquire)` 来说，它只能看见 `tx` 中 `Release` 后的 `x` 值，也就是 `true` 。但是对于 `t1` 中的 `y.load(Acquire)` 来说 t1 的 y 只有和 ty 建立了 happens-before 的关系，这个时候的 `t1` 中 `y.load(Acquire)` 理论上是可以看见所有 `y` 的可能，也就是可能会是 `false` 或者 `true` 。

同理 `t2` 也是一样的，`t2` 中的 `y.load(Acquire)` 一定会看见 `y` 是 `true` ，但是 `t2` 中的 `x.load(Acquire)` 可能会看见所有的 `x` 。

在这个前提下，那么最后 `z` 就有可能是 0 。

**这个部分的例子非常的令人困惑，非常奇怪，我并不能完全理解。YouTube 视频下的评论中更多关于这部分的提问，可以进一步的阅读和理解。包括 [https://en.cppreference.com/w/cpp/atomic/memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order) 中 Sequentially-consistent ordering 的部分说明，可能更加明确。**

`SeqCst` 是最为严格的限制，同时需要最大的开销，所以如果不理解可能存在的错误，那么最好使用 `SeqCst` 。

最好能够阅读一些并行的数据结构的论文，看看这些论文在什么场景中使用 `SeqCst` 。

### 如何测试 [2:00:40](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=7240s) ThreadSanitizer [2:05:49](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=7549s) loom

对 Atomics 和内存模型相关代码的测试是非常困难的，通过多次运行或增加系统压力，可能可以测试出其中的问题，但是测试仍旧存在更多的问题，平台的不同、环境的一点点不同，编译器的不同都可能导致传统的测试方法并不会发现问题。而且在更多的情况中，即使出错了，程序也不会 panic ，很可能只是结果有一点点错误，问题很可能不一定会立马显现。针对测试其实存在两个问题，如何确保针对所有可能的正确情况都是正确的，如何检测错误的出现。

对于检测错误的出现，编写并行的对原子类型操作的程序，最好能在程序中加上各种各样的断言，确保能够查找到错误。使用谷歌的 https://github.com/google/sanitizers ，具体见其中关于 [ThreadSanitizer](https://github.com/google/sanitizers/wiki/ThreadSanitizerAlgorithm) 的部分。

https://github.com/tokio-rs/loom 是一个 Rust 项目，用来测试所有可能的情况。在每次测试中每次 load 和 store 中会传入所有可能的值。即便如此，在很多场景中，loom 还是无法覆盖所有的场景，即使能覆盖可能涉及的情况的数量大到执行这样的测试也是不实际的。

loom 的文档极其详细，包含了各种的场景。

### [2:22:09](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=8529s) Atomic fences

[atomic](https://doc.rust-lang.org/std/sync/atomic/index.html) 中的三个函数：

- [compiler_fence](https://doc.rust-lang.org/std/sync/atomic/fn.compiler_fence.html) A compiler memory fence. 限制编译器所能进行的重排序。
- [fence](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html) An atomic fence. 原子操作，不需要特定的内存就能对多个线程进行同步。
- [spin_loop_hint](https://doc.rust-lang.org/std/sync/atomic/fn.spin_loop_hint.html) Deprecated，弃用

### [2:27:27](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=8847s) volatile

一个常常和 Atomic 混淆的概念，但是实际上和 Atomic 并没有直接的关系。 volatile 可以直接从内存中获取数据，这个操作是不能被缓存和重新排序的。

- [https://doc.rust-lang.org/std/ptr/fn.read_volatile.html](https://doc.rust-lang.org/std/ptr/fn.read_volatile.html)
- [https://doc.rust-lang.org/std/ptr/fn.write_volatile.html](https://doc.rust-lang.org/std/ptr/fn.write_volatile.html)
- Relationship with volatile [https://en.cppreference.com/w/cpp/atomic/memory_order](https://en.cppreference.com/w/cpp/atomic/memory_order)

### [2:32:18](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=9138s) AtomicPtr

并没有和其他的原子类型有什么特殊，方法上存在不同而已，具体见 [https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html) 

### [2:35:13](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=9313s) Atomics through FFI

在 C 中也有类似的原子类型，并不是内存类型有什么不同，而是编译器对内存的操作存在不同，使得原子类型是原子类型。

### [2:36:44](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=9404s) Consume ordering?

Rust 没有实现 Consume ，十分复杂，而且 C++ 的具体说明也并不是完全稳定，可见 [https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html](https://doc.rust-lang.org/std/sync/atomic/struct.AtomicPtr.html) 

### [2:38:08](https://www.youtube.com/watch?v=rMGWeSjctlY&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=9&t=9488s) Closing thoughts

通常情况下，如果不需要 lock-free ，不要使用 Atomic ，直接使用 Atomic 。如果真的要写 Atomic ，就确保能够进行详尽而完善的测试。