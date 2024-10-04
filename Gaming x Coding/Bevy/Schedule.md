# Schedule

```rust
fn main() {
    App::new()
        // ...
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (lowercase_people, greet_people).chain(),
        )
        // ...
        .run();
}
```

Schedule 是一系列 System 的集合，以及执行它们所需要的元信息（比如顺序信息）和执行器（单线程/多线程）。

可以通过 `App::add_schedule` 来添加一个 Schedule：

```rust
pub fn add_schedule(&mut self, schedule: Schedule) -> &mut App
```

添加 Schedule 并不会让这个 Schedule 被自动执行，只是注册了这样一个资源可以向其中添加 System。

## 一、ScheduleLabel

每一个 Schedule 都有自己的 `ScheduleLabel` 作为标识，比如上面的 `Startup`、`Update` 其实都是一个个 derive 了 `ScheduleLabel` 的空结构体：

```rust
/// The schedule that runs once when the app starts.
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Startup;
```

在创建 Schedule 时就需要提供一个 `ScheduleLabel`：

```rust
pub fn new(label: impl ScheduleLabel) -> Schedule
```

## 二、ExecutorKind

每一个 Schedule 都可以设置 [`ScheduleKind`](https://docs.rs/bevy/0.14.2/bevy/ecs/schedule/enum.ExecutorKind.html) ，它决定了这个 Schedule 会被如何执行，一共有三种：

```rust
pub enum ExecutorKind {
    SingleThreaded,
    Simple,
    MultiThreaded,
}
```

获取与设置：

```rust
pub fn get_executor_kind(&self) -> ExecutorKind
```

```rust
pub fn set_executor_kind(&mut self, executor: ExecutorKind) -> &mut Schedule
```

## 三、Schedule 的执行 与 MainScheduleOrder

Bevy 的 `App` 有一个 `main_schedule_label` 字段来设置哪一个 Schedule 会被 App 的 runner 执行，默认情况下它是 `Main`：

第一次运行时会先运行：

- `PreStartup`
- `Startup`
- `PostStartup`

然后会运行：

- `First`

- `PreUpdate`

- `StateTransition`

- `RunFixedMainLoop`

    在这个 Schedule 中会运行若干次 `FixedMain`（取决于距离上一次经过了多少时间）

- `Update`

- `PostUpdate`

- `Last`

那么如果想将通过 `App::add_schedule` 的 Schdule 纳入其中，在特定的阶段前/后运行，就需要操作 `MainScheduleOrder` 这个资源：

```rust
app.add_schedule(Schedule::new(CustomStartup));
let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
main_schedule_order.insert_startup_after(PreStartup, CustomStartup);
```

