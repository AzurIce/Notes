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

## 参考

https://nihil.cc