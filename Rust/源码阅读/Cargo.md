# Cargo 源码阅读

## 项目结构

Cargo 的仓库是一个 workspace，其根 package 具有 lib 和 bin 两种 target：

- bin：位于 `src/bin/cargo`

- lib：位于 `src/cargo`，在 `Cargo.toml` 中显式声明

    ```toml
    [lib]
    name = "cargo"
    path = "src/cargo/lib.rs"
    ```

在 `crates/*` 目录下还有很多 package，都属于 workspace 的成员。

---

`src/bin/cargo/commands` 下有对每一个 Cli Command 的实现，在这里会对 Cli 参数进行处理，转换为 `src/cargo/ops` 中的 `XxxOp` 和 `XxxOptions`。

`src/cargo/ops` 下则是对每一个具体操作的相关实现，每一个操作的实际执行函数都较为类似，接受两个参数：

- `ws: &Workspace<'a>'`
- `options: &XxxOptions<'a>`：操作的选项

对于 `add` 这种可以接受一个列表的输入依此执行的命令，列表的每一项会被实现为一个 `XxxOp`，并且在 `XxxOptions` 中作为 `Vec<XxxOp>` 传入。

---

对于