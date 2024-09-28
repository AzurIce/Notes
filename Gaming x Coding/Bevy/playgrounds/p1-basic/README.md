# p1-basic

Bevy 中 App、System、Component、Resource 的基本概念

---

## 一、App

一个 Bevy App 由 `App` 创建：

```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

在 Bevy 的 Getting Started[^1] 中，提到 App 包含有三个重要的组成部分：

- `world`：存储所有游戏数据（**Entity**、**Component**、Resources 等）

    可以通过 `App` 的 `world()` 和 `world_mut()` 获取到它的不可变与可变引用。

- `schedule`：存储 **System**

- `runner`：调度 `schedule`

## 二、ECS

### 1. System

System 就是一个函数，可以通过 `App` 的 `add_systems` 添加：

```rust
pub fn add_systems<M>(
    &mut self,
    schedule: impl ScheduleLabel,
    systems: impl IntoSystemConfigs<M>,
) -> &mut App
```

- `schedule`：标识 `sytems` 应该在游戏循环中的什么阶段运行

    每一个 tick 都有很多 `schedule`：[Main in bevy::app - Rust (docs.rs)](https://docs.rs/bevy/latest/bevy/app/struct.Main.html)

    本文中会使用到 `bevy::app::Startup` 和 `bevy::app::Update`

- `systems`：可以用一些工具方法灵活组合配置的 `system`（们）

举个例子：

```rust
fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new().add_systems(Startup, hello_world).run();
    App::new().add_systems(Update, hello_world).run();
}
```

运行结果会打印两个 `hello world!`（分别由 `Startup` 和 `Update` 打印），然后就退出了。

### 2. Component

在 System 函数参数中可以添加一个 `Commands` 参数，利用它可以对 `World` 进行修改：

- 生成或移除 Entity
- 为 Entity 添加 Component
- 插入 Resources
- ......

以生成实体为例，是这样的一个函数：

```rust
pub fn spawn<T>(&mut self, bundle: T) -> EntityCommands<'_>
where
    T: Bundle,
```

可以简单理解 `Bundle` 为 Component 的元组。

此外，System 的函数参数中还可以添加一个 `Query`，利用它可以访问 Entity 与 Component：

```rust
pub struct Query<'world, 'state, D, F = ()>
where
    D: QueryData,
    F: QueryFilter,
{ /* private fields */ }
```

- **`D` (query data)**：只有包含目标数据的实体会被返回
- **`F` (query filter)**：一系列用于筛选的条件

下面是一个使用 `Commands` 在 `StartUp` 阶段初始化 Entity，并在 `Update` 阶段使用 `Query` 输出实体列表的例子：

```rust
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people)) // 添加多个
        .run();
}
```

运行结果：

```
hello Elaina Proctor!
hello Renzo Hume!
hello Zayna Nieves!
hello world!
```

QueryData 也可以是一个可变引用，这样就可以修改 Component 内的数据：

```rust
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn lowercase_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        name.0 = name.0.to_lowercase();
    }
}

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (
            hello_world,
            (lowercase_people, greet_people).chain() // ”灵活组合“
        ))
        .run();
}
```

输出结果：

```
hello world!
hello elaina proctor!
hello renzo hume!
hello zayna nieves!
```

## 三、Plugins

Bevy 的核心原则之一就是「模块化」，引擎的所有功能都被作为 Plugin 实现。

Plugin 即一组修改 App 的代码：

```rust
pub trait Plugin:
    Downcast
    + Any
    + Send
    + Sync {
    // Required method
    fn build(&self, app: &mut App);

    // Provided methods
    fn ready(&self, _app: &App) -> bool { ... }
    fn finish(&self, _app: &mut App) { ... }
    fn cleanup(&self, _app: &mut App) { ... }
    fn name(&self) -> &str { ... }
    fn is_unique(&self) -> bool { ... }
}
```

对于每一个添加到 App 的 Plugin：

- App 会立刻调用 `Plugin::build` 并注册
- 当 App 启动时，会等待所有的 `Plugin::ready` 返回 `true`
- 最终会调用所有已注册 Plugin 的 `Plugin::finish` 以及 `Plugin::cleanup`

为之前的 App 注册 `bevy` 提供的 `DefaultPlugin`（由很多 Plugin 组成的一个 Plugin），再运行程序，可以发现出现了窗口，并持续运行没有终止（其实就是其中的 `WinitPlugin` 的事件循环），`Update` 里的输出也再不断地进行。

## 四、Resources

Resource 是全局唯一的数据，可以在 System 中通过 `Res<T>` 类型的参数访问。

比如使用 Bevy 提供的 `Time` 获取到时钟相关资源：

```rust
fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("[{:?}], hello {}!", time.elapsed(), name.0);
    }
}
```

输出结果：

```
hello world!
[0ns], hello elaina proctor!
[0ns], hello renzo hume!
[0ns], hello zayna nieves!
hello world!
[8.4474ms], hello elaina proctor!
[8.4474ms], hello renzo hume!
[8.4474ms], hello zayna nieves!
hello world!
[144.2783ms], hello elaina proctor!
[144.2783ms], hello renzo hume!
[144.2783ms], hello zayna nieves!
hello world!
[160.3889ms], hello elaina proctor!
[160.3889ms], hello renzo hume!
[160.3889ms], hello zayna nieves!
hello world!
......
```

也可以自己实现一个 Resource 并注册，比如如果想实现每 2s tick 一次，就需要一个全局的计时器，这里可以使用 Bevy 的 `Timer`：

```rust
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("[{:?}] hello {}!", time.elapsed(), name.0);
        }
    }
}
```

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_people)
        .add_systems(Update, 
            (lowercase_people, greet_people).chain() // ”灵活组合“
        )
        .run();
}
```

对于 `TimerMode::Repeating` 类型的 Timer，其状态只有在到达 duration 的那一 tick 为 finish（`just_finished` 和 `finished` 一样），并且在到达 duration 时会自动重置（也可以在任何时候手动重置）。

输出：

```
[2.0081807s] hello elaina proctor!
[2.0081807s] hello renzo hume!
[2.0081807s] hello zayna nieves!
[4.024683s] hello elaina proctor!
[4.024683s] hello renzo hume!
[4.024683s] hello zayna nieves!
[6.0064425s] hello elaina proctor!
[6.0064425s] hello renzo hume!
[6.0064425s] hello zayna nieves!
[8.0095905s] hello elaina proctor!
[8.0095905s] hello renzo hume!
[8.0095905s] hello zayna nieves!
```

## 参考

[^1]: [Apps (bevyengine.org)](https://bevyengine.org/learn/quick-start/getting-started/apps/#what-makes-an-app)