## Deref 与 DerefMut

### 解引用转换

如果 `T` 实现了 `Deref<Target = U>`，对于一个 `T` 类型的 `v` 来说：

- 在不可变的上下文中，`*v` 等价于 `*Deref::deref(&v)`

- `&T` 类型可被视为为 `&U`
- `T` 隐式地实现了所有 `U` 带有 `&self` 接收器的方法

## `#[no_mangle]`

> https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-rust-functions-from-other-languages
>
> https://doc.rust-lang.org/stable/reference/abi.html?highlight=mangle#the-no_mangle-attribute

编译器往往需要对程序中的实体名称进行修饰以避免重名，或包含更多信息。

比如对于下面这个 C++ 的例子：

```cpp
int  f () { return 1; }
int  f (int)  { return 0; }
void g () { int i = f(), j = f(0); }
```

编译器处理后得到的结果是这样的：

```cpp
int  __f_v () { return 1; }
int  __f_i (int)  { return 0; } 
void __g_v () { int i = __f_v(), j = __f_i(0); }
```

对于动态加载库来说，这样并不利于调用库中的实体，因此 Rust 提供了 `#[no_mangle]` 属性，可以关闭一个函数的 mangling。

## Box

### Box::leak

消耗一个 Box 的所有权，返回其内容的 `'static` 引用。

Box 拥有对一块内存的所有权，当 Box 被 Drop 时，其拥有的内存也会被回收。

但是通过 `Box::leak` 可以解开对内存的所有权，不再管理那块内存（即让这块内存泄漏）。

也就是将那块内存的回收任务交给程序结束后的操作系统（操作系统会在进程结束后回收其所有资源）

### Box::into_raw 和 Box::from_raw

`Box::into_raw` 同样消耗一个 Box 的所有权，但是返回的是对应内存的指针。

`Box::from_raw` 可以从一个指针重新获取对应内存的所有权。这个函数是 unsafe 的，因为对同一个指针调用两次有可能导致 use-after-free 和 double-free（即有两个 Box 对同一块内存具有所有权）

## atomic 模块

目前与 c++20 的规则一致，没有 comsume 内存序。

### 原子操作

即不可再分的操作：

- 其他线程要么看到开始前的状态，要么看见结束后的状态，不存在中间状态
- 在底层，对应着特殊的硬件指令
- 是一个一般的概念，并不局限于硬件指令（比如数据库事务）

例如对于自增操作，是一个“读-修改-写”操作（非原子）：

- 从内存中读取 `x`
- 向 `x` 加 `1`
- 写入新的 `x` 到内存

如果两个线程同时对同一个变量 `x` 进行自增，取决于这些指令执行的顺序会导致不同的结果。

甚至“读”或“写”也不保证是原子的，因为对于一些数据类型可能对应的读写操作需要用多个指令来完成。

**原子类型** 即通过特殊硬件指令实现保证其操作满足 **原子性** 的类型，比如对于 `AtomicUsize` 有 `load`，`store`，`fetch_add` 等可用的 **原子操作**。

### 内存序

在执行 **原子操作** 时，可以指定使用的 **内存序**。**内存序** 指定了使用何种方式访问内存，指导了 CPU 和编译器对指令进行重排：

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);

    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::Release);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::Acquire) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {panic:?}");
    }
}
```

一共有五种内存序：

```rust
#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

- `Relaxed`：不进行任何限制，只保证原子操作。

- `Release` 和 `Acquire`：通常配对使用

    `Release` 保证一切在其之前的访问不会被重排到其后面；

    `Acquire` 保证一切在其之后的访问不会被重排到其前面。

    当线程 A 对内存中的一个位置进行了 `Release`，而线程 B 对内存中 **相同的位置** 进行了 `Acquire` 时，就建立起了因果关系：

    - 一切 A 的 `Release` 前的操作（包括非原子操作以及 `Relaxed` 的原子操作）都会在 B 的 `Acquire` 之后被观测到。

    其实也就相当于建立了一个临界区。

    `Release` 只能应用于包含 store 操作的操作，且只限制 store 操作，load 为 `Relax`；

    `Acquire` 只能应用于包含 load 操作的操作，且只限制 load 操作，store 为 `Relax`。

- `AcqRel`：`Release` 和 `Acquire` 的结合，同时对 store 和 load 进行限制

- `SeqCst`：在 `AcqRel` 的基础上保证全部线程观测到的顺序一致操作顺序一致。

## Sized 与 Dynamically Sized Types

Sized 是一个标记 Trait，标识某一个类型在编译时具有已知的大小。

**默认，类型参数（除了 trait 中的`Self`）都是 `Sized` 的，可以使用 `?Sized` 约束来放宽限制。**

对应的，编译期未知大小，运行时才知道大小的类型就是 Dynamically Sized Types（DST），主要有两类：

- Trait Object：`dyn MyTrait`
- Slice：`[T]`，`str`，等

DST 只能存在于指针（引用/Raw指针/智能指针）背后，因为指针的大小是 `Sized` 的。

不过到 DST 的指针通常会具有更大（两倍）的大小，比如到 Slice 的指针额外保存了其元素数量，到 Trait Object 的指针额外保存了到 VTable 的指针。



- 可以为 DST 实现 Trait
- 结构体也可以在最后一个字段包含一个 DST，结构体本身也会变成一个 DST。

## 手写虚函数表

在 `std::task` 中，虽然有一个 `Wake` Trait 可以用于创建 `Waker`，但是 `Waker` 的内部是手写的（数据，虚函数表）胖指针 `RawWaker`，而非一个 Trait Object。

原因包括需要为 `no_std` 提供支持等。

`Waker` 对 `From<Arc<W>> where W: Wake + Send + Sync + 'static` 的实现挺有意思，内部是这样的：

```rust
// NB: This private function for constructing a RawWaker is used, rather than
// inlining this into the `From<Arc<W>> for RawWaker` impl, to ensure that
// the safety of `From<Arc<W>> for Waker` does not depend on the correct
// trait dispatch - instead both impls call this function directly and
// explicitly.
#[cfg(target_has_atomic = "ptr")]
#[inline(always)]
fn raw_waker<W: Wake + Send + Sync + 'static>(waker: Arc<W>) -> RawWaker {
    // Increment the reference count of the arc to clone it.
    //
    // The #[inline(always)] is to ensure that raw_waker and clone_waker are
    // always generated in the same code generation unit as one another, and
    // therefore that the structurally identical const-promoted RawWakerVTable
    // within both functions is deduplicated at LLVM IR code generation time.
    // This allows optimizing Waker::will_wake to a single pointer comparison of
    // the vtable pointers, rather than comparing all four function pointers
    // within the vtables.
    #[inline(always)]
    unsafe fn clone_waker<W: Wake + Send + Sync + 'static>(waker: *const ()) -> RawWaker {
        unsafe { Arc::increment_strong_count(waker as *const W) };
        RawWaker::new(
            waker,
            &RawWakerVTable::new(clone_waker::<W>, wake::<W>, wake_by_ref::<W>, drop_waker::<W>),
        )
    }

    // Wake by value, moving the Arc into the Wake::wake function
    unsafe fn wake<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        let waker = unsafe { Arc::from_raw(waker as *const W) };
        <W as Wake>::wake(waker);
    }

    // Wake by reference, wrap the waker in ManuallyDrop to avoid dropping it
    unsafe fn wake_by_ref<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        let waker = unsafe { ManuallyDrop::new(Arc::from_raw(waker as *const W)) };
        <W as Wake>::wake_by_ref(&waker);
    }

    // Decrement the reference count of the Arc on drop
    unsafe fn drop_waker<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        unsafe { Arc::decrement_strong_count(waker as *const W) };
    }

    RawWaker::new(
        Arc::into_raw(waker) as *const (),
        &RawWakerVTable::new(clone_waker::<W>, wake::<W>, wake_by_ref::<W>, drop_waker::<W>),
    )
}
```

对于一个给定的 `wake` 来说，其 `RawWakerVTable` 是固定的，然而由于 `clone_waker` 和 `raw_waker` 中都需要包含这个函数表，会导致这个相同的函数表在编译结果中包含两次，这里通过两个 `inline` 来让 `raw_waker` 和 `clone_waker` 编译在一个代码单元中，这样 LLVM 可以进行优化。

## `From` 与 `Into`

父 Trait 为 `Sized`，应当实现 `From` 而非 `Into`，因为有类似下面的自动实现：

```rust
impl<T: From<F>, F> Into<T> for F
```

`From` 和 `Into` 均具有自反性，即 `T` 实现了 `From<T>` 和 `Into<T>`。

