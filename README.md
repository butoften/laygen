# Laygen - Rust Axum Code Generator

一个基于 Rust 开发的交互式命令行工具，用于快速生成 Axum 框架的 Controller 和 Service 代码。

## 功能特性

- ✅ 交互式目录导航
- ✅ 自动生成 controller 和 service 文件
- ✅ 自动更新 mod.rs 文件
- ✅ 支持中英文双语界面
- ✅ 可自定义目录名称
- ✅ service 文件名自动添加后缀选项

## 安装

```bash
cd your/path/laygen
cargo install --path .
```

## 使用方法

### 基本用法

```bash
laygen <目标目录>
```

### 完整参数

```bash
laygen [OPTIONS] <TARGET>

Arguments:
  <TARGET>  目标目录路径

Options:
  -f, --framework <FRAMEWORK>        框架名称 [default: axum]
  -l, --language <LANGUAGE>          语言 (en/zh) [default: en]
      --controller-dir <CONTROLLER_DIR>  Controller 目录名 [default: controller]
      --service-dir <SERVICE_DIR>        Service 目录名 [default: service]
  -h, --help                         显示帮助信息
  -V, --version                      显示版本信息
```

### 使用示例

#### 1. 使用默认设置（英文界面）

```bash
laygen /path/to/your/project
```

#### 2. 使用中文界面

```bash
laygen /path/to/your/project -l zh
```

或者

```bash
laygen /path/to/your/project --language zh
```

#### 3. 自定义目录名称

```bash
laygen /path/to/your/project --controller-dir controllers --service-dir services
```

#### 4. 完整示例

```bash
laygen ~/my-axum-project -l zh --controller-dir api --service-dir logic
```

## 工作流程

1. **语言选择**：如果指定的语言不是 `en` 或 `zh`，会提示选择语言
2. **目录检查**：检查 controller 和 service 目录是否存在
   - 如果不存在，会询问是否创建
   - 可以选择创建或放弃
3. **Controller 生成**：
   - 导航到目标子目录（可以多级导航）
   - 输入控制器文件名
   - 输入方法名（可留空，默认使用文件名）
4. **Service 生成**：
   - 导航到目标子目录
   - 输入服务文件名
   - 输入方法名（可留空，默认使用文件名）
   - 选择是否添加 `_service` 后缀
5. **自动更新**：自动更新对应目录的 `mod.rs` 文件

## 生成的代码模板

### Controller 模板

```rust
use axum::extract::Query;
use axum::extract::State;
use axum::response::Json;
use axum::Extension;
use error_crate::api_error::{ApiError, ApiResult};
use public::safe_json::SafeJson;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct YourNameRequest {
    // Add your request fields here
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct YourNameResponse {
    // Add your response fields here
}

pub async fn your_method(
    Extension(domain): Extension<String>,
    Query(query): Query<QueryParams>,
    State(state): State<AppState>,
    SafeJson(body): SafeJson<YourNameRequest>,
) -> ApiResult<Json<ApiResponse<serde_json::Value>>> {
    let res = service::your_method(&state.conn, &domain).await?;
    
    let result = origin_display(query, res, &domain);
    
    Ok(ApiResponse::success(result).to_json())
}
```

### Service 模板

```rust
use sea_orm::DatabaseConnection;
use error_crate::api_error::ApiResult;

pub async fn your_method(
    conn: &DatabaseConnection,
    domain: &str,
) -> ApiResult<()> {
    // Add your service logic here
    
    Ok(())
}
```

## 项目结构

```
laygen/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs        # 程序入口
    ├── cli.rs         # 命令行参数处理和主流程
    ├── i18n.rs        # 国际化支持
    ├── file_ops.rs    # 文件操作（目录导航、mod.rs 更新）
    └── generator.rs   # 代码生成模板
```

## 依赖项

- `clap` - 命令行参数解析
- `dialoguer` - 交互式用户界面
- `console` - 终端样式
- `anyhow` - 错误处理
- `walkdir` - 目录遍历

## 开发

### 构建

```bash
cargo build
```

### 运行测试

```bash
cargo test
```

### 开发模式运行

```bash
cargo run -- /path/to/test/project -l zh
```

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
