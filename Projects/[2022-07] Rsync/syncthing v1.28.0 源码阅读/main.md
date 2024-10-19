经过对于命令行参数的解析后 `syncthing serve` 对应的入口函数位于 https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/cmd/syncthing/main.go#L268

命令行参数文档：[Syncthing — Syncthing documentation](https://docs.syncthing.net/users/syncthing.html)

环境变量文档：[Debugging Syncthing — Syncthing documentation](https://docs.syncthing.net/dev/debugging.html#environment-variables)

默认情况下无 `STMONITORED` 环境变量设置的情况下，会进入 [`monitorMain`](https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/cmd/syncthing/monitor.go#L46)，在其中会在设定这个变量之后再通过子进程以相同的参数启动 syncthing，于是就会进入真正的 [`syncthingMain`](https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/cmd/syncthing/main.go#L518)。

## syncthingMain

首先会加载或生成一个 TLS 证书 `cert`，用于后续的安全连接使用。

然后会启动 `earlyService`（处理一切启动 app 需要的服务，如 logger、配置文件等）

然后会加载数据库 `ldb`

然后完成真正的应用，`app` 的初始化：

```go
// https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/cmd/syncthing/main.go#L637
app, err := syncthing.New(cfgWrapper, ldb, evLogger, cert, appOpts)
```

然后进行 `app.Start()` 启动 `app`，在 API 就绪后返回，再通过 `app.Wait()` 阻塞等待 `app` 结束（通过读取 `app.stopped` channel 实现）。

## app.startup()

`app.Start()` 中会通过 `app.startup()` 来启动应用。

在其中可以看到，设备 ID 是用 TLS 证书的第 0 个字节生成的。

### 1. model

然后会生成一个关键的服务并添加到 `mainService` 下：

```go
// https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/lib/syncthing/syncthing.go#L247
keyGen := protocol.NewKeyGenerator()
m := model.NewModel(a.cfg, a.myID, a.ll, protectedFiles, a.evLogger, keyGen)
```

注释是这样写的：

```
// NewModel creates and starts a new model. The model starts in read-only mode,
// where it sends index information to connected peers and responds to requests
// for file data without altering the local folder in any way.
```

创建并以只读模式运行一个新的 Model，具有以下两个功能：

- 发送索引信息给连接的设备
- 响应对于文件数据的请求。

### 2. discover 与 connection

还会生成 `discoveryManager` 和 `connectionsService` 两个服务并添加到 `mainService` 下：

```go
// https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/lib/syncthing/syncthing.go#L268

addrLister := &lateAddressLister{}

connRegistry := registry.New()
discoveryManager := discover.NewManager(a.myID, a.cfg, a.cert, a.evLogger, addrLister, connRegistry)
connectionsService := connections.NewService(a.cfg, a.myID, m, tlsCfg, discoveryManager, bepProtocolName, tlsDefaultCommonName, a.evLogger, connRegistry, keyGen)

```

## model

上面提到的 [`NewModel`](https://github.com/syncthing/syncthing/blob/6d64daaba326ba8378c700e00a31e425c7c90579/lib/model/model.go#L205) 中会初始化 model，创建并添加四个服务：

- `folderRunners`
- `progressEmitter`：负责传输进度条相关，暂略
- `indexHandlers`
- `serve`，每过一段时间执行 `promoteConnections`。

`folderRunners` 和 `indexHandlers` 都通过 `newServiceMap` 创建：

```go
folderRunners: newServiceMap[string, service](evLogger),
indexHandlers: newServiceMap[protocol.DeviceID, *indexHandlerRegistry](evLogger),
```

### AddConnections

注释是这样写的：

```
// AddConnection adds a new peer connection to the model. An initial index will
// be sent to the connected peer, thereafter index updates whenever the local
// folder changes.
```

会首先发送一个初始的索引，然后每当文件夹发生变更时更新索引。

### promoteConnections

```
// promoteConnections checks for devices that have connections, but where
// the primary connection hasn't started index handlers etc. yet, and
// promotes the primary connection to be the index handling one. This should
// be called after adding new connections, and after closing a primary
// device connection.
```

用 `conn.Start()` 启动 connections

## connections

`NewService` 中会创建几个服务：

- `connect`：循环对断开连接的设备进行连接
- `handleConns`：响应并建立连接，发送到 `s.hellos` 中
- `handleHellos`：处理 `s.hellos` 中的连接，添加到 `s.model` 中（`model.AddConnection`）
- `natService`

### Start

启动了很多个协程循环：

- `readerLoop`：通过 `readMessage` 读取信息写入 `c.inbox`
- `dispatcherLoop`：真正的主循环，见下
- `writerLoop`：通过 `writeMessage` 写信息
- `pingSender`
- `pingReceiver`

### dispatcherLoop

从 `c.inbox` 读取消息并处理：

- `ClusterConfig`
- `Close`
- `Index`
- `IndexUpdate`
- `Request`
- `Response`
- `DownloadProgress`