# **[Crust of Rust: The Drop Check](https://youtu.be/TJOFSMpJdzg)**

## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时：  6h**

1. 1h24min，完成第一遍视频
2. 2h56min，完成 nomicon 中 drop check 的学习，完成基本的代码实践，但是没有在实践中增加对实践的说明，代码注释中包含了大量的说明。等待第二遍视频的时候进行统一总结和记录。
3. 1h40min，完成第二遍视频，完成笔记和示例代码的说明。

## 总结

Drop Check 也和 Variance 一样是一个较为边缘的概念，而且根据视频的学习和看了 nomicon 的内容，会发现这个部分的内容比 Variance 更加不确定，因为这些内容中都体现了一种未来可能会改变的感觉。视频中主要的涉及内容就是两部分，一是 `#[may_dangle]` 二是 `PhantomData` ，如果是第一遍看视频，可能会有疑惑，通过 `#[may_dangle]` 应该是降低了 Drop Check 的限制，但是好像又过使用 `PhantomData` 增加了限制，灵活和安全的背后实际上是复杂的设计思路和实现。不能说今天学到的内容具体对编码存在多少的实际帮助，但是的确增加了对生命周期的理解，相比于编程内容，Variance 和 Drop Check 更像是直接的对 Rust 编译器的学习，正如 Rust Nomicon 来说，理解这些，是为了更好的了解为什么。

## **内容**

1. 使用 `Box` 。
2. 利用 `#[may_dangle]` 降低 Drop Check 的限制。
3. `PhantomData` 确保 Drop Check 的限制不被完全消减。
4. `NonNull` 替换 `*mut` 实现类型是 covarinat 的。
5. 简单理解 Drop Check 的机制。

## **参考**

1. Nomicon Drop Check [https://doc.rust-lang.org/nomicon/dropck.html](https://doc.rust-lang.org/nomicon/dropck.html)
2. [0769-sound-generic-drop](https://rust-lang.github.io/rfcs/0769-sound-generic-drop.html)
3. ManuallyDrop [https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)
4. Box [https://doc.rust-lang.org/std/boxed/struct.Box.html](https://doc.rust-lang.org/std/boxed/struct.Box.html)
    1. into_raw [https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw)
5. PhantomData [https://doc.rust-lang.org/std/marker/struct.PhantomData.html](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
    1. [https://doc.rust-lang.org/nomicon/phantom-data.html](https://doc.rust-lang.org/nomicon/phantom-data.html)
6. NonNull [https://doc.rust-lang.org/std/ptr/struct.NonNull.html](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)
7. Empty [https://doc.rust-lang.org/std/iter/fn.empty.html](https://doc.rust-lang.org/std/iter/fn.empty.html)
8. [Niche optimization, NonZero and `improper_ctypes`](https://users.rust-lang.org/t/niche-optimization-nonzero-and-improper-ctypes/41399)
9. https://github.com/rust-lang/rust/issues/34761

## **实践**

实现一个 Boks ，这个 Boks 是对 Box 的一个包装类。下面只给出具体 Boks 的部分实现，视频演示中涉及的示例代码，可见笔记或代码仓库。

### 类型定义

最初 `p` 的类型是 `*mut T` ，但是后来将其改为 `NonNull<T>` ，因为 `*mut T` 的 `T` 是 invariant 的，`NonNull<T>` 的 `T` 是 covariant 的，为了保证 `Boks` 是 covariant 的，所以进行替换。

另一个字段 `_m: PhantomData<T>` 是为了告知编译器我们实际上持有 `T` ，最后清理 `Boks` 的时候，需要清理 `T` 。

```rust
pub struct Boks<T> {
    p: NonNull<T>,      // NonNull<T> replace *mut T to make sure Boks is covariant
    _m: PhantomData<T>, // PhantomData<T> to make sure when drop the Boks drop checker will care about T
}
```

### 新建 Boks

使用 `Box::new` 创建一个 `Box` ，然后利用 `Box::into_raw` 将其转为指针 `*mut T` ，最后因为 `Box::new` 一定不会产生空指针，所以使用 `NonNull::new_unchecked` 将 `*mut T` 转为 `NonNull<T>` 。需要使用 `unsafe` 因为 `new_unchecked` 不安全的。

```rust
impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            // use box to create data on heap, always return nonnull pointer
            // Safety: because Box never give out null pointer, so it is fine to use new_unchecked
            p: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(t))) },
            _m: PhantomData,
        }
    }
}
```

### 清理 Boks

因为新建了 `Box` ，但是从没有回收内存，所以需要对 `Boks` 实现 `Drop` 。使用 `Box::from_raw` 将指针转为 `Box` ，然后就能被回收了。

对 `Drop` 增加 `unsafe` ，对 `T` 增加 `#[may_dangle]` 和文件头部增加 `#![feature(dropck_eyepatch)]` ，放宽 Drop Check 的限制，告知编译器，在 `drop` 中一定不会访问 `T` 。

```rust
#![feature(dropck_eyepatch)]

...

// Safety: there is no access the T inside the drop so it'is ok to use may_dangle attritube
unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // Safety: this is fine because the pointer came from a Box,
        // so it is safe to convert back to a box to drop it.
        let _ = unsafe { Box::from_raw(self.p.as_ptr()) };
    }
}
```

### 对 Boks 实现 Deref 和 DerefMut

在这里显示 Deref 的实现，DerefMut 实现类似。

```rust
impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: pointer came from a Box<T>,
        // so it is fine to return a &T from the pointers
        unsafe { self.p.as_ref() }
    }
}
```

### 测试类型 Oisann

**定义：**

```rust
use std::fmt::Debug;
pub struct Oisann<T: Debug>(T);
```

**在 Drop 中访问了 T 的实现：**

```rust
impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        // Oisann drop access the T, so T msut outlive self
        // if T dropped before self, then this is a dangle pinter
        println!("inside oisann: {:?}", self.0)
    }
}
```

## 学习内容

### Drop Check

主要的内容都是来自于 [https://doc.rust-lang.org/nomicon/dropck.html](https://doc.rust-lang.org/nomicon/dropck.html) ，简单进行一些记录。

清理变量的顺序和变量定义的顺序相反。struct 和 tuple 中字段的清理顺序则是和它们定义的顺序相同。更加详细的清理顺序可见：[https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md](https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md)

Sound generic drop 这是一个我并不完全理解的概念，具体可以阅读 [0769-sound-generic-drop](https://rust-lang.github.io/rfcs/0769-sound-generic-drop.html) ，我不知道应该如何翻译这个 sound ，但是我的理解应该是更加倾向于 合理的/健全的 ，结合上下文，我的理解 sound generic drop 就是对于一个范型类型，需要进行合理的清理检查，范型存在不同的生命周期，soundly implement drop 就是显式的实现了 Drop 。Drop Check 中最重要的规则是，对于一个实现了 Drop 的范型类型，范型参数必须严格的比类型活的长。**For a generic type to soundly implement drop, its generics arguments must strictly outlive it.**

服从这条规则，必然会使得 borrow checker 满意，满足这个规则，充分但不必定是健全的，不过如果一个类型服从这条规则，那么这个类型一定是健全的，可以被清理。

那么为什么不一定需要去满足这个规则呢？那是因为在有的 Drop 的实现中，并不会访问被借用的数据，即使给定类型给了能力去进行这样的访问，或者在知道确切的清理顺序，并且借用的数据即使在类型清理后，仍是可用的，而 borrow checker 并不知道这一点。

当时为一个类型实现 Drop 的时候，borrow cherker 并不知道这个 Drop 的实现中，是否有对借用数据的访问，即使我们知道在实现这个 Drop 的时候，我们确认了不可能会访问内部的数据，borrow checker 默认即使是在清理的时候（ Drop 中 ），类型是可能会访问借用的数据的，所以 drop checker 强制一个值中的所有借用数据的生命周期严格超过该值。

也就是一般来说，对于一个类型 `struct Inspector<T>(T)` 而这个类型实现了 `Drop` ，那么需要 `T` 的生命周期超过 `Inspector` 。

这个规则严格的保证了不会出现访问已经释放的情况，但是这个规则也限制了部分场景，如果我们知道一个类型的 drop 函数，也就是在 Drop 过程中不会访问借用的数据，那么理论上我们是可以允许借用的数据在值清理前就被清理（ Drop ）了，因为在清理（ Drop ）值的时候，并不访问内部的借用数据。

Rust 现在的规则并不是十分明确，而且过于严格。但是存在一个方法能暂时避开这个限制。

### 逃避 Drop Check 严格的限制 [An Escape Hatch](https://doc.rust-lang.org/nomicon/dropck.html#an-escape-hatch)

这个限制是为了确保所有的借用和生命周期关系是确切的（ sound ）。那么逃避开这个限制，是不安全的也就不难理解了。

这个方法是一个不稳定版本的属性设置，通过这个属性可以不安全的宣传一个范型类型的解构函数（ Drop ）是绝对不会访问任何过期的数据的，即使这个数据的类型确保了是能够在解构中进行访问的。

这个属性是 `may_dangle` 具体可见 [RFC 1327](https://github.com/rust-lang/rfcs/blob/master/text/1327-dropck-param-eyepatch.md) 。使用需要在头部加上：`#![feature(dropck_eyepatch)]` 同时也要在 Drop 的实现中加上  `usafe` 和 `#[may_dangle]` ，例如：`unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {}` 。这个属性可以加在任意的范型和生命周期类型参数前，对确保不访问的范型和生命周期前加上即可，但是如果在 drop 中会访问的，如果加上这个参数，那么就直接的违反了规则，也就是这个代码是的的确确不安全的。

有的时候对于没有访问借用数据是明显直接的，但是当面对一个范型类型参数，这种访问可能是非直接的。目前存在两种非直接的访问：

1. 发起一个回调
2. 通过 Trait 方法调用

即使是这种访问，也可以进一步的隐藏。如果是存在访问的情况，但是增加了 #[may_dangle] 属性，这将导致类型的不安全，所以最好是能不用这个属性就不用。这个部分的具体情况请移步文章，我这里的表述必然是不完整和存在错误的，[An Escape Hatch](https://doc.rust-lang.org/nomicon/dropck.html#an-escape-hatch) 。

### 清理顺序相关

虽然默认的清理顺序是定义好的，但是依赖在这个顺序上是脆弱而隐性的（ fragile and subtle ） 。当清理顺序重要的时候，最好使用 `[ManuallyDrop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)` wrapper。

### `[ManuallyDrop](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)`

一个组织编译器自动调用 T 的解构函数的wrapper，这个 wrapper 是无开销的。这里只是简单的记录，我并不没有完成这个内容的阅读和学习。

## **笔记**

1.  [0:00:00](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=0s) Introduction Drop Check 也是一个特别的概念，但是会比 Variance 出现的较多，大多数情况下都和 unsafe 代码开发有关。
2.  [0:01:39](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=99s) Boks: A Norwegian Box 实现一个 Box 的包装 Boks。`Boks` 仅有一个字段 `p` 是一个指针类型 `p: *mut T` 。在新建 `Boks` ，使用 `Box::into_raw(Box::new(t))` 生成一个指针。
3.  [0:04:22](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=262s) Freeing a Boks 清理 `Boks` ，因为新建了指针，但是从未清理，所以需要对 `Boks` 实现 Drop。
4.  [0:05:56](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=356s) Dereferencing a Boks 为了使用 `Boks` ，需要实现 `Deref` 和 `DerefMut` 。
5.  [0:09:40](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=580s) Boks is too restrictive 相比于 `Box` `Boks` 有更加严格的限制。
    
    下列代码将无法通过编译，但是实际上 `Boks` 中并不使用 `&mut y`，所以理论上这种使用应当是被允许的，实际上 `Box` 是能够支持这种实现。
    
    ```rust
    let mut y = 42;
    let b = Boks::ny(&mut y);
    println!("{:?}", y);
    ```
    
    错误提示中，提示 `y` 同时存在可变和不可变的引用，可能在 `Boks` 的 `Drop` 实现中访问了 `&mut y` ，即使我们知道实际上 `Drop` 实现中并不使用 `&mut y` 。编译器默认当一个范型类型实现了 `Drop` 的时候，编译器会推断 `Drop` 中实际上访问了范型数据，在这个例子中编译器就没有办法将 `&mut y` 的生命周期缩短，在函数最后清理 `b` 结束后， `&mut y` 的生命周期才结束，也就是这个时候 `&mut y` 和 `y` 的使用产生了重叠。
    
    绕过 Drop Check 的限制，在头部加上：`#![feature(dropck_eyepatch)]` 同时也要在 Drop 的实现中加上  `usafe` 和 `#[may_dangle]` ，例如：`unsafe impl<#[may_dangle] T> Drop for Boks<T> {}` 。这样上面的示例代码就能正常编译运行。
    
    这个方法并不是稳定的，rust 的开发者还没有确认到底最后的表现是如何的，这是一个权宜之计。
    
    对于一个类型为 &mut T 的，清理这个类型的时候，理论上清理一个可变引用并不是访问，因为只是放弃了引用，但是并不在乎 T 是不是也要一起被清理。
    
6.  [0:26:52](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=1612s) Boks is not restrictive enough 相比于 `Box` `Boks` 又存在不够严格的情况。
    
    考虑一个类型 `struct Oisann<T: Debug>(T)` ，这个类型也实现 `Drop` ，在 `drop` 中会访问 `T` ，具体访问使用就是打印出内部的 `T` ，考虑以下代码：
    
    ```rust
    let mut z = 42;
    let b = Boks::ny(Oisann(&mut z));
    println!("{:?}", z);
    drop(z);
    println!("z dropped");
    ```
    
    在 `b` 清理的时候，我们通过 `#[may_dangle]` 告知编译器我们绝对不会在 `drop` 内部使用 `T` ，在这里就是 `Oisann<T>` ，但是在这里 `Oisann<T>` 的 `drop` 内部使用了 `T` ，也就是说实际上当 `Oisann(&mut z)` 被清理的时候，`drop` 内部还是访问了内部数据，虽然是通过内部数据的 `drop` 间接的访问，也就是说我们违反了我们自己的约定。上面这个代码，在这个情况中会正常运行，输出大致上是这样的。
    
    ```rust
    42
    z droppped
    42
    ```
    
    第一个 `42` 是来自直接对 `y` 的打印，第二个 `42` 则是 `Oisann` 在清理时的打印，很明显这里产生了对悬空指针的访问，这是内存不安全的。为什么会出现这种情况呢？在添加 `#[may_dangle]` 后，编译器认为我们不会对内部的 `T` 做任何事情，包括清理 `T` ，所以 Drop Cheker 也就不会检查生命周期，但是在这个例子中，我们需要清理 `T` ，我们需要考虑 `T` 的生命周期是否长于 `Boks` ，也就是 `z` 的生命周期是否长于 `Boks` 。因为 `Boks` 中的所有字段并不包含一个 `T` ，只有一个指向 `T` 的指针而已，所以需要增加一个字段，`_m: PhantomData<T>,` 这个字段能够起到告知编译器，在清理 `Boks` 的时候，需要清理 `T` ，这个字段有一个特殊的类型也就是 `PhantomData` ，这个类型是无长度的。。
    
7.  [0:35:35](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=2135s) PhantomData and may_dangle
    - `#[may_dangle]` 告知编译器不会访问 `T` 。
    - `PhantomData<T>` 字段告知编译器需要清理 `T`
8.  [0:36:32](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=2192s) What if Oisann didn't touch T? 即使 `Oisann` 不访问 `T` ，但是因为显式的实现了 `Drop` ，所以编译器会默认 `Oisann` 的 `Drop` 实现中访问了 `T` ，所以这里要避免这个问题，可以继续给 `Oisann` 增加 `#[may_dangle]` ，或者直接删除 `Oisann` 的实 `Drop` 实现。
9.  [0:38:10](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=2290s) Boks isn't covariant!  [0:43:49](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=2629s) Boks isn't covariant (in code)! 实现的 Boks 不是 `covariant` 的，因为字段 `p` 类型是 `*mut T` ，所以 `T` 不是 `covariant` 的。
    
    示例代码：
    
    ```rust
    // Variance demo for the Boks
    // with field p: *mut T this test will fail
    let s = String::from("hei");
    let mut box1 = Boks::ny(&*s);
    let box2: Boks<&'static str> = Boks::ny("heisann");
    // is is not allowed because Boks<T> is invariant
    // after use NonNull to replace *mut make Boks<T> covariant this works fine
    box1 = box2;
    ```
    
    在这个示例代码中，`box1` 实际上有一个较短的生命周期 `‘a` ，而 `box2` 有一个较长的生命周期 `‘static` 。如果是 `invariant` 的，也就是当 `p` 的类型是 `*mut T` 时，这个代码无法正常编译。实际上 `Box` 是可以编译通过的，所以 `Box` 并不是 `invariant` 的，而是 `covariant` 。需要修改字段 `p` 的类型，使用类型 `NonNull<T>` 替换 `*mut T` ，在 `NonNull` 的文档中，明确说明了 `NonNull` 是 covariant 的。替换之后，编译就能够正常通过了。
    
10.  [0:48:10](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=2890s) PhantomData T without holding a T 不持有 `T` 的 `PhantomData`

    如果存在一个字段类型为 `PhantomData<fn() -> T >` ，单就这一个字段来说，T 是covariant 的，但是这个形式的 `PhantomData` 并不会告知编译器需要清理 `T` 。在我们的例子中不使用这个形式，是因为我们需要 Drop Checker 会在乎 T 。
    
11.  [0:51:08](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=3068s) std::iter::Empty and variance [1:07:34](https://www.youtube.com/watch?v=TJOFSMpJdzg&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=8&t=4054s) Why the Empty example compiles 
    
    空迭代器的 Variance ，在这里 Empty 中可以用字段 `PhantomData<fn() -> T >` 来实现，但是实际上 Empty 中使用的是 `PhantomData<T>` 。
    
    示例代码：
    
    ```rust
    // Demo for std::iter::Empty
    use std::iter::Empty;
    let mut x = 42;
    let mut empty_it: Empty<Oisann<&'static mut i32>> = Empty::default();
    // struct Empty<T>(PhantomData<T>)
    // let mut o: Option<Oisann<&'static mut i32>> = Some(Oisann(&mut x)); // <- this is wrong
    let mut o = Some(Oisann(&mut x));
    {
        o /* ...<&'a mut i32> */ = empty_it.next(); /* return ...<&'static mut i32> */
        // empty_it produce 'static lifetime get shorten
    }
    // &mut x drop before this
    drop(o);
    println!("{:?}", x);
    drop(x);
    // empty_it drop later is fine, because empty_it is never tied to the x
    // empty_it will always produce the Oisann<&'static mut i32> this is also never tied to the x
    // so this is fine
    let _ = empty_it.next();
    drop(empty_it);
    ```
    
    根据我们的推论这个代码不应该通过编译，但是实际上却通过了编译。清理空迭代器的时候，因为迭代器的类型是 `Empty<T>(PhantomData<T>)` 那么也就会清理 `PhantomData<T>` 也就会清理 `T` 也就是 `Oisann<&mut i32>` ，在 `Oisann` 中访问了 `&mut i32` ，那么理论上应该编译失败，因为这里的确很有可能访问了悬空指针。
    
    但是实际上，编译是完全没问题的，这是因为虽然最后会清理 `T` ，但是这个 `T` 并没有和 `x` 存在真正的关联，也就是实际上这个空迭代器从头到尾都没有产生对 `T` 的引用，也就是不存在对 `x` 的引用，在中间的代码部分：
    
    ```rust
    let mut o = Some(Oisann(&mut x));
    {
        o /* ...<&'a mut i32> */ = empty_it.next(); /* return ...<&'static mut i32> */
        // empty_it produce 'static lifetime get shorten
    }
    ```
    
    因为 Empty 并不产生任何引用，也不持有任何数据，所以空迭代器可以声明自己的生命周期是 `‘static` ，并不实际产生，所以即使宣传可以产生，这也不会有人和影响。
    
    实际上虽然空迭代器产生了一个内部类型是 `&'static mut i32` 的东西（ `None` ），但是因为对于 `&’a mut T` 中， `‘a` 是 covariant 的，所以在这里产生的数据的生命周期被缩短到了变量 `o` 的生命周期，并不影响迭代器。空迭代器的生命周期并没有和 `x` 的生命周期产生绑定。