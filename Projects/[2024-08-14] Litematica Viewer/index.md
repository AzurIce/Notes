# Litematica Viewer

最近产生了一个想法 —— 对 [Litematica - Minecraft Mod (modrinth.com)](https://modrinth.com/mod/litematica) 的 `.litematica` 文件做一个可视化工具。

初步设想是 Rust + Bevy + webgpu，这样还可以编译到 web 上，以后还能做一个 LitematicaHub 也说不定（）。

## 相关调查

NBT 相关：

- \[<img src="https://raw.githubusercontent.com/FortAwesome/Font-Awesome/6.x/svgs/brands/rust.svg" width="20" height="20"> crate] [fastnbt - crates.io: Rust Package Registry](https://crates.io/crates/fastnbt)：NBT 格式的 serde 支持
- [NBT - wiki.vg](https://wiki.vg/NBT)：NBT 格式的 wiki 页面

原理图相关：

- \[<img src="https://raw.githubusercontent.com/FortAwesome/Font-Awesome/6.x/svgs/brands/rust.svg" width="20" height="20"> crate] [mc_schem - crates.io: Rust Package Registry](https://crates.io/crates/mc_schem)：原理图文件的相关操作

其他：

- [mindstorm38/mc173: A work-in-progress (80%) Minecraft beta 1.7.3 server made in Rust. (github.com)](https://github.com/mindstorm38/mc173)



[iceiix/stevenarella: Multi-protocol Minecraft-compatible client written in Rust (github.com)](https://github.com/iceiix/stevenarella)

[BGR360/brine: Work-in-progress Minecraft client written in Rust using the Bevy game engine. (github.com)](https://github.com/BGR360/brine)

[PrismarineJS/minecraft-data: Language independent module providing minecraft data for minecraft clients, servers and libraries. (github.com)](https://github.com/PrismarineJS/minecraft-data)

[Trivernis/minecraft-data-rs: Rust wrapper for minecraft-data (github.com)](https://github.com/Trivernis/minecraft-data-rs)

