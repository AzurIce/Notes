## 0001 | 函数“重载”

```rust
struct Store {
    items: Vec<Item>
}

struct ItemIndex(pub usize);

impl Store {
    pub fn insert(&mut self, item: Item) -> ItemIndex {
        self.items.push(item);
        ItemIndex(self.items.len() - 1)
    }
}
```

比如有这样一个需求，有一个 `Store` 结构体用于存储一些 `Item`，我们希望插入 `Item` 后用 `ItemIndex` 返回其对应下标。



但是假如我们希望额外支持插入元组，即插入 `(Item,...,)` 返回 `(ItemIndex, ...,)` 呢？

因为在 Rust 中我们无法重载一个函数，所以我们有两个选择：

- 将 `insert` 方法提取到一个 Trait `InsertItem<T>`中，为 `Store` 实现不同 `T` 的 `InsertItem<T>`。
- 为可插入的类型编写一个 Trait `InsertableItem`，为 `insert` 添加泛型参数。

### 1. 提取方法到具有泛型参数的 Trait 中

```rust
pub trait Insert<T> {
    type Res;
    fn insert(&mut self, item: T) -> Self::Res
}
```

然后只要为 `Store` 分别实现下面的 Trait 即可：

- `Insert<Item, Res = ItemIndex>`
- `Insert<(Item,), Res = (ItemIndex,)>`
- `Insert<(Item,Item,), Res = (ItemIndex,ItemIndex,)>`
- ...

### 2. 为插入的类型编写一个 Trait，为方法添加泛型参数

```rust
impl Store {
    pub fn insert<T: Insertable>(&mut self, item: T) -> T::Res {
        item.insert_to(self)
    }
}

pub trait Insertable {
    type Res;
    fn insert_to(self, store: &mut Store) -> Self::Res;
}
```

然后只要分别为为 `Item`、`(Item,)`、... 实现 `Insertable` 即可。

### 比较

第一种方法必须将 Trait 引入才能够使用，且不利于文档，第二种更好一些。

两种方法都可以通过实现一个非 `pub` 的“基础实现” `insert_item` 来辅助 Trait 实现的编写：

```rust
impl Store {
    fn insert_item(&mut self, item: Item) -> ItemIndex {
        self.inner.push(item)
        ItemIndex(self.inner.len())
    }
}
```

```rust
impl Insert<Item> for Store {
    type Res = ItemIndex;
    fn insert(&mut self, item: Item) -> Self::Res {
        self.insert_item(item)
    }
}

impl Insert<(Item,)> for Store {
    type Res = (ItemIndex,);
    fn insert(&mut self, item: (Item,)) -> Self::Res {
        (self.insert_item(item),)
    }
}

impl Insert<(Item, Item,)> for Store {
    type Res = (ItemIndex, ItemIndex,);
    fn insert(&mut self, item: (Item, Item,)) -> Self::Res {
        (self.insert_item(item), self.insert_item(item),)
    }
}
```

```rust
impl Insertable for Item {
    type Res = ItemIndex;
    fn insert_to(self, store: &mut Store) -> Self::Res {
        store.insert_item(self)
    }
}

impl Insertable for (Item,) {
    type Res = (ItemIndex,);
    fn insert_to(self, store: &mut Store) -> Self::Res {
        (store.insert_item(self),)
    }
}

impl Insertable for (Item, Item,) {
    type Res = (ItemIndex, ItemIndex,);
    fn insert_to(self, store: &mut Store) -> Self::Res {
        (store.insert_item(self), store.insert_item(self),)
    }
}

// ...
```

### 再进一步

```rust
struct Store {
    // ...
}

struct ItemIndex(pub usize);

impl Store {
    fn insert_item<T: ItemTrait>(&mut self, item: T) -> ItemIndex {
        // ...
    }
}
```

假如再进一步，基础的支持类型不再是一个具体的 `Item` 而是任意实现了 `ItemTrait` 的类型：

```rust
impl<T: ItemTrait> Insertable for T {
    type Res = ItemIndex;
    fn insert_to(self, store: &mut Store) -> Self::Res {
        store.insert_item(self)
    }
}
```

但是假如我们想支持实现了 `IntoIterator<T>` 的类型，返回一个 Vec：

```rust
impl<E: ItemTrait, T: IntoIterator<Item = E>> Insertable for T {
    type Res = Vec<ItemIndex>;
    fn insert_to(self, store: &mut Store) -> Self::Res {
        self.into_iter().map(|item| store.insert_item(item)).collect()
    }
}
```

这时候往往又会遇到另一个问题：

```
error[E0119]: conflicting implementations of trait `Insertable`
```

这是由于 `T` 可能同时实现了 `ItemTrait` 和 `IntoIterator<Item = E>`。解决方法见 *0002 | Mark 类型在 Trait 泛型中的应用*。

## 0002 | Mark 类型在 Trait 泛型中的应用

对于一个目标 Trait `TraitTarget`，和另外两个 Trait `TraitA` 和 `TraitB`，假如我们想为 `T: TraitA` 和 `T: TraitB` 实现 `TraitTarget`，那么往往会遇到一个问题：

```rust
pub trait TraitTarget {}

impl<T: TraitA> TraitTarget {}
impl<T: TraitB> TraitTarget {}
```

```
error[E0119]: conflicting implementations of trait `TraitTarget`
```

这是因为类型 `T` 是完全有可能同时实现 `TraitA` 和 `TraitB` 的，此时这两个实现会冲突。

解决方法就是为 `TraitTarget` 引入一个 Mark 泛型参数，并使用一个 Mark 结构体来区分两个实现：

```rust
pub trait TraitTarget<Mark> {}

pub struct MarkA;
pub struct MarkB;

impl<T: TraitA> TraitTarget<MarkA> {}
impl<T: TraitB> TraitTarget<MarkB> {}
```

## | 0003 | 泛型 Trait 与 关联类型 Trait 的本质

泛型 Trait `TraitGeneric<T>` 可以为同一个类型实现不同的 `T`，而关联类型 Trait `TraitAssociated<AssociatedType = T>` 则只能实现一次。

// TODO

## 0004 | 为泛型 Trait 实现 Trait 中的泛型约束

如果我们想要为任意类型 `T: TraitRequire` 实现 Trait `TraitTarget`，可以这样写：

```rust
impl<T: TraitRequire> TraitTarget for T {}
```

但是，如果 `TraitRequire` 带有泛型参数呢？

```rust
impl<T: TraitRequire<E>, E> TraitTarget for T {}
```

这样写会无法编译通过：

```
error[E0207]: the type parameter `E` is not constrained by the impl trait, self type, or predicates
```

仔细思考一下，对于同一个 `T`，可能具有对不同的 `E` 的 `TraitRequire<E>` 的实现，是无法决定对 `T` 使用哪一个实现的。

因此要么为 `TraitTarget` 添加泛型参数来约束 `E`：

```rust
impl<T: TraitRequire<E>, E> TraitTarget<E> for T {}
```

要么，将 `TraitRequire` 的泛型参数 `E` 修改为关联类型：

```rust
impl<T: TraitRequire> TraitTarget<> for T {}
```

## 0005 | MutParts 模式（自己瞎想的）

常规的封装使用 `&T` 或 `&mut T` 为接收器：

```rust
pub struct Foo {
    pub field1: Foo1,
    pub field2: Foo2,
}

impl Foo {
    pub fn foo(&mut self) {
        self.field1.do_something();
        self.field2.do_something();
        // ...
    }
}
```

然而，实际上 `foo` 并不依赖于整个结构体，而是结构体内部的字段（所有的方法都是），也就是说即便 `Foo` 被大卸八块，字段被分散在各处，拿到全部字段的 `&` 或 `&mut` 则完全可以完成相同的操作。

具体的实现方法就是单独实现一个结构体 `FooMut<'a>`，为对全部字段的引用，然后将对应的方法修改为基于 `FooMut<'a>` 的：

```rust
pub struct FooMut<'a> {
    pub field1: &'a mut Foo1,
    pub field2: &'a mut Foo2,
}

impl FooMut {
    pub fn foo(&mut self) {
        self.field1.do_something();
        self.field2.do_something();
        // ...
    }
}
```

在这个基础之上，可以通过一个 `MutParts` Trait 来对不同的结构（都可以整合出全部 field 的引用）提供相同的接口：

```rust
pub trait MutParts<'a> {
    type Mut: 'a;
    fn mut_parts(&'a mut self) -> Self::Mut;
}

#[item]
pub struct Arrow {
    pub tip: VItem,
    pub line: VItem,
}

/* Can be generated through macros
pub struct ArrowRabject<'t> {
    pub tip: Rabject<'t, VItem>,
    pub line: Rabject<'t, VItem>,
}

pub struct ArrowMutParts<'r> {
    pub tip: &'r mut VItem,
    pub line: &'r mut VItem,
}

impl<'r> MutParts<'r> for Arrow {
    type Mut = ArrowMutParts<'r>;
    fn mut_parts(&'r mut self) -> Self::Mut {
        ArrowMutParts {
            tip: &mut self.tip,
            line: &mut self.line,
        }
    }
}

impl<'r, 't: 'r> MutParts<'r> for ArrowRabject<'t> {
    type Mut = ArrowMutParts<'r>;
    fn mut_parts(&'r mut self) -> Self::Mut {
        ArrowMutParts {
            tip: &mut self.tip.data,
            line: &mut self.line.data,
        }
    }
}
*/

pub trait ArrowMethods<'a>: MutParts<'a, Mut = ArrowMutParts<'a>> {
    fn set_tip(&'a mut self, tip: VItem);
    fn set_line(&'a mut self, line: VItem);
}

impl<'a, T: MutParts<'a, Mut = ArrowMutParts<'a>>> ArrowMethods<'a> for T{
    fn set_tip(&'a mut self, tip: VItem) {
        *self.mut_parts().tip = tip;
    }

    fn set_line(&'a mut self, line: VItem) {
        *self.mut_parts().line = line;
    }
}

fn foo() {
    let timeline = RanimTimeline::new();

    let mut arrow = Arrow {
        tip: VItem::empty(),
        line: VItem::empty(),
    };

    arrow.set_tip(VItem::empty());

    let mut arrow_rabject = ArrowRabject {
        tip: Rabject {
            timeline: &timeline,
            id: 0,
            data: arrow.tip,
        },
        line: Rabject {
            timeline: &timeline,
            id: 1,
            data: arrow.line,
        },
    };

    arrow_rabject.set_tip(VItem::empty());
}
```

## 0006 | 何时使用 inline？

> https://matklad.github.io/2021/07/09/inline-in-rust.html

通过 `#[inline]` 标记的函数会在被调用的时候将调用替换为函数体内容。

比如对于下面的例子：

```rust
#[inline]
fn inline_mul(x: u32, y: u32) -> u32 {
    x * y
}

fn square(x: u32) -> u32 {
    inline_mul(x, x)
}
```

`square` 会被转换为：

```rust
fn square(x: u32) -> u32 {
    x * x
}
```

优点在于：

- 无函数调用产生的 overhead
- 被调用者和调用者会被在一起优化

对于 Rust 来说，编译器拥有对同一个 Crate 中的函数的完全访问，但是对于外部的 Crate，由于只具有其签名信息，不具有函数体信息，无法 inline。而通过 `#[inline]` 则可以实现跨 Crate 的内联，但是会显著提高编译时间。

此外：

- 泛型函数是隐式内联的
- LTO（Link-Time Optimization）也可以实现类似内联的效果，但是会使编译时间更慢。

## 0007 | Move 原理与 Copy

https://stackoverflow.com/questions/30288782/what-are-move-semantics-in-rust/30290070#30290070

https://users.rust-lang.org/t/how-move-works-in-rust/116776

Copy 和 Move 所做的事情都是 `memcpy` `size_of::<T>()` 大小的数据（即浅拷贝），唯一的区别在于 Move 会将原来的数据标记为失效。

也就是说，**Copy 的性能损耗与 Move 完全一致**。

不过对于绝大多数类型，其性能损耗都是极小的。对于部分较大的类型，如 `[T;N]`，可能会导致性能损耗是非平凡的，同时也可能导致栈溢出。

## 参考

https://nihil.cc