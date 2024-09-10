# Deref

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
- `&T` 被转换为 `&U`
- `T` 隐式地实现 `U` 中使用 `&self` 的全部方法