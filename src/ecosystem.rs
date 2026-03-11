/// 生态系统与常用 Crate 示例模块
/// 涵盖：serde、错误处理、tokio、数据库、Web 框架

/// 演示 serde 序列化
fn demo_serde() {
    println!("┌─────────────────────────────────────┐");
    println!("│   1. serde 序列化                    │");
    println!("└─────────────────────────────────────┘");

    println!("\n基本用法：");
    println!("use serde::{{Serialize, Deserialize}};");
    println!();
    println!("#[derive(Serialize, Deserialize)]");
    println!("struct User {{");
    println!("    name: String,");
    println!("    age: u32,");
    println!("    #[serde(rename = \"isActive\")]");
    println!("    is_active: bool,");
    println!("}}");

    println!("\n常用属性：");
    println!("#[serde(rename = \"name\")] - 重命名字段");
    println!("#[serde(skip)] - 跳过字段");
    println!("#[serde(skip_serializing_if = \"Option::is_none\")] - 条件跳过");
    println!("#[serde(default)] - 使用默认值");
    println!("#[serde(flatten)] - 扁平化嵌套");

    println!("\n枚举序列化：");
    println!("#[serde(tag = \"type\", content = \"value\")]");
    println!("enum Message {{");
    println!("    Text(String),");
    println!("    Number(i32),");
    println!("}}");

    println!("\n自定义序列化：");
    println!("impl Serialize for MyType {{");
    println!("    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>");
    println!("    where S: Serializer");
    println!("    {{ ... }}");
    println!("}}");

    println!("\n支持的格式：");
    println!("JSON - serde_json");
    println!("YAML - serde_yaml");
    println!("TOML - toml");
    println!("Bincode - bincode");
    println!("MessagePack - rmp-serde");
}

/// 演示错误处理库
fn demo_error_handling() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   2. 错误处理库                      │");
    println!("└─────────────────────────────────────┘");

    println!("\nthiserror - 库错误定义：");
    println!("use thiserror::Error;");
    println!();
    println!("#[derive(Error, Debug)]");
    println!("pub enum DataStoreError {{");
    println!("    #[error(\"数据连接失败: {{0}}\")]");
    println!("    ConnectionError(String),");
    println!("    #[error(\"记录未找到: {{id}}\")]");
    println!("    NotFound {{ id: u64 }},");
    println!("    #[error(\"IO 错误\")]");
    println!("    Io(#[from] std::io::Error),");
    println!("}}");

    println!("\nanyhow - 应用错误处理：");
    println!("use anyhow::{{Context, Result, bail}};");
    println!();
    println!("fn read_config(path: &str) -> Result<Config> {{");
    println!("    let content = std::fs::read_to_string(path)");
    println!("        .with_context(|| format!(\"无法读取: {{}}\", path))?;");
    println!("    if config.port == 0 {{");
    println!("        bail!(\"端口号不能为 0\");");
    println!("    }}");
    println!("    Ok(config)");
    println!("}}");

    println!("\n选择建议：");
    println!("库代码 - 使用 thiserror");
    println!("应用代码 - 使用 anyhow");
    println!("组合使用 - 库定义错误类型，应用使用 anyhow");

    println!("\n错误处理最佳实践：");
    println!("1. 提供有意义的错误信息");
    println!("2. 使用 context 添加上下文");
    println!("3. 区分可恢复和不可恢复错误");
    println!("4. 避免错误信息泄露敏感信息");
}

/// 演示 tokio 异步运行时
fn demo_tokio() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   3. tokio 异步运行时                │");
    println!("└─────────────────────────────────────┘");

    println!("\n运行时配置：");
    println!("#[tokio::main(flavor = \"multi_thread\", worker_threads = 4)]");
    println!("async fn main() {{ ... }}");
    println!();
    println!("#[tokio::main(flavor = \"current_thread\")]");
    println!("async fn main() {{ ... }}");

    println!("\n任务管理：");
    println!("let handle = tokio::spawn(async {{ 42 }});");
    println!("let result = handle.await.unwrap();");
    println!();
    println!("let mut set = tokio::task::JoinSet::new();");
    println!("set.spawn(async {{ task1() }});");
    println!("set.spawn(async {{ task2() }});");

    println!("\n同步原语：");
    println!("tokio::sync::Mutex - 异步互斥锁");
    println!("tokio::sync::RwLock - 异步读写锁");
    println!("tokio::sync::Semaphore - 信号量");
    println!("tokio::sync::mpsc - 多生产者单消费者通道");
    println!("tokio::sync::broadcast - 广播通道");

    println!("\n定时器：");
    println!("tokio::time::sleep(Duration::from_secs(1)).await;");
    println!("let interval = tokio::time::interval(Duration::from_millis(100));");

    println!("\n网络 I/O：");
    println!("tokio::net::TcpListener");
    println!("tokio::net::TcpStream");
    println!("tokio::net::UdpSocket");
    println!("tokio::fs - 异步文件操作");
}

/// 演示数据库访问
fn demo_database() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   4. 数据库访问                      │");
    println!("└─────────────────────────────────────┘");

    println!("\nsqlx - 编译时检查 SQL：");
    println!("use sqlx::postgres::PgPoolOptions;");
    println!();
    println!("let pool = PgPoolOptions::new()");
    println!("    .max_connections(5)");
    println!("    .connect(\"postgres://...\").await?;");
    println!();
    println!("let users = sqlx::query_as::<_, User>(\"SELECT * FROM users\")");
    println!("    .fetch_all(&pool).await?;");

    println!("\nsqlx 特点：");
    println!("1. 编译时 SQL 验证");
    println!("2. 异步 API");
    println!("3. 多数据库支持");
    println!("4. 无 ORM 开销");

    println!("\ndiesel - ORM 框架：");
    println!("use diesel::prelude::*;");
    println!();
    println!("let users = users::table");
    println!("    .filter(users::id.gt(0))");
    println!("    .load::<User>(&mut conn)?;");
    println!();
    println!("diesel::insert_into(users::table)");
    println!("    .values(&new_user)");
    println!("    .execute(&mut conn)?;");

    println!("\ndiesel 特点：");
    println!("1. 类型安全查询");
    println!("2. 编译时检查");
    println!("3. 迁移管理");
    println!("4. Schema 生成");

    println!("\n选择建议：");
    println!("sqlx - 更灵活，更接近原生 SQL");
    println!("diesel - 更安全，ORM 抽象");
    println!("sea-orm - 异步 ORM，类似 diesel");
}

/// 演示 Web 框架
fn demo_web_frameworks() {
    println!("\n┌─────────────────────────────────────┐");
    println!("│   5. Web 框架                        │");
    println!("└─────────────────────────────────────┘");

    println!("\naxum - Tower 生态：");
    println!("use axum::{{Router, routing::get, Json}};");
    println!();
    println!("let app = Router::new()");
    println!("    .route(\"/users/:id\", get(get_user))");
    println!("    .route(\"/users\", post(create_user));");
    println!();
    println!("async fn get_user(Path(id): Path<u64>) -> Json<User> {{ ... }}");

    println!("\naxum 特点：");
    println!("1. 基于 Tower 服务");
    println!("2. 类型安全提取器");
    println!("3. 无宏路由");
    println!("4. 与 tokio 完美集成");

    println!("\nactix-web - 高性能：");
    println!("use actix_web::{{web, App, HttpServer, HttpResponse}};");
    println!();
    println!("HttpServer::new(|| {{");
    println!("    App::new()");
    println!("        .route(\"/users\", web::get().to(get_users))");
    println!("}})");
    println!(".bind(\"127.0.0.1:8080\")?.run().await");

    println!("\nactix-web 特点：");
    println!("1. 高性能");
    println!("2. Actor 模型");
    println!("3. WebSocket 支持");
    println!("4. 中间件生态");

    println!("\n框架选择：");
    println!("axum - 简洁，Tower 生态");
    println!("actix-web - 高性能，功能丰富");
    println!("warp - 函数式，Filter 组合");
    println!("rocket - 易用，宏驱动");
}

/// 公开的演示函数
pub fn demo_ecosystem() {
    println!("\n╔═════════════════════════════════════════╗");
    println!("║      模块十五：生态系统与常用 Crate      ║");
    println!("╚═════════════════════════════════════════╝");

    demo_serde();
    demo_error_handling();
    demo_tokio();
    demo_database();
    demo_web_frameworks();
}
