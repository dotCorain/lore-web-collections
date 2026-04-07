use std::fs;
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};

// 主程序
fn main() {
    let config = load_from_config();
    let src_dir = Path::new(config.from_lore_path.as_str());
    let dst_dir = Path::new(config.to_html_path.as_str());

    map_lore_files(src_dir, dst_dir, &config.link_base, &config.css_url).unwrap();

    println!("done.");
}

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

/// 处理单个文件：从 .lore 文件生成 .html 文件
fn process_lore_file(src: &Path, dst: &Path, link_base: String, css: &str) -> io::Result<()> {
    println!("处理文件: {:?}", src);

    // 1. 读取 .lore 文件
    let content: String = fs::read_to_string(src)?;

    // 2. 解析 lore 内容
    let mut lore_data = vec![];
    for line in content.lines() {
        lore_data.push(lore_lexer_web_collections_d1::Parser::parse_line(line));
    }

    // 3. 转换
    let parser = lore_implementation_web_collections_to_info::parser::Parser {
        config: lore_implementation_web_collections_to_info::config::Config {
            link_base
        }
    };
    let html_content = parser.parse(&lore_data);

    // 4. 生成
    let file_name = src
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let final_file: String = format!(
        "{} {} {}",
        file_name,
        css.to_string(),
        html_content
    ).into();

    // 6. 修改输出文件名为 .html
    let dst_html = dst.with_extension("html");

    // 7. 写入文件
    fs::write(dst_html, final_file)?;

    Ok(())
}

/// 确保目录存在
fn ensure_dir(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 获取相对路径
fn get_relative_path(full_path: &Path, link_base: &Path) -> PathBuf {
    full_path.strip_prefix(link_base)
        .unwrap_or(full_path)
        .to_path_buf()
}

/// 主函数：遍历目录并处理所有 .lore 文件
pub fn map_lore_files(src_dir: &Path, dst_dir: &Path, link_base: &str, css: &str) -> io::Result<()> {
    ensure_dir(dst_dir)?;

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let src_path = entry.path();

        if src_path
            .is_dir() {
            // 递归处理子目录
            let rel_path = get_relative_path(&src_path, src_dir);
            let dst_path = dst_dir
                .join(rel_path);
            map_lore_files(&src_path, &dst_path, link_base, css)?;
        } else {
            // 只处理 .lore 文件
            if src_path
                .extension()
                .and_then(|ext| ext.to_str()) == Some("lore")
            {
                let rel_path = get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                ensure_dir(dst_path.parent().unwrap())?;
                process_lore_file(&src_path, &dst_path, link_base.to_string(), css)?;
            }
        }
    }

    Ok(())
}