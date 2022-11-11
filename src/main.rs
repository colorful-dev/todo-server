use axum::{routing::get, Router};
use dotenv::dotenv;

mod config;
mod handler;
mod error;
mod response;
pub use response::Response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    // 解析 .env文件
    dotenv().ok();
    // 初始化配置
    let cfg = config::Config::from_env().expect("初始化配置失败");
    // 路由
    let app = Router::new().route("/", get(handler::usage));

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
