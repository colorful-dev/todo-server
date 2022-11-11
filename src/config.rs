use serde::Deserialize;

/// Web 配置
#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub addr: String,
}

/// 应用配置
#[derive(Deserialize, Debug)]
pub struct Config {
    pub web: WebConfig,
}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder().add_source(config::Environment::default());
        let cfg = cfg.build()?;
        cfg.try_deserialize()
    }
}
