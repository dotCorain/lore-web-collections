use serde::Deserialize;
use std::fs;
use std::path::Path;

// 配置信息
#[derive(Deserialize)]
pub struct Config {
    pub link_base: String,
    pub css_url: String,
    pub from_lore_path: String,
    pub to_html_path: String,
}

pub fn load_from_config() -> Config {
    let config_path = Path::new("Lore.toml");
    let config_string = fs::read_to_string(config_path);
    let config: Config = toml::from_str(
        config_string
            .unwrap()
            .as_str()
    ).unwrap();
    config
}