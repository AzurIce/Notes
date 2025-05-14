# Deref 与 DerefMut

> [Deref in std::ops - Rust (rust-lang.org)](https://doc.rust-lang.org/std/ops/trait.Deref.html)

```rust
pub trait Deref {
    type Target: ?Sized;

    // Required method
    fn deref(&self) -> &Self::Target;
}
```

用于 **显式** 的不可变解引用操作，如 `*v`。

除此之外，还有一些额外的 **隐式** 的用法，被称作 *Deref coercion*，对于实现了 `Deref<Target = U>` 的 `T`，假设 `v` 是 `T` 类型的，则有：

- `*v` 等价于 `*Deref::deref(&v)`
- `&T` 被转换为 `&U`（`DerefMut` 则是对应的可变引用，当然 `&mut T` 可以作为 `&T`）
- `T` 隐式地实现 `U` 中使用 `&self` 的全部方法

## 标准库中的使用

比如 `Box<T>` 就实现了 `Deref<Target = T>` 以及 `DerefMut<Target = T>`，因此对与一个 内容是 `v` 的 `Box<T>` 类型的 `u`：

- `*u` 可以当作 `*(&v)` 和 `*(&mut v)`
- `&u` 可以当作 `&v`，`&mut u` 可以当作 `&mut v`。

