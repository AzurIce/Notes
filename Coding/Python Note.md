## Collections

### defaultdict

继承自 `dict`，定义如下：

```
class collections.defaultdict(default_factory=None, /[, ...])
```

在第一次访问某一个 `key` 时，用 `default_factory` 初始化其 `value`，不需手动判断是否存在再初始化：

```python
s = [('yellow', 1), ('blue', 2), ('yellow', 3), ('blue', 4), ('red', 1)]
d = defaultdict(list)
for k, v in s:
    d[k].append(v) # value 被使用 list() 初始化

sorted(d.items()) # [('blue', [2, 4]), ('red', [1]), ('yellow', [1, 3])]
```

## 重载

不支持重载，后定义的会覆盖先定义的。

## @classmethod 与 @staticmethod

> https://stackoverflow.com/questions/12179271/meaning-of-classmethod-and-staticmethod-for-beginner

```python
class Date(object):
    
    def __init__(self, day=0, month=0, year=0):
        self.day = day
        self.month = month
        self.year = year
    #...
```

### @classmethod

第一个参数为类本身，而非类的实例，不会被继承。

```python
class Date(object):
    # ...
	@classmethod
    def from_string(cls, date_as_string):
        day, month, year = map(int, date_as_string.split('-'))
        date = Date(day, month, year)
        return date

date = Date.from_string('11-09-2012')
```

### @staticmethod

与类和实例均无关

```python
class Date(obejct):
    # ...
    @staticmethod
    def is_date_valid(date_as_string):
        day, month, year = map(int, date_as_string.split('-'))
        return day <= 31 and month <= 12 and year <= 3999

is_date = Date.is_date_valid('11-09-2012')
```

## @property

将某一个方法作为属性，常用于 getter 类的方法

## 一些 \__xxx__ 变量

对于每一个 Python 模块，都有如下变量：

- `__name__`：模块名

    在使用 `python xxx.py` 运行时，会把 `__name__` 赋值为 `__main__`

- `__all__`：用于在 `__init__.py` 中定义 `import *` 时导入的内容

## type()

type 是一个类，而不是函数！！！

- 【常用】`class type(object)`：得到的结果与 `object.__class__` 相同。

- `class type(name, bases, dict, **kwds)`：相当于 `class` 语句的一种动态形式，

    `name` 为类名，会成为 `__name__` 属性

    `bases` 为基类，会成为 `__bases__` 属性（如果为空则为 `object`）

    `dict` 为属性和方法定义，会成为 `__dict__`（可能经过拷贝/包装）

    **例子**：下面两条语句创建的 type 对象相同

    ```python
    class X:
        a = 1
    
    X = type('X', (), dict(a=1))
```

### 类型标注

此外，在类型标注中可以使用 `type[C]` 来表示一个 `C` 的类对象，即：

```python
a = 3         # 为 ``int`` 类型
b = int       # 为 ``type[int]`` 类型
c = type(a)   # 同样为 ``type[int]`` 类型
```

`type[Any]` 等价于 `type`

## 元类 metaclass

默认情况下，类是使用 `type()` 来创建的。

可以通过传入 `metaclass` 关键字参数或继承一个包含此参数的现有类进行定制：

```python
# MyClass 和 MySubclass 都是 Meta 的实例
class Meta(type):
    pass

class MyClass(metaclass=Meta):
    pass

class MySubclass(MyClass):
    pass
```

## 一些 \__xxx__ 方法

- `__get__(self, instance, owner=None)` 方法：用来访问属性

    这里看似 `owner` 可选，但是实际上 python 自己总会传入两个参数。

    `instance` 为被用来访问属性的实例，如果为空则通过 `owner` 访问。

    也就是说，`owner` 为类，

    当通过某一个实例访问属性时，`instance` 为对应的类实例

    当通过某一个类访问属性时，`instance` 为 `None`

