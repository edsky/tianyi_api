# 天翼路由器非官方API

这是一个非官方的天翼路由器API，用于支持各种功能，如获取公网IP、更新端口转发规则等。

## 安装

在您的项目的 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
tianyi-api = "0.1.0"
```

然后在您的项目中使用此库。

## 示例

以下示例展示了如何使用本库更新端口转发规则。

```
use anyhow::Result;
use tianyi_api::TianyiBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let tianyi_instance = TianyiBuilder::new()
        .username("useradmin")
        .password("***")
        .build()
        .await?;

    let old_ip = "192.168.1.11";
    let new_ip = "192.168.1.12";

    tianyi_instance.update_port_forwarding_rule(old_ip, new_ip).await?;

    println!("Port forwarding rules updated successfully.");

    Ok(())
}
```

## 功能

- 获取公网IP
- 查询端口转发规则
- 添加、删除、启用和禁用端口转发规则
- 查询网关信息
- 登录和登出路由器

更多详细信息，请查看库的文档和示例。

## 许可证

本项目采用MIT许可证。详情请参阅 [LICENSE](https://github.com/edsky/tianyi_api/blob/main/LICENSE) 文件。