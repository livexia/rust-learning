### **[Crust of Rust: Declarative Macros](https://youtu.be/q6paRBbLgNw)**

**方法：先看一遍视频，然后自己脱离视频实现视频中的演示内容，再重新看一遍视频。**

**进度：总耗时 5h20min**

1. 1h36min，看完第一遍视频
2. 54min，完成vecmac
3. 26min，完成hashmapmac
4. 2h24min，完成第二遍视频和笔记内容

**参考：**

1. **[Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#macros)**
2. ****[The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)****
    1. **[Metavariables and Expansion Redux](https://veykril.github.io/tlborm/decl-macros/minutiae/metavar-and-expansion.html#metavariables-and-expansion-redux)**
3. https://github.com/dtolnay/cargo-expand
4. **[Documentation tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html#documentation-tests)**
5. ****[trybuild](https://docs.rs/trybuild/latest/trybuild/#)****
6. ****[Slice length](https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length)****
7. ****[maplit](https://docs.rs/maplit/latest/maplit/index.html)****
8. [https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a](https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a)

**实践：vec macro**

**加强实践：hashmap macro**

参考：****[maplit](https://docs.rs/maplit/latest/maplit/index.html)****

**内容：**

1. 如何使用 macro。
2. 如何创建声明式 macro。
3. 优化宏展开代码。
4. 介绍了一个较好的对于宏参数技术的实现。

**笔记：**

1. [0:00:00](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=0s) Introduction，主要的关注点在于 **Declarative Macros** （声明式宏）。
    1. Macro类型：Declarative Macros、Procedural Macros
    2. [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html) 的视频直播可见：[https://www.youtube.com/watch?v=geovSK3wMB8&list=PLqbS7AVVErFgwC_HByFYblghsDsD5wZD](https://www.youtube.com/watch?v=geovSK3wMB8&list=PLqbS7AVVErFgwC_HByFYblghsDsD5wZDv)
2. [0:01:35](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=95s) 实现自己的 vec macro
    
    vec宏是rust中较为常用的宏，是标准库的一员。常用做初始化 `Vec` ，存在两种使用方法。第一种是 `vec![1, 2, 3]` 创造一个含有元素 `1，2，3` 的 `Vec` 。第二种是 `vec![1; 3]` 创造一个含有3个元素 1 的 `Vec`，使用第二种方式初始化 `Vec` 的时候，要求元素满足 `Clone`。
    
3. `macro_rules!` 自定义宏。要求需要有一个名字，至少一个 `rule` 。
    1. `rule` ： `($pattern) => {$expansion}`
    2. 参考：[macro_rules!](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html)
    3. `macro_rules!` 并不是一个正常的宏，可能类似于 Procedural Macros。
4. [0:04:08](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=248s) The Little Book of Rust Macros，一个很好的学习 Macro 的网站，详细的解释了 Macro，也包含大量有用的 Macro 实践。
5. [0:05:17](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=317s) Macro syntax and hygiene
    1. Macro 大部分情况下对 `pattern` 和 `expansion` 最外侧的括号类型并没有限制
    2. `pattern` 是对 Macro 的参数的匹配。但是匹配并不向函数参数那么严格，理论上只需要句法（syntactically/grammer）上合法就可以。
        1. 合法的 input token tree
    3. `expansion` macro 输出的部分需要是合法的Rust代码，也就是需要能够通过编译。
6. `cargo-expand`：https://github.com/dtolnay/cargo-expand 
    1. 可以实现对宏的实际展开，便于调试和测试宏的效果
    2. 安装：`cargo install cargo-expand`
    3. 使用：`cargo expand`
7. Hygiene： 视频中只是简单带过，具体可见 [2.3.2. Hygiene](https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html) 。针对其中的能够通过的例子，我简单编写了一个测试，见 macrohygiene 。例子1中，`a` 存在着自己的上下文语境，的两个标识符 `a` 实际上存在不同的上下文语境，所以存在错误。例子2中的token是被替换了，所以 `let` 表达式和传入的 `e` 中的 `a` 实际上是存在着统一的上下文语境，所以能够编译通过正常运行。
8. [0:16:42](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=1002s) 实现空 `Vec` 即 `vec![]`，同样也是**先构造测试用例**。`rule` : `() ⇒ { Vec::new() };`
9. 将macro暴露给其他的 scope（域），参考： [2.3.6. Import/Export](https://danielkeep.github.io/tlborm/book/mbe-min-import-export.html) 
    1. `#[macro_use]`
    2. `#[macro_export]`
10. [0:19:26](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=1166s) 实现非空 `Vec` 即 `vec![1]`，需要实现不同的 `rule` ，较为复杂的 `expansion` ，这个时候 `expansion` 中不仅仅包含新建 `Vec` ，也包含将 `pattern` 中匹配到的内容推入 `Vec` ，最后再返回 `expansion` 的 `Vec` 所以需要一个 `block` 实现这一系列功能，也就是需要在 `expansion` 里增加一层花括号。在实现空 `Vec` 中不需要双层花括号是因为只存在一条语句，恰好这一条语句的返回值就是所需要的内容。关键词：Rust 中的 Expression
11. [0:25:50](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=1550s) Macros v2，更加强大和完善的Macro，但是并不会删除 `macro_rules!` 。
    1. [https://rust-lang.github.io/rfcs/1584-macros.html](https://rust-lang.github.io/rfcs/1584-macros.html)
    2. https://github.com/rust-lang/rust/issues/39412
12. [0:26:34](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=1594s) Macro delimiters  无法强制选择使用某一类型的括号，目前三种括号在使用上是一致的。
13. [0:30:15](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=1815s) Repeated macro arguments，参考：[Repetitions](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html#repetitions)
    1. 参数：类似正则的语法，但是极为局限。例如 `$($elem: expr), *` 表示0个或多个`$elem: expr` 由 `,` 分隔的参数。同样的 `+` 表示一个过多个，`?` 表示0个或多个。
    2. 使用： `$(v.push($elem);)*` 将参数中的元素全部推入 `Vec` 中。
14. [0:39:49](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=2389s)  Trailing commas，即实现 `vec![1, 2, ]` ，可见最后会可选的尾随一个 `,` 。
15. 声明式的宏对错误控制并不完美。
16. [0:44:10](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=2650s) macro的用途：演示了当为所有的数字类型实现 Trait MaxValue 的时候，可以利用 macro 来避免重复的编写函数。
17. [0:47:29](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=2849s)  实现非空 `Vec` 即 `vec![1; 10]` ，构造一个含有10个元素1的 `Vec` 。注意 macro 就是替换，那么在实现宏的时候，要注意传入宏参数是否能够被多次求值
18. [0:51:02](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3062s) Macro rules readability 声明式宏的可读性并不好，但是较为轻便，而过程式宏具有更好的可读性，但是编译更加昂贵。
19. [0:52:00](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3120s) Invalid macro inputs 宏产生的错误，最后在编译器报错时，会指向使用宏的位置。
20. [0:54:52](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3292s) Test that something doesn't compile 利用 doc test 进行编译不通过测试，并不可靠，因为很可能不是由于预期的原因而导致编译错误。参考：[Documentation tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html#documentation-tests)
21. [trybuild](https://docs.rs/trybuild/latest/trybuild/#)：另一个编译错误测试的 crate。
22. [0:56:50](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3410s) Tidying up the patterns、[1:08:28](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=4108s) The standard library pattern
    
    阅读了 `std::vec` ，其中的 `pattern` 和视频到现在为止实现的并不相同，为了防止允许以 `vec![,]` 初始化，最后还是选择了官方的两个 `pattern`。要注意在 macro 中 rule 的顺序是重要的。 March 17, 2022 今天再去看官方的 rule，可以发现又有了新的变化，所以这个实现并非唯一。具体见： [vec](https://doc.rust-lang.org/std/macro.vec.html) 
    
23. [0:59:05](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3545s) Reallocations for repetition constructor 对于 `vec![elem; count]` 进行优化
    
    每次 `push` 的时候会检查容量是否满足，如果不满足会重新分配 `Vec`，这是非常昂贵的。要注意不要反复运算宏的参数，会增加开支。
    
    实现：
    
    1. 根据传入的容量创建 Vec：`Vec::with_capacity(count)`
    2. 填入元素：
        1. 方法一：`vs.extend(::std::iter::repeat($elem).take(count));`
        2. 方法二：`vs.resize($count, $elem);`
24. [1:04:08](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=3848s) Macro argument trait bounds 宏没有 Trait 约束，但是如果传入参数不满足 `expansion` 中的使用约束，则会产生编译错误。
25. [1:06:40](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=4000s) "use" hygiene in macros 在宏中使用 `use` 的时候，推荐使用绝对路径引用，例如 `::std::iter::repeat` 开头的 `::` 表示 root 层级。`$crate` 表示宏所在的 crate。
26. [1:10:20](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=4220s) The need for counting 在优化了 `vec![elem; count]` 的 pattern 后，也需要对`vec![e1, e2, e3]` 进行优化，`new` 配合 `push` 还是会导致 `Vec` 容量变化的开销。这个优化可见 [Slice length](https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length) 。
    1. 把 element 变为 ()。pattern：`(@SUBST; $_elem: expr) => { () };`
    2. 然后就可以把重复的 element 变为 [()]。再利用 <[()]>::len(&[()]) 取得 count。
        
        ```rust
        (@COUNT; $($elem: expr), *) => {
            <[()]>::len(&[$(count!(@SUBST; $elem)), *])
        }
        ```
        
    3. `()` 不占用任何空间，在这个情况下是可以在编译时就知道长度。所以用`()` 类型，保证了不会产生额外的内存分配。
27. [1:24:48](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=5088s) 其他的计数方法，参考： [5.2. Counting](https://danielkeep.github.io/tlborm/book/blk-counting.html) 
28. [1:27:27](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=5247s) 通过 `const C: usize = $crate::count!(@COUNT; $($elem: expr), *);` 确定这个计数操作在编译时是确定的。
29. [1:28:32](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=5312s) Hiding internal macro patterns 在文档中隐藏 pattern ，可以将需要隐藏的 pattern 放到一个单独的内部宏中，然后再增加 `#[doc(hidden)]` ，这样内部宏在文档中就不可见了。
30. [1:31:13](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=5473s) Other collection literals 实现了 Vec 的初始化宏，但是也可以对其他的数据集合实现宏，类似 HashMap 的初始化宏。在这里我也实现了 HashMap 的宏，参考 [maplit](https://docs.rs/maplit/latest/maplit/index.html) 。自己实现的和 [maplit](https://docs.rs/maplit/latest/maplit/index.html) 中的几乎类似。
31. [1:33:00](https://www.youtube.com/watch?v=q6paRBbLgNw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=2&t=5580s) 视频中实现的和标准库中的宏的对比。
    1. [https://doc.rust-lang.org/src/alloc/macros.rs.html#41-51](https://doc.rust-lang.org/src/alloc/macros.rs.html#41-51)