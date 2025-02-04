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

