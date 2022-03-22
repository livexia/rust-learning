# **[Crust of Rust: Sorting Algorithms](https://youtu.be/h4RkCyJyXmM)**

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时： 10h44min**

1. 2h50min，完成第一遍视频
2. 4h26min，基本完成实践，但是没能完成所有的加强实践
3. 3h28min，完成视频笔记记录，完成合并排序，完成内容总结

## 总结

算法的确是我的弱点，在算法阅读、实践和分析都花费很多的时间，这应该是到目前为止花费时间最多的学习了，而且也在一个部分中跨越了两天，内容很多，而且我还没有完全实现，在加强实践中仅仅实现了合并排序和堆排序，对于 radix 排序还没有实现，如果要实现可能这个部分的学习会达到12个小时。所以暂时不实现了，后续深入学习算法的时候再补上吧。

虽然 Jon 在开头说并不是主要学习排序算法，而是通过在 Rust 中实践这一系列的算法，展示 Rust 的 idiomatic 的编程方式，在其中也可能会学习一系列的 Rust 知识点。的确有很多 Rust 的细节内容，但是大部分我都比较清楚了，所以学起来还是更多的像是算法学习。即便如此，通过耐心的学习，我还是对这些常见的算法有了很好的认知，我现在知道只要我耐心，有时间能静下心来阅读算法，实现算法，那我也是可以学习算法的，过去对排序算法总是过于着急，以至于反复的学习，却总是没有收获，而对这些算法间的不同和差别却没有认识，即便是花费 12 个小时，我觉得也是很值得的，也是很少的时间，起码我是真正认真的学习了算法。如果回到大一，我肯定会告诉自己，遇到第一个算法，不要着急，静下心来，可能也许算法不再会是我的弱点。

最后关于算法基准测试的部分也让我有了新的收获，通过简单的输出内容，加上简单的几句 R 语言，就可以实现数据的可视化，虽然可视化并不是极其美观，但是无论如何都是要比文字输出来的直观的，经过这个的简单学习，我想后续如果遇到更多的需要比较的场景，还可以实现一下数据可视化。

## **内容**

1. 排序算法
2. Ord trait
    1. Eq + PartialOrd
3. 浮点数只实现了 PartialOrd ，存在一个特殊值 nan
4. sort stable 不重排相同元素
5. 实现各种排序算法
    1. 对实现的各种排序算法中的比较次数
    2. Bubble sort 冒泡排序
        1. slice.swap
    3. Insertion sort 插入排序
        1. slice.binary_search 的 Ok 和 Er
        2. slice.rotate_right [https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)
    4. Selection sort 选择排序
    5. Quicksort 快速排序
        1. [split_at_mut](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)
6. 实现算法的测试和比较
    1. 增加 bench
    2. 使用 rand 生成数据
    3. 使用 R语言可视化数据 
        1. rscript

## **参考**

1. Ord [https://doc.rust-lang.org/std/cmp/trait.Ord.html](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
2. Total order [https://en.wikipedia.org/wiki/Total_order](https://en.wikipedia.org/wiki/Total_order)
3. PartialOrd [https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)
4. slice::sort [https://doc.rust-lang.org/std/primitive.slice.html#method.sort](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)
5. Sorting algorithm [https://en.wikipedia.org/wiki/Sorting_algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm)
6. Bubble sort [https://en.wikipedia.org/wiki/Bubble_sort](https://en.wikipedia.org/wiki/Bubble_sort)
7. slice.rotate_right [https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)
8. Selection sort [https://en.wikipedia.org/wiki/Selection_sort](https://en.wikipedia.org/wiki/Selection_sort)
9. https://github.com/BurntSushi/quickcheck
10. Quicksort [https://en.wikipedia.org/wiki/Quicksort](https://en.wikipedia.org/wiki/Quicksort)
11. https://github.com/bheisler/criterion.rs
12. [cargo-bench(1)](https://doc.rust-lang.org/cargo/commands/cargo-bench.html#cargo-bench1)
13. https://github.com/jonhoo/orst
14. Heap sort [https://en.wikipedia.org/wiki/Heapsort](https://en.wikipedia.org/wiki/Heapsort)
15. Merge sort [https://en.wikipedia.org/wiki/Merge_sort](https://en.wikipedia.org/wiki/Merge_sort)
16. Radix sort [https://en.wikipedia.org/wiki/Radix_sort](https://en.wikipedia.org/wiki/Radix_sort)
17. Installing ggplot2 [https://www.datanovia.com/en/blog/how-to-install-ggplot2-in-r/](https://www.datanovia.com/en/blog/how-to-install-ggplot2-in-r/)
18. rand [https://docs.rs/rand/latest/rand/index.html](https://docs.rs/rand/latest/rand/index.html)

## **实践**

1. Bubble sort 冒泡排序，详见笔记中的冒泡排序部分。
2. Insertion sort 插入排序，详见笔记中的插入排序部分。
3. Selection sort 选择排序，详见笔记中的快速排序部分。
4. Quicksort 快速排序，详见笔记中的快速排序部分。
5. 实现对不同排序算法的简单基准效率测试，详见笔记中的基准测试部分。
    1. 绘图：利用 R 语言实现绘图
        1. 安装 R ，因为是在 Mac 上，所以用了 brew `brew install r`
        2. 安装 ggplot2 `install.packages("ggplot2")`
6. 结果：[Rplots.pdf](./orst/Rplots.pdf)

### **加强实践：**

**Merge sort 合并排序**

合并排序理解上其实更简单，详细一个序列，将序列分成两半，每一部分各自排序，在合并两个部分实现最后整个序列的有序，可见合并排序也是一个分治的算法。理论上合并排序的最好、平均和最差时间复杂度都是 O(nlogn) 。但是应该会有额外的内存开销，因为在合并的时候，需要比较两个部分的头部元素，也就是需要弹出较小部分的元素，如果完全按照算法描述实现，是无法实现原地排序的。我的实现更加取巧，虽然性能较差但是属于原地排序，而且也没有完全改变其中分治的思路，所以在此记录。 

当分裂的两个部分长度是1或0时，这个部分已经有序，所以不需要进一部的排序。

合并排序分成两部分，一部分是分裂，一部分是合并。分裂的实现如下：计算长度，从中间分裂。

```rust
fn merge_sort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        _ => (),
    }
    let mid = slice.len() / 2;
    // let (left, right) = slice.split_at_mut(mid);
    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);
    merge(slice, mid);
}
```

**合并的实现：**

因为想要在原地完成，所以合并的部分实际上传入的是整个序列和分裂的位置。第一部分是从 0 到 mid，第二部分是从 min 到末尾，都包含开头。两个指针 i 和 j ，i 的初始值是 0 也就是第一部分的开头，j 的初始值是 mid 也就是第二部分的开头。判断 slice[i] 和 slice [j] 的大小，如果 slice[i] 大，那那么说明从 0 到 i 的部分已经有序，而如果 slice[j] 大，那么就是说需要将 j 的元素，移动到 i 出，同时 j 加一。我这里利用 `slice[i..j].rotate_right(1);` 对slice 中 i 到 j 的部分进行轮转，刚好令 j 的转到了 i  的位置（注意 j 先进行了加一）。最后再将 i 加一，因为无论slice[i] 和 slice [j] 谁大谁小，有序的部分总是会增加。知道 i 和 j 都指向了序列的末尾，这个时候完成了排序。

```rust
fn merge<T: Ord>(slice: &mut [T], mid: usize) {
    let mut i = 0;
    let mut j = mid;
    while i < j && j < slice.len() {
        if slice[i] > slice[j] {
            j += 1;
            slice[i..j].rotate_right(1);
        }
        i += 1;
    }
}
```

简单分析一下复杂度

1. 如果是非原地排序，对于长度为 n 的序列，合并左右两个部分序列，需要 n/2 次比较，n 次交换位置。
2. 如果我这样的实现，对于长度为 n 的序列，合并左右两个部分序列，需要 n/2 次比较，n^2 次交换位置。

也就是说为了原地实现排序，而牺牲了时间。最后在结果的 pdf 中，也可以看出，这样的实现实际上最后会是 nlogn 次的比较，也就是和较为聪明的选择排序类似的查找次数，而聪明的选择排序中实际上也是使用了 `rotate_right(1)` 来实现插入。所以最后在 pdf 中，无论是查找还是运行时间，这两个算法都是比较接近的。

**Heap sort 堆排序**

堆排序的最好、平均和最差时间复杂度都是 O(nlogn) 。

参考了直播仓库中代码，需要更加仔细的阅读算法的实现，维基上的说明实际上已经极其详细和足够了。我实现的是一种从下到上，在不断的从上到下维护堆属性的算法，维基百科中有着更多的算法。

堆排序实际上很好理解，堆的顶部是最大值，那么实际上只需要不断从堆顶取出数据，放到序列尾部即可。难的部分在于如何构造和维护最大堆。

**将序列变为最大堆**

堆是二叉树结构，而最大堆是所有父亲节点的值是大于孩子节点的值的二叉树。我们可以将序列看成是二叉树的层次遍历，也就是说索引 0 实际上是根节点，1、2是根节点的左右子数。对于任意索引 x ，节点 x 的左节点应该是索引为 x * 2 + 1，而右节点索引应该为 x * 2 + 2。不难看出所有可能的父亲节点，应该都在序列的前一半。

那么究竟该如何构造树呢？考虑一个父亲节点，两个叶子节点的情况，最大堆需要父亲的值是最大，那么只需要简单的将父亲的值和父亲、左右节点值中最大进行互换即可。这个简单的树就是最大堆了。再考虑父亲的父亲也就是爷爷节点，如果父亲的值是爷爷和兄弟中最大的值，那么父亲和爷爷的值就需要互换，的确这个时候爷爷的值是最大的了，但是父亲的值发生了变化，可能父亲的值不再比左右孩子的值大了，所以需要重新进行判断和交换。

可以看出，这是一个自底向上的构造过程，自底向上保证了树的根部是最大值，同时在自底向上的过程中，还需要不断的对孩子节点进行树的规整，也就是重新实现最大堆的属性。

**从序列中部开始自顶向上的构造最大堆**

```rust
for i in (0..(slice.len() / 2)).rev() {
    heapify(slice, i);
}
```

**自顶向下维护最大堆的属性**

```rust
fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    // make sure the root has the biggest value
    // think of the slice as a level oreder traveresal of a binary tree
    // if x is the root, x * 2 + 1 is the left, x * 2 + 2 is the right
    let left = root * 2 + 1;
    let right = root * 2 + 2;
    let length = slice.len();
    let mut max = root;
    // find the biggest node
    if left < length && slice[max] < slice[left] {
        max = left
    }
    if right < length && slice[max] < slice[right] {
        max = right
    }
    if max != root {
        // if max value is not the root
        // swap the root value with the child
        slice.swap(root, max);
        // and make sure the affacted child tree is also heapify
        heapify(slice, max);
    }
}
```

**具体的排序过程**

拥有了一个最大堆，然后呢？这里实现了原地的排序，可以看见在构造最大堆的时候，实际上也是原地的。因为是最大堆，所以每次从序列头部取出的值应该是最大的，也就是实际上应该是放到序列尾部，可以直接将当前最大堆的顶部，直接放置到已排序的部分头部，最初已排序的部分为尾部空白，第一次取元素，是将头部和尾部进行交换，已排序部分增加一个元素，这个时候堆的长度减一，交换后的长度减一的堆，实际上并不是最大堆了，所以需要对这个堆重新进行堆属性的恢复。

```rust
for unsorted in (0..slice.len()).rev() {
    slice.swap(unsorted, 0);

    heapify(&mut slice[..unsorted], 0);
}
```

**Radix 排序**

1. 搁置，可以参考 [https://github.com/jonhoo/orst/blob/master/src/radixsort.rs](https://github.com/jonhoo/orst/blob/master/src/radixsort.rs)

## **笔记**

### [0:00:00](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=0s) 议题介绍

并不是完全学习排序算法，不过由于排序算法非常常见，所以是一个很好的资源用来比较不同的语言。所以会通过展示实现在 Rust 中实现排序算法，来展示 idiomatic 的 Rust。

### [0:02:42](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=162s) Ord 和 标准库中的 std

`Ord` 是表示一个类型能比较（排序）的，如果想要使用 `slice::sort` 就需要实现 `Ord` 。

float point number 也就是浮点数不是 `Ord` 而是 `PartialOrd`，因为浮点数存在一个特殊的值 `NAN` ，而 `NAN` 是无法比较的。

稳定的排序（ stable sort ）是指，对于比较相同的数据，排序后并不会修改原有的顺序，参考 [https://stackoverflow.com/questions/1517793/what-is-stability-in-sorting-algorithms-and-why-is-it-important](https://stackoverflow.com/questions/1517793/what-is-stability-in-sorting-algorithms-and-why-is-it-important)

`slice::sort` 的实现是受 [timsort](https://en.wikipedia.org/wiki/Timsort) 启发的自适应、迭代式合并排序（ merge sort ）。参考：[https://doc.rust-lang.org/std/primitive.slice.html#method.sort](https://doc.rust-lang.org/std/primitive.slice.html#method.sort)。

`slice::sort_by` 可以实现反向排序。

### [0:10:04](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=604s) 排序算法

介绍部分排序算法，详细内容和实现可见维基百科，[https://en.wikipedia.org/wiki/Sorting_algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm) 。在不同的场景中不同的排序算法能实现不同的效率。对于基于比较的排序算法，对于平均时间复杂度，一般最好的就是O(nlogn)。

### [0:12:02](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=722s) Bubble sort 实现冒泡排序

冒泡排序并不复杂，实现简单，但是冒泡排序的效率非常差，最好的时间复杂度是 O(n) ，平均和最差时间复杂度都是 O(n^2)。遍历序列，每一次对两个临近元素进行比较，如果顺序错误就交换两个元素的位置。遍历序列需要不断重复的进行直到一次遍历中不再出现交换。

冒泡排序是稳定和原地的，原地是指没有使用额外的内存空间，通过代码可以看见是通过 `slice::swap` 进行原地交换。

实现代码：

```rust
// see https://en.wikipedia.org/wiki/Bubble_sort
// repeatedly steps through the list,
// compares adjacent elements and swaps them if they are in the wrong order.
let mut swaped = true;
while swaped {
    swaped = false;
    for i in 1..slice.len() {
        if slice[i - 1] > slice[i] {
            slice.swap(i - 1, i);
            swaped = true;
        }
    }
}
```

### [0:27:42](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=1662s) Insertion sort 实现插入排序

在大数列中效率较差，但是实现也很直接，最好的时间复杂度是 O(n) ，平均和最差时间复杂度都是 O(n^2)。基本思路是将序列分成两部分，一部分是已排序的，另一部分是等待排序的，对于等待排序部分的头部元素，将其插入已排序部分，于是已排序部分的长度会逐渐从 1 变为整个数列。

插入排序也是稳定和原地的，通过代码可以看见是通过 `slice::swap` 进行原地交换。

一种方法是对于未排序的头部元素，每次向前比较，如果小于前一个元素，那么就和前一个元素交换位置，然后继续向前比较，确定最终位置。也就是从右向左确定位置，同时交换位置。实现：

```rust
for unsorted in 1..slice.len() {
    let mut unsorted = unsorted;
    while unsorted > 0 && slice[unsorted - 1] > slice[unsorted] {
        slice.swap(unsorted, unsorted - 1);
        unsorted -= 1;
    }
}
```

另一种方法是在已排序的部分查找到合适的位置，然后将这个位置后的内容全部右移动一位。也就是从左向右确定位置，再从左向右交换内容。因为未排序部分头部的元素左侧的所有内容都已经排序了，所以可以使用 `slice::binary_search` 进行查找恰当的位置，然后利用 `slice::rotate_right` 将从这个位置开始的 `slice` 向右转一位。`slice[i..=unsorted].rotate_right(1)` 将 `slice` 从 `i` 到 `unsorted` 的部分进行旋转，向右移动一位，同时尾部会移动到头部。参考：[https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right)

`slice::binary_search` 会返回 Result ，因为查找的内容很可能不在查找对象中，这种时候会返回 `Er(i)`  `i` 表示如果要插入，应该插入到位置 `i` 。如果存在会返回 Ok(i) 要注意，对于存在的情况，如果希望排序是稳定的，那么应该插入到 `i + 1` 。`slice::binary_search` 参见 [https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)

具体实现：

```rust
for unsorted in 1..slice.len() {
    let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
        // see https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
        // If the value is found then Result::Ok is returned, containing the index of the matching element.
        Ok(i) => i,
        // If the value is not found then Result::Err is returned,
        // containing the index where a matching element could be inserted while maintaining sorted order.
        Err(i) => i,
    };
    slice[i..=unsorted].rotate_right(1);
}
```

相比于第一种实现，第二种实现中存在更少的比较，和更少的 `swap` 。可以在类型中增加一个内部状态（字段）来动态选择使用哪一种实现。

### [0:52:18](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=3138s) Selection sort 实现选择排序

选择排序，大部分情况下比插入排序较差，最好、平均和最差时间复杂度都是 O(n^2)。同样将序列视为两部分，和插入排序一样，前面的部分是已排序的，后面的部分是未排序的。每次从未排序的部分中找出最小的元素，将这个元素和未排序的头部进行交换，这个时候已排序的部分长度加一，直到未排序的部分为空。

具体实现中使用 slice 的 min 实现从未排序的部分取得最小的值，具体实现如下：

```rust
for unsorted in 0..slice.len() {
    let min_index = slice[unsorted..]
        .iter()
        .enumerate()
        .min_by_key(|&(_, v)| v)
        .map(|(i, _)| i + unsorted)
        // if slice is empty, then it won't go inside the for loop
        // so if inside the for loop, then the slice won't be empty
        .expect("slice is non-empty");
    slice.swap(unsorted, min_index);
}
```

涉及到的部分点：

1. `.iter()` 将 slice 转为迭代器
2. `.enumerate()` 将迭代器的返回值变为附带索引的 tuple 
3. `.min_by_key(|&(_, v)| v)` 表示根据迭代器返回中的值而不是索引进行取最小值。其中的 & 是 pattern 的一部分，是解引用。
4. `.map(|(i, _)| i + unsorted)` 实现了将取得的 tuple 拆解，取得索引的同时加上偏移量，得到的就是最后的实际位置。
5. `slice.swap(unsorted, min_index);` 最后将取得的最小位置和未排序的头部进行交换。

### [1:07:27](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=4047s) Quicksort 实现快速排序

快速排序效率较好，最好和平均的时间复杂度是 O(nlogn) ，最差时间复杂度是 O(n^2)。快速排序是一种分治算法算法。

**确定 pivot 的位置**

同样是对于一个未排序序列，从中选择一个 pivot ，可以是任意位置，pivot 的选择可能会对性能有轻微的影响，但是在这里我们直接选择第一个序列。快速排序的算法可以简单的看作是，遍历从序列中第二个元素开始的所有元素，如果元素大于 pivot 那么这个元素属于 pivot 的右侧，小于或等于 pivot 则属于 pivto 的左侧，如果简单按照这样的实现，那就需要创建两个额外的序列存储 pivto 的左侧和右侧序列，而且需要不断的移动原有序列，开销太高。

实际上，只需要两个指针，left 和 right ，对于 left 初始值应该是 1 也就是指向第二个元素，right 应该指向最后一个元素。如果 left 指向的元素是小于或等于 pivot 那么 left 指向下一个元素，也就是加一。如果 left 指向的元素大于 pivot 那么 left 指向的元素应该是属于 pivot 的右侧的，那么我们只需要将 left 和 right 指向的值进行交换，然后 right 指向前一个元素，也就是 right 减一。这个时候我们确保了 right 指向元素的所有后续元素都是大于 pivot 的，但是这个时候 left 指向的元素，也就是经过交换的后的元素，这个元素并不一定比 pivot 小，所以 left 指向不变，left 指向的元素仍需要继续判断。

不断的进行这样的判断，直到当左指针 left 大于 right 时，也就是两个指针相交，这个时候 left 指向元素的前面所有元素应该都是小于 pivot 的，而 right 指向元素的后面所有元素应该都是大于 pivot 的。也就是从 left 开始（包含 left ）所有的元素都是大于 pivot 的，而所有的元素直到 right （包含  right ）是小于 pivot 的，因为 pivot 最初选择的是第一个元素，那么这个时候要完成元素位置的互换，也就是需要将 pivot 和 right 的元素位置互换，即 `slice.swap(pivot, left - 1);` 或 `slice.swap(pivot, right);`，这样在序列中就保证了 pivot 指向元素前面的所有元素小于 pivot 指向的元素 ，而右侧元素全都大于 pivot 指向的元素。

**对交换的优化**

当 left 指向的元素大于 pivot 指向元素时，本来要交换 left 和 right 指向元素和对 right 减一，然后重新判断 left 指向元素是否不大于 pivot 指向元素，如果还是大于 pivot 指向元素又要进行交换，可以发现这个时候实际上进行了一次冗余的交换。实际上当 left 指向的元素大于 pivot 指向元素时，我们可以先不交换，检查这个时候 right 指向的值是否大于 pivot ，如果大于那么实际上不需要交换，因为这个时候 right 指向的值一定是属于 pivot 右侧的，所以只需要将 right 减一。

**分治实现所有元素的排序**

经过这样的一轮遍历，可以发现 pivot 实际上已经找到了自己的位置，但是左右两个部分还是无序的，那么只需要分别在对左右两个进行上述操作，假设都选择左右两个部分的第一个元素作为 pivot ，确定 pivot 的位置后，每个部分又回分成两个未排序的部分。当一个部分的长度为0 或 1 的时候，实际上这部分已经是有序的了，所以不再需要进行排序。这样就完成了序列中所有元素的排序。

```rust
fn quicksort<T: Ord>(slice: &mut [T]) {
    // if slice length is 0 or 1, then slice is sorted just return
    match slice.len() {
        0 | 1 => return,
        _ => (),
    }
    // choos pivot index with 0
    let pivot = 0;

    // left is start is 1
    // slice[(pivot + 1)..left] is all the value that smaller than the pivot
    // slice[left..] is all the value that bigger than the pivoit
    let mut left = 1;
    let mut right = slice.len() - 1;
    while left <= right {
        if slice[left] < slice[pivot] {
            // if slice[left] is smaller than the slice[pivot], then left should increase
            left += 1;
        } else if slice[right] > slice[pivot] {
            // if slice[right] is bigger than the slice[pivot]
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            // jsut to decrease the right to check another value
            right -= 1;
        } else {
            // if slice[left] is bigger than the slice[pivot]
            // then swap the slice at left an right,
            // because after swap slice[right] is bigger than the slice[pivot],
            // so we need decrease the right
            slice.swap(left, right);
            right -= 1;
        }
    }
    slice.swap(pivot, left - 1);
    quicksort(&mut slice[..left]);
    quicksort(&mut slice[left..]);
}
```

 **[2:06:20](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=7580s) Quicksort underflow 演示中的实现可能会出现 right 的下界溢出**

演示中 Jon 将 slice 进行了分裂，pivot 和 rest 部分，然后 left 初始值是 0， right 则是 rest 的长度减一。考虑如果序列完全有序，那么 right 会不断的减一，直到 rest 的头部也就是 0 ，而这个时候 left 和 right 相等，所以还会进行判断，因为原始序列完全有序，那么 right 和 left 指向的值还是大于 pivot ，所以right 还会减一，这个时候也就导致 right 下界溢出。这是演示中独有的问题，因为我的实现中，right 是原有序列中的位置，left 是从 1 开始的，也就是说即使 right 和 left 相等，是 1 的时候，即使情况需要对 right 减一，那么也不会导致下界溢出，同时也不影响结果，因为这个时候pivot 和 left - 1 或者 pivot 和 right 交换位置，实际上就是原地不动，因为 pivot 和 left - 1 和 right 都是 0 。

### [1:46:22](https://www.youtube.com/watch?v=h4RkCyJyXmM&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=6&t=6382s) Benchmarking 编写基准测试

基准测试的思路是，对于序列长度0，10，100，1000，和10000的序列，对于不同长度的序列利用 rand 构造随机序列，同时对于每个长度进行10次测试，每一次测试中再对序列进行打乱重排，每次测试中，测试所有的排序算法，对于每一个排序算法，首先将序列完全拷贝，然后将计数器清零，然后对序列进行排序，记录计数器的值，同时记录排序时间，记录之后再判断序列是否有序。输出每个算法，在不同序列长度上所需要的比较次数和执行时间，利用 R 语言将结果进行绘图展示。

**通过对比每一个排序算法中使用比较的次数，来粗略的判断算法的效率**

编写一个用于测试的类型，这个类型有两个字段，一个是 `t` 也就是实际的数据，另一个字段就是 `counter` ，用来记录比较的次数。 `counter` 的类型是 `Rc<Cell<usize>>`。

```rust
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}
```

因为是自己实现的测试类型，这个类型需要进行排序，也就是需要能够进行比较。那么就需要实现 Ord Trait。Ord 则又依赖于 Eq 和 PartialOrd，Eq 又依赖于 PartialEq，所以需要对 SortEvaluator 实现 `PartialEq`  `Eq` `PartialOrd` `Ord` 四个 Trait。

`PartialEq` 的实现：比较字段 `t` 是否和另外一个实例的字段 `t` 相同，同时对 `counter` 加一。

```rust
impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}
```

`Eq` 实际上就是 `PartialEq` ，因为只要字段 t 相同，我们就视为两个类型相同。

```rust
impl<T: Eq> Eq for SortEvaluator<T> {}
```

`PartialOrd` 和 `Ord`  实现：两个实现几乎一致，都需要对 counter 加一，同时对字段 t 调用 partial_cmp 或 cmp。给出 `PartialOrd` 的实现：

```rust
impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}
```

**使用 rand 构造测试数据**

n 是序列的长度，利用 `rand.gen::<usize>()` 生产每一个随机元素值

```rust
for _ in 0..n {
    values.push(SortEvaluator {
        t: rand.gen::<usize>(),
        cmps: Rc::clone(&counter),
    });
}
```

利用 `values.shuffle(&mut rand); // use rand::prelude::*` 在每次测试中将序列重新随机排序。需要使用 `use rand::prelude::*` 或者使用参考中的方法。

参考：

1. [https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen](https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen)
2. [https://docs.rs/rand/latest/rand/seq/trait.SliceRandom.html#tymethod.shuffle](https://docs.rs/rand/latest/rand/seq/trait.SliceRandom.html#tymethod.shuffle)

**对单一排序算法的测试**

```rust
fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1])
    }
    (count, took.as_secs_f64())
}
```

**使用宏来快速增加运行测试的代码**

```rust
#[macro_export]
macro_rules! run_bench {
    ($n: ident, $sorter: expr, $name: expr, $values: expr, $counter: expr) => {
        let took = bench($sorter, $values, $counter);
        println!("{} {} {} {}", $name, $n, took.0, took.1);
    };
}
```

**使用 R 语言来对结果进行数据展示**

将输出数据转存到一个文件中，`cargo r --release > values.dat` ，然后利用 R 语言来进行快速的绘图。

```r
t <- read.table('values.dat', header=TRUE)
library(ggplot2)
# to plot # comparisons
ggplot(t, aes(n, comparisions, colour = algorithm)) + geom_point() + scale_y_log10()
# to plot runtime
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + scale_y_log10(
# add line
ggplot(t, aes(n, time, colour = algorithm)) + geom_point() + geom_line() + scale_y_log10()
```

结果：[Rplots.pdf](./orst/Rplots.pdf)