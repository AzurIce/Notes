资源更新逻辑：

1. 若当前无资源，则 clone
2. 若有资源，fetch 最新的 version 于当前比较，有新版本才 pull

为 resource 目录新添加一个 resource.toml，在其中添加一个 `last_update`，然后使用 just 去更新

## Task 格式

```rust
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Task 的名称
    pub name: String,
    /// Task 的描述
    pub desc: Option<String>,
    /// Task 的步骤
    pub steps: Vec<TaskStep>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskStep {
    /// 在此 Step 开始前的延迟
    pub delay_sec: Option<f32>,
    /// 如果此 Step 失败，是否跳过（否则会直接中断退出）
    pub skip_if_failed: Option<bool>,
    /// 重复次数
    pub repeat: Option<u32>,
    /// 每次重试次数
    pub retry: Option<i32>,
    /// 在此 Step 中要执行的 Action
    pub action: Action,
}
```

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    ByName(String),
    // Multi(Multi),
    // Action
    ActionPressEsc,
    ActionPressHome,
    ActionClick {
        x: u32,
        y: u32,
    },
    ActionSwipe {
        p1: (u32, u32),
        p2: (i32, i32),
        duration: f32,
        slope_in: f32,
        slope_out: f32,
    },
    ActionClickMatch {
        match_task: MatchTask,
    },
    // Navigate
    NavigateIn(String),
    NavigateOut(String),
}
```

```toml
name = "start_up"
desc = "start_up to the main screen"

[[steps]]
retry = -1 # keep retry until success
[steps.action.ActionClickMatch]
type = "Template"
template = "start_start.png"

[[steps]]
retry = -1 # keep retry until success
[steps.action.ActionClickMatch]
type = "Template"
template = "wakeup_wakeup.png"

[[steps]]
delay_sec = 6.0
skip_if_failed = true
retry = 3
[steps.action.ActionClickMatch]
type = "Template"
template = "confirm.png"

[[steps]]
delay_sec = 2.0
skip_if_failed = true
retry = 2
[steps.action.ActionClickMatch]
type = "Template"
template = "qiandao_close.png"

[[steps]]
delay_sec = 2.0
skip_if_failed = true
retry = 2

[steps.action.ActionClickMatch]
type = "Template"
template = "notice_close.png"
```

## Copilot 格式

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Copilot {
    pub name: String,
    pub level_code: String,
    pub operators: HashMap<String, String>,
    pub steps: Vec<CopilotStep>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CopilotStep {
    pub time: CopilotStepTime,
    pub action: CopilotAction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CopilotStepTime {
    DeltaSec(f32),
    /// As Soon As Possible
    Asap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CopilotAction {
    Deploy {
        operator: String,
        position: (u32, u32),
        direction: Direction,
    },
    AutoSkill {
        operator: String,
    },
    StopAutoSkill {
        operator: String,
    },
    Retreat {
        operator: String,
    },
}
```

```toml
name = "1-4"
level_code = "1-4"

[operators]
spot = "char_284_spot"
lancet = "char_285_medic2"
rangers = "char_503_rang"
ansel = "char_212_ansel"
noir_corne = "char_500_noirc"
durin = "char_501_durin"
melantha = "char_208_melan"
yato = "char_502_nblade"
myrtle = "char_151_myrtle"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "myrtle"
position = [
    2,
    1,
]
direction = "Right"

[[steps]]
time = "Asap"

[steps.action.AutoSkill]
operator = "myrtle"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "yato"
position = [
    3,
    1,
]
direction = "Right"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "noir_corne"
position = [
    4,
    1,
]
direction = "Right"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "melantha"
position = [
    2,
    2,
]
direction = "Down"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "ansel"
position = [
    5,
    2,
]
direction = "Up"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "rangers"
position = [
    1,
    3,
]
direction = "Down"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "durin"
position = [
    5,
    3,
]
direction = "Up"

[[steps]]
time = "Asap"

[steps.action.Retreat]
operator = "yato"

[[steps]]
time = "Asap"

[steps.action.Deploy]
operator = "spot"
position = [
    3,
    1,
]
direction = "Up"
```

## Navigate 格式

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Navigate {
    pub enter_task: Task,
    pub exit_task: Task,
}
```

