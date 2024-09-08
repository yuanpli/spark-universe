# 🔥SparkGarden

这是一个使用 Rust 和 Actix-web 框架构建的 Web API 项目，提供用户注册、登录以及管理“小火苗🔥”资源的功能。

## 目录结构

```
src/ 
├── config.rs         // 配置文件 
├── db.rs             // 数据库连接和操作 
├── jwt.rs            // JWT 处理 
├── spark.rs          // 处理“小火苗🔥”相关的请求 
└── user.rs           // 用户相关的请求处理 
main.rs               // 应用程序入口
```
## 功能

- 用户注册
- 用户登录
- 获取最新的小火苗🔥
- 获取最受欢迎的小火苗🔥
- 添加新的小火苗🔥
- 点赞小火苗🔥

## 环境要求

- Rust 1.50 及以上版本
- MongoDB 数据库

## 安装与运行

1. 克隆项目到本地：
  ```bash 
   git clone <your-repo-url> 
   cd <your-project-directory>
  ```
2. 安装依赖：
  ```bash
   cargo build
  ```
3. 创建 `.env` 文件并配置 MongoDB 连接信息：
  ```plaintext 
  MONGO_DB_URI=mongodb://localhost:27017 
  MONGO_DB_NAME=your_database_name
  ```
4. 运行项目：
  ```bash
   cargo run
  ```
5. 访问 API：
  Web 服务器将运行在 `http://0.0.0.0:8080`。

## API 端点

- `POST /register` - 注册用户
- `POST /login` - 用户登录
- `GET /sparks/latest` - 获取最新的小火苗🔥
- `GET /sparks/top` - 获取最受欢迎的小火苗🔥
- `POST /sparks` - 添加新的小火苗🔥
- `POST /sparks/{id}/like` - 点赞指定的小火苗🔥

## 贡献

欢迎任何形式的贡献！请提交问题或拉取请求。

## 许可证

该项目使用 MIT 许可证。有关详细信息，请查看 [LICENSE](LICENSE) 文件。