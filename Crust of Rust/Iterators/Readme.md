# [Crust of Rust: Iterators](https://youtu.be/yozQ9C69pNs)

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时：6h22min  March 18, 2022** 

1. 1h32min，完成第一遍视频
2. 2h40min，完成实践：Flatten、FlatMap
3. 2h10min，完成第二遍视频笔记、代码说明和总结

## 总结

迭代器是语言重要的一部分，主要用于 `for i in iter {}` 这样的循环中，Jon的视频首先介绍了部分 Rust 中的迭代器，以及相关的 Trait。然后通过自己实践标准库中的 Flatten，来达成学习迭代器的目标， Flatten 仅在迭代器的元素也是可迭代时才能使用，相比一般的迭代器更加复杂，需要考虑到对元素进行约束。在这个基础上又实现了 DoubleEndIterator ，使得可以双向遍历迭代器。又利用 Trait 插件的实现，实现类似于官方的方法调用 flatten。加强的实践是进一步实践 FlatMap。很多实践中的复杂其实是由于 Flatten 引入的，并非是迭代器实践的原因，很多细节点在此不过多说明，参见后续的笔记和代码说明。

第一遍的视频只让我对学习和实践的内容有所了解，而之后的实践则对代码有了更直接的认知。即便能实现实践的功能，但是对其中涉及的细节还是一知半解。通过第二遍的视频，和详细的进一步学习，进而对所学的内容有了更完整的理解。对之前的疑问也可以在这个阶段思考，并且通过笔记和进一步的查询资料，也可以对这些疑问进行多角度的解答。即使还存在疑惑，但是也已经搜集了资料，倘若后续再次遇到，那么也可以参考资料进行快速学习。所以当前的学习方法我觉得还是有效果的。

## **内容**

1. 使用迭代器
2. 实现迭代器
3. 实现双向迭代器
4. 更加熟悉 Trait 约束
5. Trait Extension

## **参考**

1. [https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)
2. [https://doc.rust-lang.org/stable/std/iter/struct.Flatten.html](https://doc.rust-lang.org/stable/std/iter/struct.Flatten.html)
3. [https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.flat_map](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.flat_map)
4. [https://doc.rust-lang.org/stable/std/iter/struct.FlatMap.html](https://doc.rust-lang.org/stable/std/iter/struct.FlatMap.html)
5. 实现vscode保存时自动格式化：[https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode](https://stackoverflow.com/questions/67859926/how-to-run-cargo-fmt-on-save-in-vscode)
6. [https://gist.github.com/jonhoo/dd63b720fa4a220ea845a77e2d75831e](https://gist.github.com/jonhoo/dd63b720fa4a220ea845a77e2d75831e)
7. 一个看起来很复杂的代码片段：[https://www.reddit.com/r/rust/comments/ngwlct/fora_a_t_as_intoiteratoritem_trait/](https://www.reddit.com/r/rust/comments/ngwlct/fora_a_t_as_intoiteratoritem_trait/)

## **实践**

**Flatten**

具体实践并不复杂，但是在定义时，涉及到约束的时候，因为是两层迭代器，所以就会产生一些疑惑和不解，虽然最后实现了，但是我只能说有一定的理解，无法说自己已经完全了解，对于更加复杂的形态看我就会又被搞迷惑了，在这个时候需要耐心，一层一层的分析总是能解决。

1. 对于单向的迭代器实现并不复杂，两层的 Flatten 就需要一个 outer 记录外侧迭代器， inner 记录当前的内层迭代器。
2. `Flatten` 成员变量：
    1. `outer_iter: O,`
    2. `inner: Option<<O::Item as IntoIterator>::IntoIter>,`
        1. 将外层迭代器的元素应该是能够成为迭代器，也就是实现了 `IntoIterator` 的类型，所以 `inner` 的类型就应该是 `<O::Item as IntoIterator>::IntoIter` ，也就是外层迭代器的元素变为迭代器。例如对于迭代器 `vec![vec![1], vec![2]].into_iter()` ，实际上第一个 inner 就是 `vec![1].into_iter();`
3. 迭代器成员 `Item` 的实现和 `Trait` 类型说明：
    1. 因为 `Flatten` 的 `Item` 是，内部迭代器的 Item。所以存在有两个约束。 
        1. `O` 是 `Flatten` 的范型。
        2. `O: Iterator` 约束 `O` 需要是迭代器。
        3. `O::Item IntoIterator` 约束 `O` 需要是迭代器。
    2. 那么对于 `Flatten` 的 `Iterator` 实现的 `Item` 就应该是：`type Item = <O::Item as IntoIterator>::Item;` 
    3. 这个时候 `Item` 要求内部迭代器的元素。原有迭代器是两层的，`Flatten` 后的迭代器是一层的，也就是 `Flatten` 的 `Item` 就是 原有迭代器里最里层的 `Item`。
4. 单向的迭代器的 `next` 实现说明如下：
    
    ```rust
    fn next(&mut self) -> Option<Self::Item> {
    				// 利用 loop 或者递归都可以实现内层迭代器为空的情况时的 next 返回
            loop {
    						// 首先利用 let Some 取得内层迭代器的可修改ref
                if let Some(ref mut inner_iter) = self.inner_iter {
    								// 利用 if let Some 判断是否还有值，如果有值就直接返回
                    if let Some(inner_item) = inner_iter.next() {
                        return Some(inner_item);
                    }
    								// 如果内层迭代器已经为空，那么就设内层迭代器为 None
                    self.inner_iter = None;
                }
    						// 如果内层迭代器无值，那么就要从外层迭代器通过 next()?取值
    						// 在通过 into_iter() 将其变为迭代器
    						// 也就是 <O::Item as IntoIterator>::IntoIter 类型
                self.inner_iter = Some(self.outer_iter.next()?.into_iter());
                // self.next() // use recursion or loop
            }
        }
    ```
    
5. 进一步实现 `DoubleEndedIterator`
    1. 如果仅仅按照 `DoubleEndedIterator` 的说明和以类似的方式实现 `Iterator` 。
    2. 因为 `next` 和 `next_back` 是 `DoubleEndedIterator` 同时支持的，所以简单记录一个内层迭代器是不够的。如果只记录一个内层迭代器，以第一次为力，那么如果第一次调用了`next`， 这个时候记录的内层迭代器是第一个元素的迭代器，会取回第一个值，下一次如果调用 `next_back` 会得到第一个元素的迭代器的最后一个值。所以记录一个内层迭代器是不够的，需要同时记录正向和反向的一个内层迭代器（仅在 `next` / `next_back` 调用时分配）。
        1. `front_iter: Option<<O::Item as IntoIterator>::IntoIter>,` 头部内层迭代器，正向遍历，首先从头部内层迭代器取值。
        2. `back_iter: Option<<O::Item as IntoIterator>::IntoIter>,` 尾部内层迭代器，反向遍历时，首先从尾部内层迭代器取值。
    3. 因为对于 Iteratro ，内层迭代器不需要是 `DoubleEndedIterator` ，而对于 `DoubleEndedIterator`，如果内层迭代器不是双向的，那么外层肯定也无法双向。所以需要增加一个约束。
        1. 内层迭代器类型是 `O::Item as IntoIterator>::IntoIter` 
        2. 约束是 `O::Item as IntoIterator>::IntoIter: DoubleEndedIterator`
    4. 对于正向遍历，也就是调用 `next` 的时候，当头部迭代器为空，外层迭代器也为空时，这个时候尾部迭代器可能非空，所以也需要对尾部迭代器调用 `next` 。
    5. 对于反向遍历，也就是调用 `next_back` 的时候，当尾部迭代器为空，外层迭代器也为空时，这个时候头部迭代器可能非空，所以也需要对头部迭代器调用 `next_back` 。

### **加强实践：FlatMap**

创建 `FlatMap` 时，传入一个迭代器和一个函数闭包。这个函数闭包的返回类型是一个可以变为迭代器的类型，进而在对每一个闭包生成的迭代器进行 `flatten` 。对于标准库来说。 `iter.flat_map(f)` 等同于 `iter.map(f).flatten()`

**FlatMap 定义：**

存在三个类型约束：

1. `O: Iterator,` 类似必须是 `Iterator` 才能使用 `FlatMap`
2. `F` 是一个函数，参数是迭代器 `O` 的 `Item`，返回的类型是 `U`
3. `U: IntoIterator,` 这个函数闭包的返回类型是一个可以变为迭代器的类型

```rust
pub struct FlatMap<O, F, U>
where
    O: Iterator,
    F: FnMut(O::Item) -> U,
    U: IntoIterator,
{
    outer_iter: Map<O, F>,
    front_iter: Option<U::IntoIter>, // should be a iterator with Item is U
    back_iter: Option<U::IntoIter>,  // should be a iterator with Item is U
}
```

### Trait 扩展

根据相应 Flatten 或 FlatMap 的约束，对 Trait 扩展进行约束。

```rust
pub trait FlatMapExt: Iterator + Sized {
    fn our_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator;
}

impl<T> FlatMapExt for T
where
    T: Iterator,
{
    fn our_flat_map<F, U>(self, f: F) -> FlatMap<Self, F, U>
    where
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        FlatMap::new(self, f)
    }
}
```

## **笔记**

1. [0:01:45](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=105s) Iterator trait, [https://doc.rust-lang.org/std/iter/trait.Iterator.html](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
2. `for x in vec![1, 2, 3]` 在 desugar 后会变成 `let mut iter = vec![1, 2, 3].into_iter(); while let Some(x) = iter.next() {};` 实际上进一步的 desugar 会变成 loop + break。
3. [0:04:25](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=265s) IntoIterator trait, [https://doc.rust-lang.org/std/iter/trait.IntoIterator.html](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    1. `into_iter` 、`IntoIterator` 是任何能转变为 `Iterator` 的 Trait ，可以理解为是对 `Iterator` 的包装。`Iterator` 自然也是 `IntoIterator`。
    2. 举例：[https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#impl-IntoIterator-5](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#impl-IntoIterator-5)
4. [0:06:24](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=384s) Generic traits vs associated types 
    1. 为什么使用关联类型 Item，而不使用范型。理论上并不是唯一的。通常使用关联类型是在能确定对于这个类型，仅有一个 Trait 的实现。例如对于 Vec 来说，对于 Vec 来说，大部分情况只有一种 Vec 的迭代器实现。
    2. 这个位置的说明仍有一些疑问，留待后续学习。参考：[https://stackoverflow.com/questions/32059370/when-is-it-appropriate-to-use-an-associated-type-versus-a-generic-type](https://stackoverflow.com/questions/32059370/when-is-it-appropriate-to-use-an-associated-type-versus-a-generic-type)
    3. 例子：• `Deref` uses an associated type because without unicity the compiler would go mad during inference
    4. 更多的参考：[https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
5. `iter()` vs `into_iter()` ：`iter()` 是borrow的， `into_iter()` 是 consume 的。
6. [0:13:37](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=817s) Provided Iterator methods 迭代器 Trait 所提供的方式，默认 Iterator 已经实现了部分方法。实现自己的 Iterator 需要实现两个成员，`type Item;` `fn next();` 
7. [0:14:42](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=882s) `Iterator::flatten` 当 Self::Item 实现了 IntoIterator 时才能使用，也就是只有当前迭代器的元素也是可以变成迭代器的类型时，允许进行 `flatten` ，输出是迭代器的所有内部元素展开（压平）后的一个迭代器，对这个迭代器调用 `next` 会对原有迭代器的第一个元素变为迭代器调用 `next`。
8. [0:20:07](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=1207s) Associated items of generics in bounds. [0:23:10](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=1390s) Why must O as Trait be bracketed?
    1. 因为 `Flatten` 的 `Item` 是，内部迭代器的 Item。所以存在有两个约束。 
        1. `O` 是 `Flatten` 的范型。
        2. `O: Iterator` 约束 `O` 需要是迭代器。
        3. `O::Item IntoIterator` 约束 `O` 需要是迭代器。
    2. 那么对于 `Flatten` 的 `Iterator` 实现的 `Item` 就应该是：`type Item = <O::Item as IntoIterator>::Item;` 
    3. 这个时候 `Item` 要求内部迭代器的元素。原有迭代器是两层的，`Flatten` 后的迭代器是一层的，也就是 `Flatten` 的 `Item` 就是 原有迭代器里最里层的 `Item`。
9. `Flatten` 可以是多层的，但是这就需要更加复杂的 Trait 约束。
10. [0:33:29](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=2009s) 利用 `?` 简化代码，对于 `Option` 类型使用 `?` ，如果是 `Some` 那么会取出内部的值，如果是 `None` 那么当前函数会返回`None` ，所以这要求当前代码块的返回需要为 `Option` 。
11. [0:44:15](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=2655s) 实现 `DoubleEndedIterator`
12. [1:04:02](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=3842s) The cost of two cursors 两个内层迭代器的开销，虽然有方法能不用两个指针，但是较为麻烦，而且这两个指针是在栈上的，所以即使是两个指针，也是轻量的。
13. [1:06:28](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=3988s) Iterators are like Futures 多层的迭代器看起来像是 Futures
14. [1:07:14](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4034s) Calling next and next_back concurrently 不能并行的同时调用 `next` 和 `next_back`，无法同时使用 `&mut self`
15. [1:07:55](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4075s) ref in patterns 在匹配中使用 ref ，这个内容应该已经在 **[Crust of Rust: Lifetime Annotations](https://www.notion.so/Crust-of-Rust-Lifetime-Annotations-ff7f9bba428e4fcd88bab711359a18e9)** 中说明了。
16. [1:09:33](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4173s) Why not flatten first, then iterate 为什么不首先把所有的元素收集，在构造迭代？因为那样的实现需要内存分配，而且无法支持无限元素的迭代器。
17. [1:12:51](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4371s) More ref in patterns 更加详细的 ref 在匹配中的使用，[https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html](https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html)
18. [1:14:09](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4449s) Deeper flattening 对于三层的 Flatten ，实现会更加复杂，需要记录更多的内层迭代器。如果存在这样的需求，理论上只需要调用两次两层的 Flatten 即可。
19. [1:17:00](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4620s) FlatMap 加强实践的部分。具体见实践说明。
20. [1:18:19](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4699s) Ergonomics through extension traits 利用 Trait 扩展，实现类似于标准库的调用，也就是通过 `.` 调用，而不是函数调用。
    1. 扩展并非一种标准实践，但是更像是一种习惯性（idiomatic）的做法
    2. 参考：[https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html](https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html)
21. [1:21:19](https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3&t=4879s) Sized in traits 我没能完全理解，大致是，对于 `Flatten` ，对于类型 `O` 会将其存放在 `struct Flatten` 中，所以必须要知道 `O` 的大小，于是需要增加约束 `Sized`
22. `fn foo<T:?Sized>(){}` 现在可以接受UnSized的数据类型了。