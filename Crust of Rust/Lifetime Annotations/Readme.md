### **[Crust of Rust: Lifetime Annotations](https://youtu.be/rAl-9HwD858)**

**方法：先看一遍视频，然后自己脱离视频实现视频中的演示内容，再重新看一遍视频。**

**进度：5h34min，未完成**

March 15, 2022 10:00 PM 完成第一遍的视频观看，耗时 1h34min，没有做笔记。

March 16, 2022 2:29 PM 实现基本脱离视频演示完成StrSplit。

March 16, 2022 8:00 PM 完成第二遍的视频记录，基本上第二遍视频所需要的时间会翻倍，整体上这一期视频我花费了大致五个半小时，没有统计具体的时间，后续会进行统计。

**参考：**

1. [https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8](https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8)
2. **[Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html?highlight=lifetime#trait-and-lifetime-bounds)**
3. **[Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision)**
4. **[The Rustonomicon Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html#lifetimes)**

**实践：实现一个StrSplit库。耗时1h。**

**笔记：**

1. [0:03:36](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=216s) Rust Lints: [https://doc.rust-lang.org/rustc/lints/index.html](https://doc.rust-lang.org/rustc/lints/index.html)
    1. 利用Rust的[Attributes](https://doc.rust-lang.org/reference/attributes.html)来扩展lints。`#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]`
    2. [missing_debug_implementations](https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#missing-debug-implementations)
    3. [rust_2018_idiom](https://doc.rust-lang.org/rustc/lints/groups.html)
    4. [missing_docs](https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#missing-docs)
2. [0:05:20](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=320s) 首先定义了类型（Struct）和方法。
    1. 实现[Iterator Trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
3. 然后简单的构造了最基础的测试用例。
    1. 利用Outer Attritubes声明测试函数。`#[test]`
    2. Iterator（迭代器）的比较：长度和元素的一致性
4. [0:09:32](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=572s) How you decide between a library and a binary
    1. Binary：可运行的程序
    2. Library：发布的库，查看测试的结果需要编写测试
5. [0:16:15](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=975s) When to use match vs if let some
    1. `match`：对多个pattern关心时使用。
    2. `if let`：对单个可能性关心时使用。
6. [0:17:10](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=1030s) Doesn't compile! missing lifetime specifier
    1. struct中使用了[Borrow](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)，所以需要增加 [lifetime annotation](https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html)
    2. StrSplit中的remainder的生命周期是应该要长于StrSplit本身的。
    3. StrSplit迭代器的返回内容的生命周期应该也是要长于StrSplit的。
    4. 所以迭代器的返回内容应该是和remainder生命周期相同。
7. [0:21:25](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=1285s) Anonymous lifetime `'_`
    1. **[Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision)**
8. [0:23:10](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=1390s) Order lifetimes based on how long they are。（生命周期的长度）
    1. `‘a: ‘b`: `'a` *outlives* `'b`
9. [0:25:18](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=1518s) 多个生命周期标注中，`'_` 的差别
    1. `fn foo<’x, ‘y>(x: &’x str, y: &’y str) → &’x str {}` ⇒ `fn foo(x: &str, y: &’_ str) → &’_ str {}`
    2. `'_` 在函数参数中类似于任意的生命周期长度
    3. `'_` 在返回类型中指需要编译器去猜想具体是哪个生命周期的长度，在上面这个定义中，实际上就是x的生命周期长度。
10. [0:26:52](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=1612s) Compile error: lifetime of reference outlives lifetime of borrowed content
    1. 在视频中，这个时候是在new的时候，使用了匿名生命周期`'_`，而在struct定义中使用了显式的生命周期`‘a`，参数haystack是匿名生命周期`'_`，这个时候没有办法确定这两个生命周期是否有关联，所以也需要在 `impl` 中使用显式的生命周期`‘a`。
11. 尽量的使用匿名生命周期，在**能使用的时候**选择使用匿名生命周期。
12. `impl <'a>` ：impl的代码块的生命周期就是 `‘a`
13. [0:34:02](https://youtu.be/rAl-9HwD858?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&t=2042) Rust的类型系统：不能确定在这里说的是什么具体的内容，大致上是和类型系统相关的，可以留待后续学习。
14. [0:34:45](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=2085s) Static lifetime
    1. [https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)
    2. 直到程序结束
    3. 可以将较长生命周期赋值给较短生命周期。所以 `let x: &'a str = ""` 。`""` 的生命周期是 `‘static`
15. [0:48:07](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=2887s) What is the ref keyword and why not &
    1. ****[Keyword ref](https://doc.rust-lang.org/std/keyword.ref.html)****
        - `&` denotes that your pattern expects a reference to an object. Hence `&` is a part of said pattern: `&Foo` matches different objects than `Foo` does.
        - `ref` indicates that you want a reference to an unpacked value. It is not matched against: `Foo(ref foo)` matches the same objects as `Foo(foo)`.
    2. `if let Some(remainder) = self.remainder {` ：self.remainder 被移出（move），默认可以拥有self.remainder。
    3. `if let Some(&mut remainder) = self.remainder {` ：self.remainder 被移出（move），默认可以拥有self.remainder。同时`&mut` 是pattern的一部分。
    4. `if let Some(ref mut remainder) = self.remainder {` ：对self.remainder 的引用，而非拥有
    5. `if let Some(remainder) = &mut self.remainder {` ：对self.remainder 的引用，而非拥有。这个方法存在更多的魔法，和d中的结果一致。
16. [0:51:36](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3096s) What's the * on the left of remainder
    1. **[Following the Pointer to the Value with the Dereference Operator](https://doc.rust-lang.org/book/ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator)**
17. [0:52:46](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3166s) What is take() doing
    1. 将Option拿走，留下None。
    2. [https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.take](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.take)
18. 尝试操作符 `?` 也可以在Option中使用
    1. 不推荐
19. [0:54:48](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3288s) Mutable references are one level deep
    1. 这个地方不是很理解。大致意思是mut ref只有一层，self是mut的，也就是说我可以修改self.remainder 的指向，但是我无法修改 self.remainder 的内容。 
20. [0:55:39](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3339s) Solving a hang with as_mut()
    1. `let remainder = self.remainder?;` 理论上会将Option<T>的T进行移动，但是因为在这个例子中T是&str，&str是Copy的，所以会进行copy而非move。在此之后，就存在两个不同的指针。
    2. `let remainder = self.remainder.as_mut()?;` 利用as_mut()和尝试运算符，取得Option中的可变引用。这个时候修改remainder，就是在修改self.remainder。
        1. `pub fn as_mut(&mut self) -> Option<&mut T>` Converts from `&mut Option<T>`
         to `Option<&mut T>`
21. [0:57:49](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3469s) Multiple lifetimes, implementing until_char
    1. 在这之前的例子里haysatck和delimiter的生命周期是一致的，但是很可能delimiter的生命周期是不同于haystack的，所以需要引入多个生命周期。
    2. 实际上在StrSplit中，迭代器返回内容的生命周期总是和haystack的生命周期相关，而和delimiter的生命周期无关。而在定义中让haysatck和delimter生命周期相同，实际上这是一个不必要的限制。
    3. 在大部分场景中，不需要多个生命周期
22. [1:03:19](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=3799s) Difference between a str and a String
    1. str 类似于 [char]
    2. &str 类似于 &[char]
    3. String 类似于 Vec<char>
    4. String 可以变为 &str，是容易的、cheap，使用 AsRef。`&String[..]`
    5. &str 也可以变为 String，是昂贵的，会涉及memcpy
    6. 使用String就需要allocator，所以相对昂贵，对部分嵌入式设备不友好。
23. Rust pointer类型
    1. fat pointer
    2. narrow pointer
24. [1:15:24](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=4524s) Generic delimiter (Delimiter trait)
    1. 实际上分隔符的类型不一定是&str或者char，所以可以实现一个范型delimiter。或者说StrSplit支持任何包含trait Delimiter的类型的分隔符。
    2. 声明一个trait Delimiter。包含方法：`fn find_next(&self, s: &str) -> Option<(usize, usize)>; }`
    3. 对类型char和&str分别实现`find_next`
    4. 那么所以就不限于char和&str，任何可能出现在String中的，都可以成为分隔符，只需要实现特定的Delimiter trait。
    5. **[Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior)**
25. [1:23:14](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=4994s) char length utf8
    1. 在利用char作为分隔符的时候，增加长度不能简单的用1。1是byte的长度，`char_indices` 是以bytes的长度进行slice的，这个时候增加长度需要利用char的[len_utf8](https://doc.rust-lang.org/std/primitive.char.html#method.len_utf8)。
    2. [char_indices](https://doc.rust-lang.org/stable/std/primitive.str.html#method.char_indices)
26. [1:25:30](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=5130s) Standard library split
    1. 为什么不使用Pattern，而自己实现trait？学习目的。
    2. Split: [https://doc.rust-lang.org/stable/std/str/struct.Split.html](https://doc.rust-lang.org/stable/std/str/struct.Split.html)
    3. Pattern: [https://doc.rust-lang.org/stable/std/str/pattern/trait.Pattern.html](https://doc.rust-lang.org/stable/std/str/pattern/trait.Pattern.html)
        1. 类似于这个演示中的Delimiter trait。
27. [1:27:39](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=5259s) Q&A
    1. Rust额外的特效的确让Rust代码更加难读。
    2. Rust标准库中的Split实现更加复杂和曲折。

**总结：**

1. 如何使用lifetime标记，以及多个lifetime标记的使用和关系。
2. 如何实现迭代器。
3. 如何声明和实现Trait。
4. Ref关键词。
5. Option使用 ? 运算符和 as_mut()。