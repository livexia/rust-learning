## 方法

1. **先看一遍视频**
2. **脱离视频实现视频中的演示内容**
3. **代码说明**
4. **再重新看一遍视频，对关键内容进行记录**
5. **总结**

## 进度

**耗时： 8h25min**

1. 2h20min，完成第一遍视频
2. 3h，完成实践内容
3. 3h5min，完成第二遍视频笔记、代码说明和总结

## 总结

学习使用和实现内部可变性的数据结构，`Cell` 通过移动值实现内部可变性， `RefCell` 使用生命周期实现运行时的动态内部可变性。学习智能指针，`Rc` 是实现在运行时进行共享引用。也简单介绍了线程安全的这些类型，最后介绍了一个特定的智能指针 `Cow`。

智能指针和内部可变性很大程度上是反 Rust 直觉的，所以在完成这个视频之前，对这些数据结构只有一个大致的认知，使用起来总是不顺手。尽管自己实现了视频中的演练，但是对于如何实现 `Cell` 、 `RefCell` 和 `Rc` 中涉及到的细节还是存在一知半解，但是也认识到了这些数据结构并不和 Rust 的设计相违背，是一种在严格的 Rust 约束下实现复杂场景的方法。

因为实践是一个试错的过程，如果将每一个变化都完全的展现在说明中，那会需要很多的时间和记录很多的内容，类似于视频的所有内容文字版。而如果只说明最后正确运行的代码，那么也不能完全解释这样做的原因，所以最后我选择了在代码的实现中说明最后的实现，而在笔记的实现中尝试说明其中的变化。最好还是按照视频的顺序和其中的实现，一步一步说明这样写代码的原因。

## **内容**

1. SmartPointer 智能指针
    1. Arc/Rc/Deref
2. Interior Mutability 内部可变形数据结构
    1. RefCell/Mutex/Cell
3. 不同类型的 Interior Mutability 类型的使用要求和场景是什么
4. Box 不提供 Interior Mutability
5. Cell 的内层是 UnsafeCell
    1. Cell 的常见实用场景 thread_local
6. 实现 RefCell
7. 对 Ref 实现 Deref
8. 对 RefMut 实现 DerefMut
9. 实现 Rc
10. 简单介绍 RwLock 和 Mutex
11. 简单介绍 Arc，类似于 Rc ，但是利用 Atomic Count
12. 简单介绍 std::borrow::Cow

## **参考**

1. [https://doc.rust-lang.org/book/ch15-00-smart-pointers.html](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
2. cell [https://doc.rust-lang.org/std/cell/](https://doc.rust-lang.org/std/cell/)
3. UnsafeCell [https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)
4. thread local [https://doc.rust-lang.org/std/macro.thread_local.html](https://doc.rust-lang.org/std/macro.thread_local.html)
5. Drop [https://doc.rust-lang.org/std/ops/trait.Drop.html](https://doc.rust-lang.org/std/ops/trait.Drop.html)
6. Deref [https://doc.rust-lang.org/std/ops/trait.Deref.html](https://doc.rust-lang.org/std/ops/trait.Deref.html)
7. rc [https://doc.rust-lang.org/std/rc/index.html](https://doc.rust-lang.org/std/rc/index.html)
8. Raw, unsafe pointers [https://doc.rust-lang.org/std/primitive.pointer.html](https://doc.rust-lang.org/std/primitive.pointer.html)
    1. [https://internals.rust-lang.org/t/why-const-t-and-not-t/7749](https://internals.rust-lang.org/t/why-const-t-and-not-t/7749)
    2. https://github.com/rust-lang/rfcs/pull/68
9. NonNull [https://doc.rust-lang.org/std/ptr/struct.NonNull.html](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)
10. PhantomData [https://doc.rust-lang.org/std/marker/struct.PhantomData.html](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
11. [https://doc.rust-lang.org/nomicon/](https://doc.rust-lang.org/nomicon/)
12. Drop Check [https://doc.rust-lang.org/nomicon/dropck.html](https://doc.rust-lang.org/nomicon/dropck.html)
13. `?Sized` [https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html](https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html)
14. RwLock [https://doc.rust-lang.org/std/sync/struct.RwLock.html](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
15. Mutex [https://doc.rust-lang.org/std/sync/struct.Mutex.html](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
16. FFI [https://doc.rust-lang.org/nomicon/ffi.html](https://doc.rust-lang.org/nomicon/ffi.html)
17. Cow [https://doc.rust-lang.org/std/borrow/enum.Cow.html](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
18. 实践参考：
    1. [https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8](https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8)
    2. [https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e11759fb8fd3d8c21d605e0523dda419](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e11759fb8fd3d8c21d605e0523dda419)

## **实践**

### Cell

**安全限制：**

1. 限制1：查看 Cell 的所有方法，可以发现不存在任何的方法返回引用，例如 get 是直接返回 T 而不是 `&T` ，这就保证 `Cell` 总是可变的，因为不存在其他的指向值的引用。
2. 限制2：因为 `Cell` 不实现 `Sync` ，也就是说无法将 `Cell` 的引用传给其他的线程。假设如果有两个线程同时拥有一个 `Cell` 的引用（ `&Cell` ），那么这两个线程就有可能同时修改内部的值。
3. 这两个限制保证了安全，保证了在每一刻只能有一个修改。

```rust
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// Cell is !Sync

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            // SAFETY: we know no-one else is concurrently mutating this value(because !Sync)
            // SAFETY: we know we're no invalidating any references, because we never give out any
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only one thread can mutate
        // (because !Sync), and it is executing this function instead
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }
}
```

### RefCell

**自己实现时存有的疑惑：**

Q：为什么要让 RefCell 的 borrow 和 borrow_mut 返回 Ref 而不直接 返回 &T 呢？

A：如果返回 `&T` ，首先即使是通过判断 `RefCell` 中的 state 保证了 Rust 的引用规则，但是当执行 borrow 后取得的引用 `drop` 时，并不会修改 `RefCell` 中的 state ，因为 `drop` 的只是 `&T` 。因为在`borrow` 和 `borrow_mut` 的引用 `drop` 后我们仍希望使用 `RefCell` 内部的 `T` ，但同时又要动态的检查保证每次的引用是符合 Rust 规则的，所以需要利用一个 Ref 把 RefCell 包装起来。

Q：那么为什么不让 RefCell 的 borrow 和 borrow_mut 返回 &self 而不直接 返回 &T 呢？

A：因为那样无法做到取得内部 &T，也就无法进行读取或者修改了。所以需要返回一个 Ref<T>

**具体实现和使用限制：**

相比于 `Cell` ，`RefCell` 需要额外记录引用情况。记录引用状态的 field 需要是 `Cell` ，需要利用 `Cell` 来保证对引用状态使用是线程安全的。存在3个状态：未引用（`Unshared`），共享引用计数（`Shared(usize)`），独享引用（`Exclusive`）。

**给出引用时：**

1. `borrow`：当引用状态是独享的，那么就不能再给出共享引用。
2. `borrow_mut`：当引用状态是共享的或者是独享的，那么就不能再给出独享引用，而共享引用计数应当加一。

**收回引用时：[0:54:21](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=3261s) RefCell Smart Pointer**

1. 如果 `borrow` 返回是 `&T` ，`borrow_mut` 返回是 `&mut T`，那么没有办法统计这些引用 `drop` 的情况，也就没办法对 `RefCell` 的引用状态进行统计。
2. 使用 `Ref` 和 `RefMut` 类型，包含对 `RefCell` 的引用。通过对 Ref 和 RefMut 实现 Drop 就能对 RefCell 的引用情况进行修改。
3. `Ref` `Drop` 当引用状态为存在一个共享引用时，`drop` 设置引用状态为未共享（未引用）。 当引用状态为存在多个共享引用时，`drop` 将引用状态为共享引用数量减一。
4. `RefMut` `Drop` 当引用状态为独享引用时，`drop` 设置引用状态为未共享（未引用）。

**对 Ref / RefMut 实现 Deref 取得内部 T** 

1. `Deref` 基本上是能够实现自动对返回 `Ref` 内部的值也就是 `T` 的引用 `&T`。
2. `DerefMut` 类似但是返回 `&mut T` 。 `Deref` 只允许在 `RefMut` 上实现，而不能在 `Ref` 上实现，不然就会导致在 `Ref` 上存在多个可变引用。

```rust
#[derive(Clone, Copy)]
enum SharedState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<SharedState>,
}

// implied by UnsafeCell
// RefCell is !Sync

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(SharedState::Unshared),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        match self.state.get() {
            SharedState::Unshared => {
                self.state.set(SharedState::Shared(1));
                Ref { refcell: self }
            }
            SharedState::Shared(n) => {
                self.state.set(SharedState::Shared(n + 1));
                Ref { refcell: self }
            }
            SharedState::Exclusive => panic!("already mutably borrowed: BorrowError"),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        if let SharedState::Unshared = self.state.get() {
            self.state.set(SharedState::Exclusive);
            // SAFETY: no other references have been given out since state would be
            // Shared or Exclusive
            RefMut { refcell: self }
        } else {
            panic!("already borrowed: BorrowMutError")
        }
    }
}
```

**Ref**

声明 `Ref` 实现对 `RefCell` 的可变引用，需要对`Ref` 实现 `Deref` 实现对内部值的引用，也要实现 `Drop` 对引用状态进行修改。

```rust
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // a Ref is only created id no exclusive references have been given out.
        // once it is given out, state is set to Shared, so no exclusive referneces are given out
        // so dereferening into a shared reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            SharedState::Unshared | SharedState::Exclusive => unreachable!(),
            SharedState::Shared(1) => self.refcell.state.set(SharedState::Unshared),
            SharedState::Shared(n) => self.refcell.state.set(SharedState::Shared(n - 1)),
        }
    }
}
```

**RefMut**

声明 `RefMut` 实现对 `RefCell` 的可变引用，同样需要对 `RefMut` 实现 `Deref` 实现对内部值的共享引用，还要实现 `DerefMut` 实现对内部值的独享引用，也要实现 `Drop` 对引用状态进行修改。

```rust
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            SharedState::Unshared | SharedState::Shared(_) => unreachable!(),
            SharedState::Exclusive => self.refcell.state.set(SharedState::Unshared),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // see safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to Exclusive, so no further referneces are given out
        // so we have an exclusive lease on the inner value, so mutably dereferening is fine
        unsafe { &mut *self.refcell.value.get() }
    }
}
```

### Rc

虽然按照实践的演示完成了实践，但是有很多的部分并不理解。

内层 `RcInner` 的定义：存储具体数据和引用计数

```rust
struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}
```

Rc 的定义：存储 `RcInner` 的指针

```rust
pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}
```

Rc 实例化：

利用 `Box` 方法在堆上生成 `inner`， 同时利用 `into_raw` 保证指针在 `new` 结束之后的有效性，堆上的 `inner` 不会被销毁。

```rust
impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });
        Self {
            // SAFETY: Box doesn't give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}
```

对 `Rc` 实现 `Deref`，保证能实现对内部数据的引用：

```rust
impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc drop
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine
        &unsafe { self.inner.as_ref() }.value
    }
}
```

对 `Rc` 实现 `clone` ，每次 `clone` 将 `inner` 的引用计数加一：

```rust
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let n = inner.refcount.get();
        inner.refcount.set(n + 1);
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}
```

对 `Rc` 实现 `Drop` ，保证当存在更多引用时，将 `Rc` 引用计数减一，而当只有最后一个引用时，将 `Rc` 中的数据销毁。

```rust
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // not needed see: https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8?permalink_comment_id=3805900#gistcomment-3805900
            // should replace drop(inner); with let inner = ();
            drop(inner);
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc, and no reference to T/
            let _ = unsafe { self.inner.as_ref() };
        } else {
            // there are other Rc, so don't drop the box.
            inner.refcount.set(c - 1)
        }
    }
}
```

## **笔记**

### std::cell 和内部可变性

1. `cell`: Shareable mutable containers.
    1. rust中对于任意类型 T ，可以拥有多个不可变引用（immutable references） `&T`，或者一个可变（mutable reference）引用 `&mut T`，这是由 Rust 编译器进行限制的。
    2. 但是也在部分场景中这样的限制不够灵活，在这些场景中可能需要多个引用，但是又要是可变引用。`std::cell` 就是实现了这样的场景需求。
    3. 对于 `cell` 中的 `Cell` 和 `RefCell` ，它们有着严格的使用要求，并不是线程安全的，仅限于单线程中使用。
    4. `Cell` 和 `RefCell` 中的值可以通过共享引用（ `&T` ）进行修改，这就是内部可变性（ Interior Mutability ）。
    
    > Shareable mutable containers exist to permit mutability in a controlled manner, even in the presence of aliasing. Both `[Cell<T>](https://doc.rust-lang.org/std/cell/struct.Cell.html)` and `[RefCell<T>](https://doc.rust-lang.org/std/cell/struct.RefCell.html)` allow doing this in a single-threaded way. However, neither `Cell<T>` nor `RefCell<T>` are thread safe (they do not implement `[Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)`). If you need to do aliasing and mutation between multiple threads it is possible to use `[Mutex<T>](https://doc.rust-lang.org/std/sync/struct.Mutex.html)`, `[RwLock<T>](https://doc.rust-lang.org/std/sync/struct.RwLock.html)` or `[atomic](https://doc.rust-lang.org/std/sync/atomic/index.html)` types.
    [https://doc.rust-lang.org/std/cell/](https://doc.rust-lang.org/std/cell/)
    > 
2. [0:03:50](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=230s) Interior Mutability 内部可变性
    1. 主要有三个这种数据结构。
    2. `cell` 下的 `Cell` 和 `RefCell` [https://doc.rust-lang.org/std/cell/](https://doc.rust-lang.org/std/cell/)
    3. `sync` 下的 `Mutex` [https://doc.rust-lang.org/std/sync/struct.Mutex.html](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
    4. 不同的结构存在不同的限制。`Cell` `RefCell` `Mutex` 的使用限制逐渐减少，但是开销逐渐增加。
    5. `Box` 不提供内部可变性

### [0:07:47](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=467s) 实现 Cell

实现 `Cell` ，`Cell` 通过将值取出、放入实现内部可变性。

**主要方法：**

1. `pub fn get(&self) -> T where T: Copy {}`
2. `pub fn set(&self, value: T) {}` 

**安全限制：**

1. 限制1：查看 Cell 的所有方法，可以发现不存在任何的方法返回引用，例如 get 是直接返回 T 而不是 `&T` ，这就保证 `Cell` 总是可变的，因为不存在其他的指向值的引用。
2. 限制2：因为 `Cell` 不实现 `Sync` ，也就是说无法将 `Cell` 的引用传给其他的线程。假设如果有两个线程同时拥有一个 `Cell` 的引用（ `&Cell` ），那么这两个线程就有可能同时修改内部的值。
3. 这两个限制保证了安全，保证了在每一刻只能有一个修改。

**使用场景：**

1. `Cell` 对于在一些情况，需要能够在很多位置使用可以修改引用，同时在这个场景中能保证每个时刻最多只有一个位置能对引用内容进行修改，例如单线程下的树、图的情况。
2. `Cell` 常用语存储小类型，这个类型需要能实现 `Copy` ，小类型保证了较小的开销。

**具体实现：**

1. 需要使用 `UnsafeCell` 来构建 `Cell`
2. 线程安全： `Cell` 需要实现 `!Sync`，但是因为 `UnsafeCell` 就是 `!Sync` 所以`Cell` 就自动是 `!Sync`
3. 只允许 `get` 返回 `T` 而不是返回 `&T` ，保证即使在单线程的情况下也是安全的，因为不存在任何在外的引用，所以也不存在不安全的情况。

**测试错误实现下的 Cell 出现的问题（Trying to Test Cell）：[0:23:39](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=1419s)** 

1. 错误的实现
    1. 将 `get` 的返回值修改为 `&T`
    2. 声明 `Cell` 实现 `Sync`
2. 测试用例参考：**[cell-tests.rs](https://gist.github.com/jonhoo/7cfdfe581e5108b79c2a4e9fbde38de8#file-cell-tests-rs)** 

**推荐做法：在使用 unsafe 的时候，在使用位置和模块位置详细说明为什么这样做是安全的。** 

[0:40:17](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=2417s) UnsafeCell [https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html)

1. `UnsafeCell` 是 `cell` 的基础核心。 
2. `UnsafeCell` 是必须的，在 `Rust` 中除了使用用 `UnsafeCell` ，不允许将一个共享的引用转为一个独有的引用（~~利用 unsafe~~ ）。

### [0:41:21](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=2481s) 实现 RefCell

`RefCell` 通过记录已有引用类型和数量，实现在运行时动态检查借用 （ dynamic borrowing ），将内部值的引用传出，实现内部可变性。[https://doc.rust-lang.org/std/cell/struct.RefCell.html](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

**使用场景：**遍历树和图的时候很好用。

**具体实现和使用限制：**

相比于 `Cell` ，`RefCell` 需要额外记录引用情况。记录引用状态的 field 需要是 `Cell` ，需要利用 `Cell` 来保证对引用状态使用是线程安全的。存在3个状态：未引用（`Unshared`），共享引用计数（`Shared(usize)`），独享引用（`Exclusive`）。

**给出引用时：**

1. `borrow`：当引用状态是独享的，那么就不能再给出共享引用。
2. `borrow_mut`：当引用状态是共享的或者是独享的，那么就不能再给出独享引用，而共享引用计数应当加一。

**收回引用时：[0:54:21](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=3261s) RefCell Smart Pointer**

1. 如果 `borrow` 返回是 `&T` ，`borrow_mut` 返回是 `&mut T`，那么没有办法统计这些引用 `drop` 的情况，也就没办法对 `RefCell` 的引用状态进行统计。
2. 使用 `Ref` 和 `RefMut` 类型，包含对 `RefCell` 的引用。通过对 Ref 和 RefMut 实现 Drop 就能对 RefCell 的引用情况进行修改。
3. `Ref` `Drop` 当引用状态为存在一个共享引用时，`drop` 设置引用状态为未共享（未引用）。 当引用状态为存在多个共享引用时，`drop` 将引用状态为共享引用数量减一。
4. `RefMut` `Drop` 当引用状态为独享引用时，`drop` 设置引用状态为未共享（未引用）。

**对 Ref / RefMut 实现 Deref 取得内部 T** 

1. `Deref` 基本上是能够实现自动对返回 `Ref` 内部的值也就是 `T` 的引用 `&T`。
2. `DerefMut` 类似但是返回 `&mut T` 。 `Deref` 只允许在 `RefMut` 上实现，而不能在 `Ref` 上实现，不然就会导致在 `Ref` 上存在多个可变引用。

### [1:06:27](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=3987s) 实现 Rc

Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.

单线程的引用计数指针，“Rc” 是 “Reference Counted’” 的缩写。对 `Rc` 调用 `clone` ，会取得一个指向同样位置的指针，只有当最后一个 `Rc` 指针被摧毁，`Rc` 中存储的值才会被摧毁。 可以将 `Cell` 和 `RefCell` 放置到 `Rc` 中，实现 `Rc` 中存储值的可修改。 `Rc` 也不是线程安全的，但是即使在单线程中， `Rc` 也是有意义的，例如在图和树中。标准库中实现的 `Rc` 和演示中的实践并不相同。

Rc 在单线程的场景下使用较多，例如 GUI 中。

**具体实现：**

`Rc` 定义，对于每一个 `Rc` ， `clone` 并不直接复制内在数据，而是将引用计数加1。所以 `Rc` 需要一个内部的类型 `RcInner`，这个类型包含所存储的类型 `T` ，和引用计数，`Rc` 的 Field 应该是指向这个内在类型的指针。 `inner: *const RcInner<T>`

**实例化 `Rc`** ：因为 `Rc` 的成员是一个指针，那么就无法确定这个指针指向的内容是否还存在。在 `new` 中会利用 `Box` 实现在堆上生产 `RcInner` 。然后将 `RcInner` 转为指针放入 `Rc`，利用 `Box::into_raw(inner)` 不会将生成的 `RcInner` 销毁，而使用 `&*inner` 则会将 `inner` 销毁。

同样需要将引用计数放入 `Cell`  中，实现在共享引用中安全的引用计数修改。

**对 Rc 实现 Drop ：[1:23:49](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=5029s) NonNull**

1. 检查 `inner: *const RcInner<T>` 的引用计数是否为 1 ，如果是1，也就是所有的 Rc 都被舍弃或者正在舍弃，所以需要在 `drop` 中将 `inner: *const RcInner<T>` 指向的值进行销毁，利用 `let _ = Box::from_raw(self.inner);` 进行销毁。
2. 因为 `inner` 指针类型是 `*const` 所以是无法利用这个方法进行销毁的，需要将其类型设为 `*mut` 我自己实现的时候，第一次也是用 `*mut` 完成了销毁。但是 Jon 在视频中使用了 `NonNull` ，对于 `NonNull` 类型调用 as_ptr 就可以取得 `*mut` 了，而且因为在 `new` 中 `Box` 不可能给出空指针，所以这也是完全安全的。修改 `inner` 类型为 `NonNull<RcInner<T>>` 利用 `let _ = unsafe { Box::from_raw(self.inner.as_ptr) };` 进行销毁。

[**1:31:55](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=5515s) 使用 PhantomData ，确保 Rust 的 Drop Check 正常运行。**我并不太理解，对于正常的类型，如果 `T` 直接在 `Rc` 中，很可能 `T` 包含的数据，在 `Rc` 销毁前就被销毁了，但是在 `Rc` 的销毁过程中还是会检查 `T` 的可用性，这个时候 Rust 的 Deop Check 会发现错误。但是对于 `inner: NonNull<RcInner<T>>` 来说，因为 `T` 并不直接的在 `Rc` 中，所以 Rust 的 Drop Check 就无法正常运行。所以需要在 `Rc` 中增加一个 field ，`_marker: PhantomData<RcInner<T>>,` 保证 Drop Check 会对 T 的生命周期进行检查。参考：[https://doc.rust-lang.org/nomicon/dropck.html](https://doc.rust-lang.org/nomicon/dropck.html) 。

`Rc` 必须是 `!Send` ，因为计数器是线程不安全的，所以 `Rc` 必须是不允许在多线程中使用。

`*const` 无法成为独享引用，`*mut` 可以成为独享引用。`*mut` 不是独享的，和 `&mut` 是不同的。

[1:44:25](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=6265s) ?Sized Briefly [https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html](https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html)

### [1:47:30](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=6450s) 多线程下

1. `RwLock` 类似于 `RefCell` [https://doc.rust-lang.org/std/sync/struct.RwLock.html](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
2. `Mutex` 类似于简化的 `RefCell` [https://doc.rust-lang.org/std/sync/struct.Mutex.html](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
3. `Arc` 几乎和 `Rc` 一致 [https://doc.rust-lang.org/std/sync/struct.Arc.html](https://doc.rust-lang.org/std/sync/struct.Arc.html)
4. `FFI` 中几乎所有的代码默认都是 `unsafe` 的
5. 因为 `Arc` 比 `Rc` 开销更多，所以在能使用 Rc 的场景中要优先使用 `Rc` 

### [1:54:20](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=4&t=6860s) Copy-on-Write (Cow)

1. borrow [https://doc.rust-lang.org/std/borrow/index.html](https://doc.rust-lang.org/std/borrow/index.html)
2. Cow [https://doc.rust-lang.org/std/borrow/enum.Cow.html](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
3. 如果需要修改 `Cow` ，会自动将引用进行 `Copy` 。在面对大多数情况不需要修改数据，仅在少数情况中修改数据是， `Cow` 就会很有用，例如标准库中 String 的 [from_utf8_lossy](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy) 方法。