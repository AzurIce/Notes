想要做一个系列视频来讲解 Git 的原理，使用 manim。

---

一个 Git仓库 中的 `.git/` 目录下一般有四个关键的文件/目录：

- `HEAD` 文件：用于指向当前被检出的分支
- `index` 文件（刚初始化的时候并没有）：用于保存暂存区信息
- `objects` 目录：存储所有的 Git Objects
- `refs` 目录：存储指向数据的提交对象的指针

## 一、Git Objects

Git 是一个内容寻址（content-addressable）文件系统，其核心其实是一个简单的 **键值对数据库**。
因此，你可以向 Git 仓库插入任何类型的内容，对应的 Git 会借助哈希返回一个唯一的键，通过键可以在任意时刻再次取回内容。

如果你随便找一个仓库，逛一逛它的 `.git/objects` 目录，可以发现里面有很多两个字符的命名的文件夹，在其内有些 38 个字符命名的文件。将它们连接在一起可以得到一个40个字符的完整的 SHA-1 值，这便是数据的 **键**，而数据本身的 **值** 就被存储在文件中。

为了存储不同的信息，Git 对象主要有三种：`blob`、`tree` 和 `commit`

- `blob` 对象：存储文件的二进制内容数据
- `tree` 对象：存储目录结构
- `commit` 对象：存储提交信息

下面将依次进行讲解。

### 1. blob 对象

`blob` 对象用于存储文件内容数据。

Git 的一个底层命令 `git hash-object` 可以计算并返回传入的数据，也可以将其写入 `.gits/objects` 目录（Git Objects 数据库），下面我们将使用这个命令来进行一些尝试。

---

首先初始化一个新的仓库：

```console
$ mkdir git-playground
$ cd git-playground
$ git init
Initialized empty Git repository in /mnt/d/_Dev/git-playground/.git/
```

在仓库刚被创建的时候 `.git/objects` 目录会被初始化，其中有两个子目录 `info` 和 `pack`，不过目前 `.git/objcets` 目录中没有任何一个文件：

```console
$ find .git/objects
.git/objects
.git/objects/info
.git/objects/pack
$ find .git/objects -type f
```

---

使用 `git hash-object` 创建一个新的数据对象并使用 `-w` 指示 Git 将其存储到数据库中：

> `-w` 表示不仅仅计算并显示哈希，还写入 Git 数据库，`--stdin` 表示从标准输入读取数据而非指定文件。

```console
$ echo 'test content' | git hash-object -w --stdin
d670460b4b4aece5915caf5c68d12f560a9fe3e4
```

它返回了一个 40 个字符长度的字符串，这是数据 `test content` 的 SHA-1 哈希值。

现在再查看一下 `.git/objects` 中的内容：

```console
$ find .git/objects -type f
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
```

Git 将上面的 SHA-1 哈希值的前 2 个字符作为子目录名，后面 38个字符作为文件名将数据存储为文件。

---

下面介绍另一个命令 `git cat-file`，这个命令可以用来很方便地查看 Git Objects 的内容：

> `-p` 表示识别内容类型并以合适的方式显示

```console
$ git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
test content
```

---

下面创建一个新文件并将其写入数据库：

```console
$ echo 'version 1' > test.txt
$ git hash-object -w test.txt
83baae61804e65cc73a7201a7252750c76066a30
```

然后修改其内容，再写入数据库：

```console
$ echo 'version 2' > test.txt
$ git hash-object -w test.txt
1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
```

现在 `.git/objects` 中就会包含三个文件，分别存储了先前的字符串以及 `test.txt` 的两个版本：

```console
$ find .git/objects -type f
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
```

现在就算我们将 `test.txt` 删除，也可以通过唯一的键值获取到对应版本的内容：

```console
$ git cat-file -p 83baae61804e65cc73a7201a7252750c76066a30 > test.txt
$ cat test.txt
version 1
```

```console
$ git cat-file -p 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a > test.txt
$ cat test.txt
version 2
```

这就是 `blob` 对象：

> `-t` 表示显示对象的类型

```console
$ git cat-file -t 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
blob
```

---

不过你其实可以注意到，`blob` 对象只能够存储文件的数据内容，而不能存储目录结构以及文件名等信息。

### 2. tree 对象

Tree 对象用于存储存储目录结构（文件路径、文件名等）。
快照其实就是存储根目录信息的 tree 对象。

---

这里先以一个假设的仓库为例解释一下 tree 对象的概念：

假设有一个仓库，其最新的 `tree` 如下：

```console
$ git cat-file -p master^{tree}
100644 blob a906cb2a4a904a152e80877d4088654daad0c859      README
100644 blob 8f94139338f9404f26296befa88755fc2598c289      Rakefile
040000 tree 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0      lib
```

> `master^{tree}` 指定了 `master` 分支最新的提交所指向的 tree 对象。

可以看到 tree 对象的内容包含一系列 Git 对象的关联模式、类型、哈希值以及文件名。
这与 Unix 的文件系统很相似，不过是经过简化的。

如果进一步查看 `lib` 对象的内容可以得到：

```console
$ git cat-file -p 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0
100644 blob 47c6340d6459e05787f644c2447d2595f5d3a54b      simplegit.rb
```

其结构可以用下面这张图来表示：

![Simple version of the Git data model](./assets/data-model-1.png)

---

接下来进行一些尝试：

Git 创建 tree 时会使用 暂存区 或 索引 的状态来创建，所以我们要想创建一个 tree 对象，也需要通过暂存一些文件来创建索引。

以一个单入口 `test.txt` 文件为例：

```console
$ git update-index --add --cacheinfo \
  100644 83baae61804e65cc73a7201a7252750c76066a30 test.txt
```

通过 `git update-index` 命令来更新索引，使用 `--add` 是因为 `test.txt` 目前并不在暂存区内（甚至暂存区都还未创建），使用 `--cacheinfo` 是因为 `test.txt` 目前不在目录中而是在数据库中。
之后指定模式、哈希值、文件名。

`100644` 表示是一个普通文件，其他更多的模式比如 `100755` 表示可执行文件，`120000` 表示一个符号链接。

现在索引创建完毕，可以使用 `git write-tree` 来将暂存区写入到 tree 对象中并保存进数据库。

```console
$ git write-tree
d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git cat-file -p d8329fc1cc938780ffdd9f94e0d364e0ea74f579
100644 blob 83baae61804e65cc73a7201a7252750c76066a30      test.txt
```

接下来再创建一个由第二个版本的 `test.txt` 以及一个新文件 `new.txt` 组成的 tree 对象：

```console
$ echo 'new file' > new.txt
$ git update-index --cacheinfo 100644 \
  1f7a7a472abf3dd9643fd615f6da379c4acb3e3a test.txt
$ git update-index --add new.txt
```

```console
$ git write-tree
0155eb4229851634a0f03eb265b69f5a2d56f341
$ git cat-file -p 0155eb4229851634a0f03eb265b69f5a2d56f341
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt
```

接下来可以通过 `git read-tree` 来读取 tree 对象的内容并放到暂存区内，我们取出第一个 tree 的内容置于 `bak` 目录（使用 `--prefix` 可以指定存储 tree 对象的目录）然后再创建一个 tree 对象：

```console
$ git read-tree --prefix=bak d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git write-tree
3c4e9cd789d88d8d89c1073707c3585e41b0e614
$ git cat-file -p 3c4e9cd789d88d8d89c1073707c3585e41b0e614
040000 tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579      bak
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt
```

现在整个仓库的状态可以用下图表示：

![The content structure of your current Git data](./assets/data-model-2.png)

这便是 tree 对象。

### 3. commit 对象

到目前为止，`blob` 和 `tree` 对象虽然可以存储所有文件及目录的信息，但是仍旧没有保存下来有关谁在何时为何保存了快照的信息，而这些信息就由 commit 对象保存。

可以通过 `git commit-tree` 并指定一个 tree 对象来创建 commit 对象：

```console
$ echo 'First commit' | git commit-tree d8329f
fdf4fc3344e67ab068f836878b6c4951e3b15f3d
```

```console
$ git cat-file -p fdf4fc3
tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579
author Scott Chacon <schacon@gmail.com> 1243040974 -0700
committer Scott Chacon <schacon@gmail.com> 1243040974 -0700

First commit
```

一个 commit 对象包含以下内容：

- 用于表示当前快照的顶级的 tree 对象
- 前一个 commit 对象（如果有）
- 作者和提交者的相关信息（用户名称以及邮箱还有时间戳）
- 提交信息

下面再创建两个 commit 对象，并使用 `-p` 来指定前一个提交：

```console
$ echo 'Second commit' | git commit-tree 0155eb -p fdf4fc3
cac0cab538b970a37ea1e769cbbde608743bc96d
$ echo 'Third commit'  | git commit-tree 3c4e9c -p cac0cab
1a410efbd13591db07496601ebc7a059dd55cfe9
```

其实目前，我们几乎通过手动操作得到了一个实际的 Git 仓库，可以使用 `git log` 来查看历史记录：

```console
$ git log --stat 1a410e
commit 1a410efbd13591db07496601ebc7a059dd55cfe9
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:15:24 2009 -0700

	Third commit

 bak/test.txt | 1 +
 1 file changed, 1 insertion(+)

commit cac0cab538b970a37ea1e769cbbde608743bc96d
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:14:29 2009 -0700

	Second commit

 new.txt  | 1 +
 test.txt | 2 +-
 2 files changed, 2 insertions(+), 1 deletion(-)

commit fdf4fc3344e67ab068f836878b6c4951e3b15f3d
Author: Scott Chacon <schacon@gmail.com>
Date:   Fri May 22 18:09:34 2009 -0700

    First commit

 test.txt | 1 +
 1 file changed, 1 insertion(+)
```

现在再查看一下 `.git/objects`（注释表示存储的内容）：

```console
$ find .git/objects -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341 # tree 2
.git/objects/1a/410efbd13591db07496601ebc7a059dd55cfe9 # commit 3
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a # test.txt v2
.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614 # tree 3
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30 # test.txt v1
.git/objects/ca/c0cab538b970a37ea1e769cbbde608743bc96d # commit 2
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4 # 'test content'
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579 # tree 1
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92 # new.txt
.git/objects/fd/f4fc3344e67ab068f836878b6c4951e3b15f3d # commit 1
```

整个仓库的内容可以表示为下图：

![All the reachable objects in your Git directory](./assets/data-model-3.png)

## 二、Git References

到目前为止，我们从 Git仓库 取东西都需要一个对应对象的哈希值，**Git引用** 就是一个特殊的文件，通过保存不同的哈希值来动态地指向不同的 Git对象，他们被存储在 `.git/refs` 目录下。

对于我们刚才手动创建的“仓库”，目前并没有任何引用：

```console
$ find .git/refs
.git/refs
.git/refs/heads
.git/refs/tags
$ find .git/refs -type f
```

若要创建一个新引用来帮助记忆最新提交所在的位置，从技术上讲我们只需简单地做如下操作：

```console
$ echo 1a410efbd13591db07496601ebc7a059dd55cfe9 > .git/refs/heads/master
```

现在，你就可以在 Git 命令中使用这个刚创建的新引用来代替 SHA-1 值了：

```console
$ git log --pretty=oneline master
1a410efbd13591db07496601ebc7a059dd55cfe9 third commit
cac0cab538b970a37ea1e769cbbde608743bc96d second commit
fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit
```

不过并不建议直接手动修改文件， 如果想更新某个引用，Git 提供了一个更加安全的命令 `update-ref` 来完成此事：

```console
$ git update-ref refs/heads/master 1a410efbd13591db07496601ebc7a059dd55cfe9
```

这基本就是 Git 分支的本质：一个指向某一系列提交之首的指针或引用。 若想在第二个提交上创建一个分支，可以这么做：

```console
$ git update-ref refs/heads/test cac0ca
```

这个分支将只包含从第二个提交开始往前追溯的记录：

```console
$ git log --pretty=oneline test
cac0cab538b970a37ea1e769cbbde608743bc96d second commit
fdf4fc3344e67ab068f836878b6c4951e3b15f3d first commit
```

现在，仓库看起来会像是这样：

![Git directory objects with branch head references included](./assets/data-model-4.png)

### 1. HEAD 引用

HEAD 文件通常是一个符号引用（symbolic reference），指向目前所在的分支。 所谓符号引用，表示它是一个指向其他引用的指针。

然而在某些罕见的情况下，HEAD 文件可能会包含一个 git 对象的 SHA-1 值。 当你在检出一个标签、提交或远程分支，让你的仓库变成 [“分离 HEAD”](https://git-scm.com/docs/git-checkout#_detached_head)状态时，就会出现这种情况。

如果查看 HEAD 文件的内容，通常我们看到类似这样的内容：

```console
$ cat .git/HEAD
ref: refs/heads/master
```

如果执行 `git checkout test`，Git 会像这样更新 HEAD 文件：

```console
$ cat .git/HEAD
ref: refs/heads/test
```

当我们执行 `git commit` 时，该命令会创建一个提交对象，并用 HEAD 文件中那个引用所指向的 SHA-1 值设置其父提交字段。

你也可以手动编辑该文件，然而同样存在一个更安全的命令来完成此事：`git symbolic-ref`。 可以借助此命令来查看 HEAD 引用对应的值：

```console
$ git symbolic-ref HEAD
refs/heads/master
```

同样可以设置 HEAD 引用的值：

```console
$ git symbolic-ref HEAD refs/heads/test
$ cat .git/HEAD
ref: refs/heads/test
```

不能把符号引用设置为一个不符合引用规范的值：

```console
$ git symbolic-ref HEAD test
fatal: Refusing to point HEAD outside of refs/
```

### 2. Tags 引用

前面我们刚讨论过 Git 的三种主要的对象类型（**数据对象**、**树对象** 和 **提交对象** ），然而实际上还有第四种。 **标签对象（tag object）** 非常类似于一个提交对象——它包含一个标签创建者信息、一个日期、一段注释信息，以及一个指针。 主要的区别在于，标签对象通常指向一个提交对象，而不是一个树对象。 它像是一个永不移动的分支引用——永远指向同一个提交对象，只不过给这个提交对象加上一个更友好的名字罢了。

咕

// TODO: Remotes

## 三、Packfiles

假如有一个 22K 的 `repo.rb`，它在被修改一点点后会被存储为一个崭新的对象。但是 Git 并不会真的占用 44K 的空间来存储它们。

Git 最开始存储对象的格式叫做 "loose" 对象格式，在一些情况 Git 会将一些这样的对象打包为一个二进制文件（被称作 packfile）来节省空间。

可以通过手动执行 `git gc` 来让 Git 打包对象（在 push 的时候也会看到这个）：

```
$ git gc
Counting objects: 18, done.
Delta compression using up to 8 threads.
Compressing objects: 100% (14/14), done.
Writing objects: 100% (18/18), done.
Total 18 (delta 3), reused 0 (delta 0)
```

这时候再去查看 `objects` 目录就会发现，很多对象都消失了，然后多了 `.idx` 和 `.pack` 两个文件：

```
$ find .git/objects -type f
.git/objects/bd/9dbf5aae1a3862dd1526723246b20206e5fc37
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4
.git/objects/info/packs
.git/objects/pack/pack-978e03944f5c581011e6998cd0e9e30000905586.idx
.git/objects/pack/pack-978e03944f5c581011e6998cd0e9e30000905586.pack
```

通过 `git verify-pack` 可以查看有什么被打包了：

```
$ git verify-pack -v .git/objects/pack/pack-978e03944f5c581011e6998cd0e9e30000905586.idx
2431da676938450a4d72e260db3bf7b0f587bbc1 commit 223 155 12
69bcdaff5328278ab1c0812ce0e07fa7d26a96d7 commit 214 152 167
80d02664cb23ed55b226516648c7ad5d0a3deb90 commit 214 145 319
43168a18b7613d1281e5560855a83eb8fde3d687 commit 213 146 464
092917823486a802e94d727c820a9024e14a1fc2 commit 214 146 610
702470739ce72005e2edff522fde85d52a65df9b commit 165 118 756
d368d0ac0678cbe6cce505be58126d3526706e54 tag    130 122 874
fe879577cb8cffcdf25441725141e310dd7d239b tree   136 136 996
d8329fc1cc938780ffdd9f94e0d364e0ea74f579 tree   36 46 1132
deef2e1b793907545e50a2ea2ddb5ba6c58c4506 tree   136 136 1178
d982c7cb2c2a972ee391a85da481fc1f9127a01d tree   6 17 1314 1 \
  deef2e1b793907545e50a2ea2ddb5ba6c58c4506
3c4e9cd789d88d8d89c1073707c3585e41b0e614 tree   8 19 1331 1 \
  deef2e1b793907545e50a2ea2ddb5ba6c58c4506
0155eb4229851634a0f03eb265b69f5a2d56f341 tree   71 76 1350
83baae61804e65cc73a7201a7252750c76066a30 blob   10 19 1426
fa49b077972391ad58037050f2a75f74e3671e92 blob   9 18 1445
b042a60ef7dff760008df33cee372b945b6e884e blob   22054 5799 1463
033b4468fa6b2a9547a70d88d1bbe8bf3f9ed0d5 blob   9 20 7262 1 \
  b042a60ef7dff760008df33cee372b945b6e884e
1f7a7a472abf3dd9643fd615f6da379c4acb3e3a blob   10 19 7282
non delta: 15 objects
chain length = 1: 3 objects
.git/objects/pack/pack-978e03944f5c581011e6998cd0e9e30000905586.pack: ok
```

> ```
> 033b4468fa6b2a9547a70d88d1bbe8bf3f9ed0d5 blob   9 20 7262 1 \
>   b042a60ef7dff760008df33cee372b945b6e884e
> ```
>
> 中 033b4 是第一个版本的 `repo.rb` 而 b042a 是第二个版本的

第三列表示这个对象在 pack 文件中占的大小（也就是第一个数字），可以发现，只占用了 9B。

而且第二个文件是被完整存储的，而第一个文件被存储为 delta，这是因为我们更可能需要更快地访问最新的文件。

## 五、Refspec

// TODO

## 六、Transfer Protocols

Git 可以在两个仓库之间以两种主要的方式来传输数据：一个“蠢”协议，一个“聪明”协议。

### 蠢协议

之所以说它蠢，是因为它不要求服务端有任何 Git 相关的代码，所有请求都是基于 HTTP `GET` 的。

比如对于下面这个 `http-fetch` 过程：

```
$ git clone http://server/simplegit-progit.git
```

它所做的事情如下：

1. 获取引用及 SHA-1 列表：获取 `info/refs` 文件（由 `update-server-info` 命令写入）

    ```
    => GET info/refs
    ca82a6dff817ec66f44342007202690a93763949     refs/heads/master
    ```

2. 了解要 Check Out 什么：获取 HEAD

    ```
    => GET HEAD
    ref: refs/heads/master
    ```

3. 

咕
