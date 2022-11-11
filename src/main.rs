use dotenv::dotenv;
mod config;

fn main() {
    // 解析 .env文件
    dotenv().ok();
    // 初始化配置
    let cfg = config::Config::from_env().expect("初始化配置失败");
    println!("{:?}", cfg);
}
