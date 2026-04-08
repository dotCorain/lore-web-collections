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
    /// 源文件根目录（用于计算绝对路径）
    src_root: Option<PathBuf>,
}

impl LoreConverter {
    /// 创建新的转换器实例
    #[allow(unused)]
    pub fn new(link_base: String, css_url: String) -> Self {
        Self { 
            link_base: Self::normalize_link_base(&link_base), 
            css_url,
            src_root: None,
        }
    }

    /// 从配置文件创建转换器
    pub fn from_config(config: &crate::config::Config) -> Self {
        Self {
            link_base: Self::normalize_link_base(&config.link_base),
            css_url: config.css_url.clone(),
            src_root: None,
        }
    }

    /// 规范化 link_base，确保以 "/" 结尾
    fn normalize_link_base(base: &str) -> String {
        if base.ends_with('/') {
            base.to_string()
        } else {
            format!("{}/", base)
        }
    }

    /// 设置源文件根目录（用于计算绝对路径）
    pub fn with_src_root(mut self, src_root: &Path) -> Self {
        self.src_root = Some(src_root.to_path_buf());
        self
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

        // 4. 计算文件的绝对 URL 路径
        let absolute_path = self.calculate_absolute_path(src);

        // 5. 转换为 HTML（使用绝对路径作为 link_base）
        let lore_html = self.convert_to_html_with_path(&title, &ir_nodes, &absolute_path);

        // 6. 生成最终的 HTML 字符串
        let final_html: String = lore_html.into();

        // 7. 写入文件
        let dst_html = dst.with_extension("html");
        fs::write(dst_html, final_html)?;

        Ok(())
    }

    /// 计算文件相对于 link_base 的绝对路径
    fn calculate_absolute_path(&self, src: &Path) -> String {
        if let Some(src_root) = &self.src_root {
            // 如果有源根目录，计算相对路径
            let rel_path = src
                .strip_prefix(src_root)
                .unwrap_or(src)
                .with_extension(""); // 移除 .lore 扩展名
            
            // 转换为 URL 路径（使用正斜杠）
            let url_path = rel_path
                .to_string_lossy()
                .replace('\\', "/");
            
            // 如果是 index 文件，特殊处理
            if url_path.ends_with("/index") || url_path == "index" {
                url_path
                    .trim_end_matches("/index")
                    .trim_end_matches("index")
                    .to_string()
            } else {
                format!("index/{}", url_path)
            }
        } else {
            // 没有源根目录，只用文件名
            let file_stem = src
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            
            if file_stem == "index" {
                String::new()
            } else {
                format!("index/{}", file_stem)
            }
        }
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

    /// 将 IR 节点转换为 LoreHtml（使用绝对路径）
    fn convert_to_html_with_path(
        &self, 
        title: &str, 
        ir_nodes: &[LineType], 
        absolute_path: &str
    ) -> LoreHtml {
        // 构建完整的 link_base（基础 URL + 绝对路径）
        let full_link_base = if absolute_path.is_empty() {
            self.link_base.clone()
        } else {
            format!("{}{}", self.link_base, absolute_path)
        };

        // 创建 ImplContext
        let impl_context = ImplContext {
            title,
            link_base: &full_link_base,
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
        // 如果还没有设置 src_root，使用传入的 src_dir
        let converter = if self.src_root.is_none() {
            Self {
                link_base: self.link_base.clone(),
                css_url: self.css_url.clone(),
                src_root: Some(src_dir.to_path_buf()),
            }
        } else {
            self.clone()
        };

        converter.ensure_dir(dst_dir)?;

        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let src_path = entry.path();

            if src_path.is_dir() {
                // 递归处理子目录
                let rel_path = converter.get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                converter.convert_directory(&src_path, &dst_path)?;
            } else if converter.is_lore_file(&src_path) {
                // 只处理 .lore 文件
                let rel_path = converter.get_relative_path(&src_path, src_dir);
                let dst_path = dst_dir.join(rel_path);
                converter.ensure_dir(dst_path.parent().unwrap())?;
                converter.process_lore_file(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }
}

// 实现 Clone（因为我们需要在 convert_directory 中克隆）
impl Clone for LoreConverter {
    fn clone(&self) -> Self {
        Self {
            link_base: self.link_base.clone(),
            css_url: self.css_url.clone(),
            src_root: self.src_root.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_creation() {
        let converter = LoreConverter::new(
            "https://example.com".to_string(),
            "style.css".to_string(),
        );
        assert_eq!(converter.link_base, "https://example.com/");
        assert_eq!(converter.css_url, "style.css");
    }

    #[test]
    fn test_calculate_absolute_path() {
        let converter = LoreConverter::new(
            "https://example.com".to_string(),
            "style.css".to_string(),
        ).with_src_root(Path::new("/project/lore_src"));

        // 测试普通文件
        let path1 = Path::new("/project/lore_src/guide/getting-started.lore");
        assert_eq!(converter.calculate_absolute_path(path1), "index/guide/getting-started");

        // 测试 index 文件
        let path2 = Path::new("/project/lore_src/index.lore");
        assert_eq!(converter.calculate_absolute_path(path2), "");

        // 测试子目录中的 index
        let path3 = Path::new("/project/lore_src/api/index.lore");
        assert_eq!(converter.calculate_absolute_path(path3), "api");
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

    #[test]
    fn test_normalize_link_base() {
        assert_eq!(
            LoreConverter::normalize_link_base("https://example.com"),
            "https://example.com/"
        );
        assert_eq!(
            LoreConverter::normalize_link_base("https://example.com/"),
            "https://example.com/"
        );
    }
}