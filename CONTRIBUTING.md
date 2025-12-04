# 贡献指南

感谢你对本项目的关注！本项目旨在复刻原网页版系统的功能，提供查询订单和执行订单相关操作的命令行工具。为了让协作顺畅，请遵循以下指南。

## 环境准备

- 安装 Rust 稳定版工具链（建议使用 `rustup`）。
- 安装开发组件：`rustup component add clippy rustfmt`。
- 推荐使用基于 Visual Studio Code 的开发环境，配合 Rust 拓展 [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。

## 项目结构速览

- `src/main.rs`：CLI 入口与参数解析（`clap`）。
- `src/api.rs`：请求封装与重试策略（`reqwest` 阻塞客户端）。
- `src/models.rs`：数据模型与序列化（`serde`）。
- `src/parser.rs`：数据解析逻辑。
- `src/display.rs`：输出排版与高亮（`tabled`、`colored`）。
- `src/constants.rs`：常量与枚举值（接口地址、模式、英雄等）。
- `tests/cli.rs`：CLI 端到端与参数校验测试。
- `doc/`：原系统 API、HTML 与项目规则示例文档。

## 代码风格与质量

- 格式化：`cargo fmt --all`
- Lint：`cargo clippy --all-targets --all-features -- -D warnings`

## 测试

- 运行测试：`cargo test --workspace`

## 提交规范

[约定式提交（Conventional Commits）](https://www.conventionalcommits.org/zh-hans/v1.0.0/)

## 分支策略

- 主分支：`master`
- 开发分支：`dev`

## 拉取请求（PR）流程

Fork 项目后，基于 `dev` 分支创建新分支或直接在 `dev` 分支开发，创建 PR 时以主仓库的 `dev` 分支为目标。

## 安全与隐私

绝不可提交任何真实订单号、订单编号、密码或个人身份信息到仓库（无论是主仓库还是 Fork 仓库），也不要在讨论中泄露这些信息。

**再次感谢你的贡献！**
