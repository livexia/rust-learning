# **[Crust of Rust: Subtyping and Variance](https://youtu.be/iVYWDIW71jk)**

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时： 7h40min**

1. 1h52min，完成第一遍视频
2. 4h02min，简单的实践，简单的阅读了三个最主要的学习材料
3. 1h46min，完成第二遍视频的记录，巩固了对于 invariant 的理解

## 总结

这个知识点和之前的排序几乎相反了，排序的点更多的是和 Rust 无关，更多的算法的具体实现。而今天的内容更多的是一个 Rust 的机制，这个机制并不是很直接，实际上可能也不是很常用，很多时候写代码即使遇到相关的错误，可能也因为错误的理由而修改正确，所以其中的实践代码是很简单的。实践的内容更多的是尝试以相同的路线学习一遍视频中提到的点，所以笔记中有很多的内容应该来自于实践的部分了，然后在最后第二遍视频笔记的时候进行二次确认。

Subtyping and Variance 是一个有些难以理解的概念，视频主要的内容是希望通过实践理解 Subtyping and Variance 对写代码的作用。如果需要完全理解这个概念，建议去学习参考前三条。covariant 这个倒是比较好理解，但是 contravariant 就多了一些理解困难，有一种“负负得正”的感觉。而对于 invariant 即便是看完第一遍视频，然后读完参考资料，对于为何需要 invariant 并不能完全理解，第二遍视频时关于 invariant 的部分才让我真正的理解了为什么需要 invariant 。即便是无法完全理解这几种 variance ，我还是能明白这几种的差别，如果遇到了问题，起码知道该如何根据机制调整代码了。理解了 variance 能够对 lifetime 有更好的理解，最初实践代码的问题，实际上修改很简单，很可能不需要理解今天涉及的任何内容，就可能误打误撞的修改实现代码正常运行。但是理解今天的内容，对于正确理解 Rust 的行为是很重要的。

## **内容**

1. Variance
    1. 在 Rust 中大部分 Variance 的情况只影响 lifetime
2. Subtyping
    1. `‘static` 是所有其他生命周期的 Subtyping
3. Covariant
4. Contravariant
    1. fn(T) -> ()
5. Invariant 一般和可变性有关， `*mut T` `&mut T` `UnsafeCell<T>`
6. 简单涉及 PhantomData 和 Drop Check
    1. `PhantomData<T>` 为了告知 Drop Check 这个类中存在 T 需要检查 T 的可用性
    2. `PhantomData<fn() → T>`  marker for Covariant
    3. `PhantomData<*const T>`  marker for Covariant
    4. `PhantomData<fn(T)>`     marker for Contravariant
    5. `PhantomData<fn() → T>`  `PhantomData<fn(T)>`      marker for Invariant
    6.  `PhantomData<*mut T>`      marker for Invariant
    7. may_dangle 、 [Drop Check](https://doc.rust-lang.org/nomicon/dropck.html#drop-check)
7. [Tuple structs](https://doc.rust-lang.org/1.9.0/book/structs.html#tuple-structs)
8. NonNull `*mut T` but non-zero and covariant. [https://doc.rust-lang.org/std/ptr/struct.NonNull.html](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)

## **参考**

1. The Rsut Reference [Subtyping and Variance](https://doc.rust-lang.org/reference/subtyping.html#subtyping-and-variance)
2. The Rustonomicon [Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html#subtyping-and-variance)
3. https://github.com/sunshowers-code/lifetime-variance
    1. [https://lifetime-variance.sunshowers.io/index.html](https://lifetime-variance.sunshowers.io/index.html)
4. [Subtyping](https://en.wikipedia.org/wiki/Subtyping)
5. [Covariance and contravariance (computer science)](https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science))
6. String::strip_prefix [https://doc.rust-lang.org/std/string/struct.String.html#method.strip_prefix](https://doc.rust-lang.org/std/string/struct.String.html#method.strip_prefix)
7. Associated types [https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html](https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html)
8. Non-Lexical Lifetimes [https://rust-lang.github.io/rfcs/2094-nll.html](https://rust-lang.github.io/rfcs/2094-nll.html) [https://rust-lang.github.io/compiler-team/working-groups/nll/](https://rust-lang.github.io/compiler-team/working-groups/nll/)
9. [Why &mut T is not covariant with T?](https://users.rust-lang.org/t/why-mut-t-is-not-covariant-with-t/54944)

## **实践**

### strtok

一个 C++ 的函数，类似于 `String::strip_prefix` ，功能是根据传入的分隔符，取得给定字符串直到分隔符的内容，并且从字符串中剔除这部分。实现并不复杂，函数内部的操作并不是关键，关键在于函数的定义上。strtok 接受一个字符串 `s: &mut &str`  和一个 `delimiter: char` ，返回一个 `&str` 。

最初的函数定义：`pub fn strtok(s: &mut &str, delimiter: char) -> &str { ... }`

这个函数定义会导致编译错误，因为传入参数存在两个引用，返回存在一个引用，编译器无法自动确定选择参数中的哪一个引用的生命周期，所以需要增加生命周期。

增加生命周期的函数定义：`pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str { ... }`

在这个定义中，所有的生命周期都是一致的，编译并不会失败，但是并不正确。因为 `s` 不一定要和 `s` 指向的字符串拥有一样的生命周期，实际上在使用中也是这样的。

考虑以下测试用例：

```rust
let mut s = "hello world";
let hello = strtok(&mut s, ' ');
assert_eq!(hello, "hello");
assert_eq!(s, "world");
```

`s` 的类型是 `&‘static str` ，传入 `strtok` 的参数 `&mut s` 的类型是 `&mut &’static str` 。因为在这个 `strtok` 定义下，因为范型的约束那么 `’a` 就是 `‘static` 了。

因为 `strtok` 的参数类型是 `&mut &str` 的，根据如果类型 `&mut T` ，类型 `T` 必须是 invarinat 的，所以 `&str` 必须是 `invarinat` 的，也就是说对于这个定义，要求传入参数类型为 `s: &'a mut &'a str` 也就实际传入的参数中不存在生命周期缩短的情况，考虑实际传入 `&'static mut &'static str` 那么在 strtok 中的生命周期 `‘a` 就是 `‘static` 而不会将 `‘static` 缩短到 `‘a` 。在看测试用例中，传入的是 `&mut s` ，显示的补充上生命周期，s 的生命周期是 `‘static` ，那么 `&mut s` 的生命周期也应该是 `’static` 也就是说这个可变引用会存活到程序结束。但是在 `assert_eq!(s, "world");` 中实际上是进行了不可变引用，所以同时存在了可变和不可变引用，导致错误。

经过这样的分析，那么可以发现一种解决办法，那就是让传入参数 `s` 中的两个生命周期不一致即可。

不一致生命周期的函数定义：`pub fn strtok<'a, 's>(s: &'a mut &'s str, delimiter: char) -> &'s str { ... }` 

这个时候 `&’s str` 还是 invariant 的，也就是在这个测试用例中 `‘s` 会成为 `‘static`，而 `‘a` 则会是这个可变引用的生命周期，编译器可以自动猜测，在这个测试用例中，实际上在 hello 赋值后这个可变引用就被丢弃了，如果在最后的 `assert_eq` 语句后增加对这个可变引用的使用，就会导致错误，因为编译器认定这个可变引用应该活到最后的使用结束，于是又出现了同时存在可变和不可变引用的使用了。

```rust
let mut s = "hello world";
let x = &mut s;
let hello = strtok(x, ' ');
// drop(x);
assert_eq!(hello, "hello");
assert_eq!(s, "world");
dbg!(x);
```

实际上编译器可以对可变引用的生命周期进行自动猜测，所以 `s` 的类型可以是 `&mut &’s str` ，也就是省去一个生命周期标注，但是为了明确表示 `s` 中的两个生命周期是不一致的，所以建议还是将 `s` 的类型写为 `&'_ mut &'s str` 。

最后的函数定义：`pub fn strtok<'s>(s: &'_ mut &'s str, delimiter: char) -> &'s str { ... }`

对于 `s` 中的两个生命周期，需不需要增加约束？也就是说内部的生命周期 `’s` 要长于 `‘_` 的约束？实际上编译器自动存在这个约束，如果没有这个约束，也就是说一个活的较久的引用，指向一个活的较短的引用，那么在较短引用被销毁了，较长引用就指向了一个未知指针了。

### 加强实践：完成对  https://github.com/sunshowers-code/lifetime-variance 实例代码的学习

对 [1.4. Variance in practice](https://lifetime-variance.sunshowers.io/ch01-04-variance-in-practice.html) 的内容进行实现，实现每步的实践。阅读了代码之后发现，实际上和 strtok 几乎一致，所以不再自己实践了。

## 学习内容

主要的内容就是 Subtyping 和 Variance ，对于 Subtyping 可以看一下[维基百科](https://en.wikipedia.org/wiki/Subtyping) ，而对于 Variance 维基百科上也存在对应的内容：[Covariance and contravariance](https://en.wikipedia.org/wiki/Covariance_and_contravariance_(computer_science)) 。Subtyping 中文应该是子类型，Variance 应该是变形，Covariant 应该是协变，Contravariant 应该是逆变，Invariant 应该翻译成不变，但是感觉并没有英文来的直接。所以以下的内容都是使用英文的词语。为了不增加混淆，我决定先熟悉 Rust 中的情况，再进一步确认是否要再阅读维基百科的必要。

Rust 中的 Subtyping 机制是作用于生命周期上的，并不作用于其他类型，例如 Trait 、Struct 。所以 Variance 也是作用于生命周期上的，涉及到 Variance 的场景和生命周期都是有关联的。

### **The Rsut Reference [Subtyping and Variance](https://doc.rust-lang.org/reference/subtyping.html#subtyping-and-variance)**

包含简单的 subtyping 和 variance 的说明，最重要的是其中类型的 variance 由编译器自动决定的一个表格。

### **The Rustonomicon [Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html#subtyping-and-variance)**

文章说明 Rust 的 subtyping 并不于其他语言中的 subtyping，因为 Rust 中的 subtyping 和 variance 实际上只有生命周期才存在 lifetime ，而 trait 并不遵循 subtyping 的机制，所以在文章开头就假设一种扩展式的 Rust ，一种 *Objective Rust* ，用来更好的解释 Rust 中的 variance 。

### 代码演示 **[Lifetime variance in Rust](https://lifetime-variance.sunshowers.io/index.html#lifetime-variance-in-rust)**

是一系列的关于 Variance 的代码演示，首先利用正确和错误的代码熟悉 Rust 中生命周期 Variance 的直觉，然后更加直接的说明 Variance 的规则，说明了冲突 Variance 的情况，最后通过一个练习代码，说明在实际中可能需要的使用场景。

### Covariant

考虑两个生命周期 `‘a` 和 `‘b` ，而且 `‘b: ‘a` 也就是说 `‘b` 长于 `‘a` ，那么对于一般情况，考虑一个参数类型是 `&’a T` ，但是实际调用中得到了 `&’b T` ，因为 `‘b` 长于 `‘a` ，所以这样是合法的，想象一下，函数需要一个较短生命周期的参数，那么当然也可以传入较长生命周期的参数，因为函数只需要这个生命周期能生活过函数大生命周期即可。这种情况是 covariant 。

### Contravariant

和 covariant 相反的 variance 就是 contravariant ，从单词上就可以看出是和 variance 相反的，的确如此，rust 中 contravariant 只有一种情况。考虑一个类型是函数类型， `fn(T) → ()` 在这个类型中 `T` 就是 contravariant 的。假设一个函数 `test` 接受一个参数 `f` 也就是 `fn test(f: fn(T) → () ) {}` ，contravariant 的意思就是说，如果 `T` 是 `U` 的子类型（ subtyping ），这个函数 `test` 允许接受一个类型为 `fn(U) → ()` 的参数，假设 `T` 为 `‘static` 那么 `U` 就可以是 `‘a` 。为什么呢？函数 test 需要一个能够支持处理生命周期为 `‘static` 的函数，那么给一个能够支持处理生命周期较短的 `‘a` 的函数当然是可以的。这里非常的迷惑，需要再思考一下内部的情况，假设 test 函数如下：

```rust
fn test(f: fn(&'static str) -> ()) {
    let s = "hello, world";
    f(s);
}
```

在这个例子中，`s` 的类型是 `&‘ static str` ，如果给定的函数是 `fn(&’a str) → ()` ，对于这个函数中的 `&’a str` 这个类型，所以是能够处理 `s` 的，实际上是符合 covariant 规则的，所以对于类型 `fn(T) → ()` 来说，这个类型中的 `T` 实际上是 contravariant 的。

### Invariant

contravariant 已经足够困难解释和说明了，variance 中的最后一种 invariant 好像就更加困难了。考虑函数 strtok ，参数 s 的类型是 `&mut &str` 。我们先将这个类型中的 `&str` 部分看作是类型 `T` ，并且加上生命周期表示，那么对于类型 `&’a mut T` 中涉及到的生命周期 `‘a` ，应该就是 covariant 的。也就是说对于需要类型为 `&’a mut T` 的参数的函数，允许给入类型为 `&’static mut T` 的变量。那么在这个情况中， `T` 的variance 是什么呢？因为 Rust 中的 subtyping 和 variance 实际上只有生命周期才存在 lifetime ，而 trait 并不遵循 subtyping 的机制。假设 `T` 是 covariant 的，考虑 `T` 为 `&’a str` ，那么如果传入的变量类型是 `&’static str` 因为 `T` 是 covariant 的，所以这是允许的，那么实际上在函数内部 `‘static` 实际上是被缩小到了 `‘a` ，如果不是 `&mut T` 而是 `&T` 那么实际上是没有关系的，因为这个生命周期的缩小并不会改变什么，内部仅仅只是把 `‘static` 看成了 `‘a` 而并没有修改生命周期，但是如果是 `&mut T` 那么在内部实际上就修改了 `T` 的生命周期了，毕竟这是一个可变的，那么可能就导致生命周期被缩短了，但是对于实际的变量来说，它只知道自己还是 `‘static` 的，那么就会继续当成 `‘static` 来用，实际上就产生了一个悬空引用。所以在 `&mut T` 中 `T` 必须是 invariant 的。

直播中的示例代码：

```rust
fn foo(s: &mut &'a str, x: &a' str) {
		*s = x;
}

let mut s: &'static str = "hello world";
let x = String::new();
foo(&mut s, &x);
drop(x);
println!("{}", s);
```

假设 `&mut T` 中的 `T` 是 `covariant` 的，那么理论上示例代码是能够正常运行的。因为 `x` 是一个生命周期 `‘static` 的变量，作为参数传入一个需要 `‘a` 生命周期函数，根据 covariant 这个是可以接受的。在 `foo` 里实际上将传入参数的生命周期修改了，也就是说 `s` 实际上不再是 `‘static` 的了，理论上在 `drop(x)` 后 `s` 已经无法访问了，但是实际上在 `s` 看来，自己明明还是 `‘static` ，也就是产生了一个悬空的指针。这就是为什么 `&mut T` 中的 `T` 是 `invariant` 的。

那么为什么这个测试无法通过呢？考虑函数签名：`foo(&mut &str'a s, &‘a str);` 因为 s 类型是 `&‘static str` 所以函数调用中实际就是 `foo(&mut &‘static str, &‘a str);` 也就是说要求 `‘a` 和 `‘static` 一样长，而实际上这两个生命周期并不一样长，所以导致编译错误。

根据 reference book 和 nomicon 实际上除了 `&mut T` 的 `T` 需要是 invariant ，对于那些提供内部可变性的类型，内部的 `T` 也是 invariant 的，例如 `Cell<T>` 、`UnsafeCell<T>` 。

### Struct 生命周期参数的 Variance 怎么确定

对于同一生命周期参数在同一个 `struct` 的多个字段同时使用了，如果所有字段都同意一种 variance ，那么这个生命周期就是这种 variance 。如果存在冲突，那么这个生命周期参数就是 invariant 的。通过这个方法，可以控制对于范型 `T` 的 variance 。

对于 `struct Test<T>` ，在不考虑其他字段的情况下，增加字段 `_marker: PhantomData<fn() → T>` 使得 T 是 covariant 的，增加字段 `_marker: PhantomData<fn(T) → ()>` 使得 T 是 contravariant 的，而增加字段 `_marker: PhantomData<fn(T) → T>` 使得 T 是 invariant 的，不一定需要使用 PhantomData ，也可以增加其他的字段，但是使用 PhantomData 可以使得这个字段中的 `T` 并不会被分配。

参考：[https://lifetime-variance.sunshowers.io/ch01-03-conflicts-and-type-parameters.html](https://lifetime-variance.sunshowers.io/ch01-03-conflicts-and-type-parameters.html)

## **笔记**

笔记很多的部分都在上面实践和学习内容的部分，所以笔记中只简单记录发生的事情。

1.  [0:02:30](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=150s) Practical variance in strtok 考虑 Variance 在 strtok 中场景，简单介绍 strtok 函数的作用，和简单实现 strtok。
2.  [0:13:00](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=780s) strtok 参数中 s 使用同一个生命周期而导致测试用例中无法调用 strtok 的原因分析。
3.  [0:17:26](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=1046s) Pretending to be the compiler 编译器根据传入参数的生命周期，自动使得类型 &’a mut &’a str 中的生命周期成为了 ‘static ，于是这个可变引用的生命周期成为了 ‘static 而测试用例后续仍有对统一变量的不可变的引用的使用，这导致了编译错误。
4.  [0:19:03](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=1143s) Shortening lifetimes 缩短生命周期，covariant 能允许缩短生命周期，也就是对于一个函数需要 `&‘a T` 而传入了 `&’static T` 这是被允许的，首先从直觉上来说，对于一个需要较短生命周期参数的函数，得到了一个更长生命周期的变量，那么当然没有问题。这就是 covariant ，`’static` 是所有其他生命周期的 subtyping 。
    
    对于代码：
    
    ```rust
    fn it_works() {
    		let mut x = "hello":
    		strtok(&mut x, ' ');
    }
    ```
    
    为什么这个能编译成功呢？因为虽然 `x` 的生命周期应该是 `‘static` 但是编译器知道 `x` 在 `strtok` 调用后就不在使用了，所以可以将 `‘static` 缩短到函数 `it_works` 的范围，如果 `strtok` 调用后还有对 `x` 的不可变引用使用，依然会导致编译错误，因为 `x` 的生命周期至少要到 `x` 不可变引用结束之后才能结束，而这个可变引用的生命周期即使不是 `‘static` 也会是 `x` 的生命周期，也就是说不可变引用和可变引用中产生了重叠，编译错误。
    
5.  [0:25:40](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=1540s) Subtypes  [0:29:12](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=1752s) Covariance/covariant 更多的关于 subtyping 和 covariant
    
    covariant 能允许缩短生命周期，也就是对于一个函数需要 `&‘a T` 而传入了 `&’static T` 这是被允许的，首先从直觉上来说，对于一个需要较短生命周期参数的函数，得到了一个更长生命周期的变量，那么当然没有问题。这就是 covariant ，`’static` 是所有其他生命周期的 subtyping 。
    
6.  [0:33:15](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=1995s) Contravariance/Contravariant 参见学习内容中的 Contravariant 部分。
    
    对于 `&T` ： `‘static T` 比 `‘a T` 更有用。而对于类型 `Fn(&T)`：`Fn(&’static T)` 比 `Fn(&’a T)` 更严格，所以对于 `Fn(&T)` 来说 `T` 是Contravariant
    
7.  [0:42:14](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=2534s) Invariance/Invariant
    
    为什么 &mut T 中的 T 是 invariant 的。
    
    视频中的示例代码：
    
    ```rust
    fn foo(s: &mut &'a str, x: &a' str) {
    		*s = x;
    }
    
    let mut s: &'static str = "hello world";
    let x = String::new();
    foo(&mut s, &x);
    drop(x);
    println!("{}", s);
    ```
    
    假设 `&mut T` 中的 `T` 是 `covariant` 的，那么理论上示例代码是能够正常运行的。因为 `x` 是一个生命周期 `‘static` 的变量，作为参数传入一个需要 `‘a` 生命周期函数，根据 covariant 这个是可以接受的。在 `foo` 里实际上将传入参数的生命周期修改了，也就是说 `s` 实际上不再是 `‘static` 的了，理论上在 `drop(x)` 后 `s` 已经无法访问了，但是实际上在 `s` 看来，自己明明还是 `‘static` ，也就是产生了一个悬空的指针。这就是为什么 `&mut T` 中的 `T` 是 `invariant` 的。
    
    那么为什么这个测试无法通过呢？考虑函数签名：`foo(&mut &str'a s, &‘a str);` 因为 s 类型是 `&‘static str` 所以函数调用中实际就是 `foo(&mut &‘static str, &‘a str);` 也就是说要求 `‘a` 和 `‘static` 一样长，而实际上这两个生命周期并不一样长，所以导致编译错误。
    
8.  [0:50:00](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=3000s) &'a mut T covariance in 'a 为什么类型 &'a mut T 中的 ‘a 是 covariant 的？
    
    示例代码：
    
    ```rust
    fn bar() {
    		let mut y = true;
    		let mut z = &mut y;
    
    		let x = Box::new(true);
    		let x: &'static mut bool = Box::leak(x);
    		let _ = z;
    		z = x;   // &'y mut bool = &'static mut bool
    		drop(z)
    }
    ```
    
    即使在 `z = x;   // &'y mut bool = &'static mut bool` 中将一个 `&'static mut bool` 类型赋值给 `&'y mut bool` ， `bool` 并没有变化，也就是说并没有可能将一个 `bool` 的子类型塞入 bool 。
    
9.  [0:57:57](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=3477s) What went wrong in our strtok test?  [1:02:24](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=3744s) Fixing strtok 根据 variance 规则修复函数 strtok，详见实践中的 strtok 的部分。
10.  [1:07:34](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=4054s) Why is 'b: 'a not needed? [1:10:11](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=4211s) Is 'b: 'a implied for &'a &'b? 在类型 `&’a mut &’b str` 中不需要约束 `'b: 'a`，如果没有这个约束，也就是说生命周期 `‘a` 有可能比 `‘b` 长，会导致编译错误，实际中 `‘a` 会因为 covariant 的机制缩短到 `‘b`。
11.  [1:09:08](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=4148s) Shortening &'a mut and NLL 编译器会自动缩短可变引用的生命周期，理论上新建一个可变引用应该直到这个新建所在的块的结尾都可用，但是如果这个可变引用在块中某一处开始不再使用，编译器（NLL）可以将这个可变引用的生命周期缩短。NLL 可见[https://rust-lang.github.io/compiler-team/working-groups/nll/](https://rust-lang.github.io/compiler-team/working-groups/nll/)
12.  [1:12:54](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=4374s) Variance, PhantomData, and drop check 简单涉及 drop check ，因为后续直播存在关于 drop check 的部分，所以这里不进行详细的记录。
    
    在类型定义中增加 PhantomData 的字段
    
    1. `PhantomData<T>` 为了告知 Drop Check 这个类中存在 T 需要检查 T 的可用性
    2. `PhantomData<fn() → T>`  marker for Covariant，为了告知 Drop Check 这个类不需要检查 T 的可用性
    3. `PhantomData<*const T>`  marker for Covariant
    4. `PhantomData<fn(T)>`     marker for Contravariant
    5. `PhantomData<fn() → T>`  `PhantomData<fn(T)>`  marker for Invariant
    6.  `PhantomData<*mut T>`      marker for Invariant
    7. `PhantomData<fn(T) → T>` marker for Invariant
13. 介绍其他更加细节的内容
    1.  [1:28:06](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=5286s) Reasons for changing variance 为什么需要改变 variance ，改变 variance 需要很谨慎，主要在提升类型可用性和 unsafe 的代码中，才会改变 variance 。
    2.  [1:30:47](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=5447s) for{'a} and variance
    3.  [1:31:51](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=5511s) Mutating through *const T
        1. 这个是不可能的 &T → *const T → *mut T→ &mut T
        2. 这个是可能的 &mut T → *mut T → *const T → *mut T→ &mut T
            1. 在这个过程中可能会改变 T 的 variance
    4.  [1:33:29](https://www.youtube.com/watch?v=iVYWDIW71jk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7&t=5609s) NonNull<T> NonNull 类型，NonNull<T> 是 covariant 的，所以使用时要小心，可以通过增加 `PhantomData` 修改 variance 为 invariant 。