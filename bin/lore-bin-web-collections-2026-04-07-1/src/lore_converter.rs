use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use lore_impl_web_collections_2026_04_07_2::{
    impl_context::ImplContext,
    lore_html::LoreHtml,
    parser::Parser,
};
use lore_web_collections_core::LineType;

/// Lore 文件转换器
pub struct LoreConverter {
    pub link_base: String,
    pub css_url: String,
}

impl LoreConverter {
    /// 创建新的转换器实例
    pub fn new(link_base: String, css_url: String) -> Self {
        Self { link_base, css_url }
    }

    /// 从配置文件创建转换器
    pub fn from_config(config: &crate::config::Config) -> Self {
        Self {
            link_base: config.link_base.clone(),
            css_url: config.css_url.clone(),
        }
    }

    /// 处理单个文件：从 .lore 文件生成 .html 文件
    pub fn process_lore_file(&self, src: &Path, dst: &Path) -> io::Result<()> {
        println!("处理文件: {:?}", src);

        // 1. 读取 .lore 文件
        let content = fs::read_to_string(src)?;

        // 2. 解析 lore 内容为 IR 节点
        let ir_nodes = self.parse_lore_content(&content);

        // 3. 提取文件标题
        let title = self.extract_title(src);

        // 4. 转换为 HTML
        let lore_html = self.convert_to_html(&title, &ir_nodes);

        // 5. 生成最终的 HTML 字符串
        let final_html: String = lore_html.into();

        // 6. 写入文件
        let dst_html = dst.with_extension("html");
        fs::write(dst_html, final_html)?;

        Ok(())
    }

    /// 解析 lore 内容为 IR 节点
    fn parse_lore_content(&self, content: &str) -> Vec<LineType> {
        content
            .lines()
            .map(|line| lore_lexer_web_collections_2026_04_07_1::Parser::parse_line(line))
            .collect()
    }

    /// 提取文件名作为标题
    fn extract_title(&self, src: &Path) -> String {
        src.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    /// 将 IR 节点转换为 LoreHtml
    fn convert_to_html(&self, title: &str, ir_nodes: &[LineType]) -> LoreHtml {
        // 创建 ImplContext
        let impl_context = ImplContext {
            title,
            link_base: &self.link_base,
            css_url: &self.css_url,
        };

        // 创建解析器
        let parser = Parser {
            impl_context: &impl_context,
        };

        // 解析并生成 LoreHtml
        parser.parse(ir_nodes, &impl_context)
    }

    /// 确保目录存在
    fn ensure_dir(&self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// 获取相对路径
    fn get_relative_path(&self, full_path: &Path, base_path: &Path) -> PathBuf {
        full_path.strip_prefix(base_path)
            .unwrap_or(full_path)
            .to_path_buf()
    }

    /// 检查是否为 .lore 文件
    fn is_lore_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "lore")
            .unwrap_or(false)
    }

    /// 遍历目录并处理所有 .lore 文件
    pub fn convert_directory(&self, src_dir: &Path, dst_dir: &Path) -> io::Result<()> {
        self.ensure_dir(dst_dir)?;

        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let src_path = entry.path();

            if src_path.is_dir() {
                // 递归处理子目录
                let rel_path = self.get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                self.convert_directory(&src_path, &dst_path)?;
            } else if self.is_lore_file(&src_path) {
                // 只处理 .lore 文件
                let rel_path = self.get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                self.ensure_dir(dst_path.parent().unwrap())?;
                self.process_lore_file(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_creation() {
        let converter = LoreConverter::new(
            "https://example.com/".to_string(),
            "style.css".to_string(),
        );
        assert_eq!(converter.link_base, "https://example.com/");
        assert_eq!(converter.css_url, "style.css");
    }

    #[test]
    fn test_extract_title() {
        let converter = LoreConverter::new(
            "https://example.com/".to_string(),
            "style.css".to_string(),
        );
        let path = Path::new("/some/path/my_lore_file.lore");
        assert_eq!(converter.extract_title(path), "my_lore_file");
    }

    #[test]
    fn test_is_lore_file() {
        let converter = LoreConverter::new(
            "https://example.com/".to_string(),
            "style.css".to_string(),
        );
        assert!(converter.is_lore_file(Path::new("test.lore")));
        assert!(!converter.is_lore_file(Path::new("test.txt")));
        assert!(!converter.is_lore_file(Path::new("test.html")));
    }
}