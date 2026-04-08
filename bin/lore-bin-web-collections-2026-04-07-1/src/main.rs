use std::path::Path;

use lore_bin_web_collections_2026_04_07_1::{config, lore_converter::LoreConverter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = config::load_from_config();
    
    // 创建转换器
    let converter = LoreConverter::from_config(&config);
    
    // 执行转换
    let src_dir = Path::new(&config.from_lore_path);
    let dst_dir = Path::new(&config.to_html_path);
    
    converter.convert_directory(src_dir, dst_dir)?;
    
    println!("ok");
    Ok(())
}