mod lore_converter;
mod config;

use lore_converter::LoreConverter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_from_config();
    
    let src_dir = Path::new(&config.from_lore_path);
    let dst_dir = Path::new(&config.to_html_path);
    
    // 创建转换器并设置源根目录
    let converter = LoreConverter::from_config(&config)
        .with_src_root(src_dir);
    
    converter.convert_directory(src_dir, dst_dir)?;
    
    println!("done.");
    Ok(())
}