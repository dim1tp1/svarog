# 1. 编译

1. 把proto编译成代码: `make proto`.
2. 编译整个cargo workspace: `make`. 输出如下
    * `out/lbc`: 鲁班前端的工作进程.
    * `out/lbm`: 鲁班服务器.
3. 编译出更高效的lbc和lbm: `make release`. 输出同上.

# 2. 部署

## 2.1. MySQL

> 以下数据库操作仅供参考. 执行前务必通晓以下操作的用途和后果. 如破坏已有数据库, 与笔者无关.

1. 配置好mysql和业务用户. 推荐阅读 [这篇文章](https://www.digitalocean.com/community/tutorials/how-to-install-mysql-on-ubuntu-22-04)
2. 修改`RunAsRoot.sql`中的用户名. 以root身份执行 `lbsql/RunAsRoot.sql`.
3. 以业务用户的身份, 依次执行 `Tables.sql`, `Daemons.sql`, `Procedures.sql`.

## 2.2. 服务

### 服务端

先配置四个环境变量

```
SVAROG_MYSQL_USER
SVAROG_MYSQL_PASS
SVAROG_MYSQL_HOST
SVAROG_MYSQL_PORT
```

之后, 启动 lbm 服务

```bash
./lbm [-p <PORT>]
```

默认只监听localhost. 请自行配置反向代理或修改代码中的监听地址.

### 客户端

```bash
./lbc -p <PORT> -d <SHARD_DIR> -s <http://lbm-server:lbm-port>
```

如果`<SHARD_DIR>`是相对路径, 则该路径出现在`./lbc`旁边.

# 3. 运行

## 3.1. 多方Keygen

(1) 发起者调用grpc函数 `LubanManager.BizNewKeygenSession`.

该接口定义于`lx_grpc/proto/lbm.proto`.

接口返回的`session_id`即为上一代鲁班的`uuid`. 接口返回的`expire_at`是秒级unix时间戳. 

> lbm会自动地清理过期会话相关的资源. 接口使用者也可以自觉地在到达`expire_at`之后停止`session_id`的一切操作.

(2) 参与者调用grpc函数 `LubanClient.ClientKeygen`.

该接口定义于`lx_grpc/proto/lbc.proto`.

接口参数中的`session_id`是从第(1)步申请来的; `owner_id`是MPC参与方在业务中的唯一标识. 

接口返回一个流式对象. 前端可以根据返回对象的`current`字段来更新前端进度条.

当`current = total`时, `harvest`为根公钥XPub和分片助记词. 

MPC分片存储到 `<SHARD_DIR>/{owner_id}-{session_id}.keystore`, 分片格式为proto2, 结构定义于 `lx_grpc/proto/gg18.proto`.

Keygen会话成功之后, `session_id`就成为根私钥的`key_id`.

(3) (可选) 任何参与者 (不论是否为发起者) 定时调用grpc函数 `LubanManager.BizPollKeygenProgress` 以查询全局进度.

## 3.2. 根助记词转化为MPC分片

(1) 发起者调用grpc函数 `LubanManager.BizNewKeygenSession`.

(2) 参与者调用grpc函数 `LubanClient.ClientShareRootKey`.

恰有一个`owner_id`提供根私钥的助记词. 同一个根助记词只能在鲁班系统里keygen一次.

(3) (可选) 任何参与者定时调用grpc函数 `LubanManager.BizPollKeygenProgress` 以查询全局进度.

## 3.3. 多方签名

(1) 发起者调用grpc函数 `LubanManager.BizNewSignSession`.

同一个`tx_hash`只能在鲁班系统里sign一次.

(2) 参与者调用grpc函数 `LubanClient.ClientSign`.

(3) (可选) 任何参与者定时调用grpc函数 `LubanManager.BizPollSignProgress`.
