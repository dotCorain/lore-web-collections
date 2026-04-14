pub struct LoreHtml {
    pub title: String,
    pub css_url: String,
    pub html_content: String,
}

impl LoreHtml {
    pub fn new(title: String, css_url: String, html_content: String) -> Self {
        LoreHtml { title, css_url, html_content }
    }
}

impl From<LoreHtml> for String {
    fn from(val: LoreHtml) -> Self {
        // 根据 cargo feature 决定是否添加 MathJax 脚本
        #[cfg(feature = "math")]
        let mathjax_script = r#"
<script>
MathJax = {
  tex: {
    inlineMath: [['$', '$'], ['\\(', '\\)']],
    displayMath: [['$$', '$$'], ['\\[', '\\]']]
  },
  options: {
    ignoreHtmlClass: 'no-mathjax'
  }
};
</script>
<script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js"></script>"#;

        #[cfg(not(feature = "math"))]
        let mathjax_script = "";

        format!(
            r#"<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<link rel="stylesheet" href="{}">
{}
</head>
<body><main>
{}
</main></body>
</html>"#,
            val.title,
            val.css_url,
            mathjax_script,
            val.html_content,
        )
    }
}