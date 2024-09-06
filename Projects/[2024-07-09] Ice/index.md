# Ice

这个想法，可以说从高中时期就有了 —— 基于 Minecraft 服务端的标准输入输出流实现的 Minecraft Server Daemon。

Minecraft Server 的输入命令、输出信息都有着固定的格式，包含着大量可以利用的信息，可以简单地实现很多原版 MC 乃至 Mod 无法实现的东西。

一个典型的例子就是当玩家输入 `#bksnap make` 的时候，为服务器存档保存为一个快照备份，输入 `#bksnap list` 的时候列出快照列表，输入 `#bksnap load <id>` 的时候加载对应的快照。

主要的启发来源于 TIS Trinity Union 的 [MCDReforged/MCDReforged: A rewritten version of MCDaemon, a python tool to control your Minecraft server (github.com)](https://github.com/MCDReforged/MCDReforged) 以及 [kafuuchino-desu/MCDaemon: a python script for automatically controlling Vanilla Minecraft server (github.com)](https://github.com/kafuuchino-desu/MCDaemon)。

当时高二疫情期间，用 Python 一边啃一边搓，模仿着做出了我自己的 MCSH（仓库好像已经被我删掉了），再之后由于接触了了 Golang，且受 Python 的虚假多线程烦扰，用 Golang 重写，于是产生了 [AzurIce/MCSHGo: A minecraft server helper based on stdin/out from server. Reforged from MCSH(my private repo, written in python) (github.com)](https://github.com/AzurIce/MCSHGo)，当时还做了网页后端操作存档、控制台。近来，一段时间的沉寂，又玩起了 MC，同时在接触了 Rust 后，在脑中酝酿了很久重构为 Rust 的想法，这就是 [AzurIce/ice: A minecraft cli helper (github.com)](https://github.com/AzurIce/Ice)。

可以说这个项目从最开始就伴随了我在软件编程方面的成长（笑）。

现在 MCDR 已经发展得很好了，生态也十分完善，不过我还是想自己写一个~

## 思路 

目前 Ice 已经有了基本的框架和雏形，以及基本的备份命令。

接下来的思路是，参照一下 MCDR 的插件 API 设计，先在 Ice 中将对应的 API 提供出来，然后开始抄插件：

- PrimeBackup

    [TISUnion/PrimeBackup: A powerful backup plugin for MCDR, an advanced backup solution for your Minecraft world (github.com)](https://github.com/TISUnion/PrimeBackup)

    先做这个，尝试拯救一下我的硬盘（）

- 镜像服相关

- 一些简单实用的小工具

    here、计分板等等

## 相关内容

[MCDReforged/MCDReforged: A rewritten version of MCDaemon, a python tool to control your Minecraft server (github.com)](https://github.com/MCDReforged/MCDReforged?tab=readme-ov-file)

MCDR 插件开发文档：[MCDR 插件 — MCDReforged 2.13.1 文档](https://docs.mcdreforged.com/zh-cn/latest/plugin_dev/basic.html#what-is-a-mcdr-plugin)

插件市场：[Plugin Catalogue - MCDReforged](https://mcdreforged.com/en/plugins)
