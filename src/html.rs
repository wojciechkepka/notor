#![allow(dead_code)]
use sailfish::{
    runtime::{Buffer, Render, RenderError},
    RenderResult, TemplateOnce,
};
use serde::Serialize;

pub const DEFAULT_LANG: &str = "en";
pub const DEFAULT_CHARSET: Charset = Charset::Utf8;

#[derive(Serialize, Debug, TemplateOnce)]
#[template(path = "html.stpl")]
pub struct HtmlContext {
    lang: String,
    head: HeadContext,
    body: String,
}

impl HtmlContext {
    /// `B` is the type of Body being rendered
    pub fn builder<B>() -> HtmlContextBuilder<B>
    where
        B: TemplateOnce + Default,
    {
        HtmlContextBuilder::default()
    }

    pub fn as_html(self) -> RenderResult {
        self.render_once()
    }
}

pub struct HtmlContextBuilder<B> {
    lang: String,
    head: HeadContext,
    body: Option<B>,
}

impl<B: TemplateOnce> Default for HtmlContextBuilder<B> {
    fn default() -> Self {
        HtmlContextBuilder {
            lang: DEFAULT_LANG.to_string(),
            head: HeadContext::default(),
            body: None,
        }
    }
}

impl<B: TemplateOnce> HtmlContextBuilder<B> {
    pub fn lang<S: Into<String>>(mut self, lang: S) -> Self {
        self.lang = lang.into();
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.head.title = title.into();
        self
    }

    pub fn charset(mut self, charset: &Charset) -> Self {
        self.head.charset = charset.clone();
        self
    }

    pub fn body(mut self, body: B) -> Self {
        self.body = Some(body);
        self
    }

    pub fn add_meta<S: Into<String>>(mut self, name: S, content: S) -> Self {
        self.head.add_meta(name, content);
        self
    }

    pub fn add_script<S: Into<String>>(mut self, src: S) -> Self {
        self.head.add_script(src);
        self
    }

    pub fn add_style<S: Into<String>>(mut self, href: S) -> Self {
        self.head.add_style(href);
        self
    }

    pub fn build(self) -> Result<HtmlContext, RenderError> {
        Ok(HtmlContext {
            lang: self.lang,
            head: self.head,
            body: if let Some(body) = self.body {
                body.render_once()?
            } else {
                "".to_string()
            },
        })
    }
}

#[derive(Serialize, Debug, TemplateOnce, Clone)]
#[template(path = "head.stpl")]
pub struct HeadContext {
    title: String,
    charset: Charset,
    meta_tags: Vec<MetaTag>,
    scripts: Vec<String>,
    styles: Vec<String>,
}

impl HeadContext {
    pub fn add_meta<S: Into<String>>(&mut self, name: S, content: S) {
        self.meta_tags.push(MetaTag::new(name, content))
    }

    pub fn add_script<S: Into<String>>(&mut self, src: S) {
        self.scripts.push(src.into())
    }

    pub fn add_style<S: Into<String>>(&mut self, href: S) {
        self.styles.push(href.into())
    }
}

impl Default for HeadContext {
    fn default() -> Self {
        HeadContext {
            title: "".to_string(),
            charset: DEFAULT_CHARSET,
            meta_tags: vec![],
            scripts: vec![],
            styles: vec![],
        }
    }
}

impl Render for HeadContext {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        self.clone().render_once_to(b)
    }
}

#[derive(Copy, Clone, Debug, Serialize)]
pub enum Charset {
    Utf8,
}

impl AsRef<str> for Charset {
    fn as_ref(&self) -> &str {
        match self {
            Charset::Utf8 => "utf-8",
        }
    }
}

impl Into<String> for Charset {
    fn into(self) -> String {
        self.as_ref().to_string()
    }
}

#[derive(Serialize, Debug, Clone)]
struct MetaTag {
    name: String,
    content: String,
}

impl MetaTag {
    fn new<S: Into<String>>(name: S, content: S) -> Self {
        MetaTag {
            name: name.into(),
            content: content.into(),
        }
    }
}

#[test]
fn renders_head() {
    let rendered = r#"<head>

    <title> some_title </title>
    <meta charset="utf-8">
    
    <meta name="author" content="test">
    

    
    <link rel="stylesheet" href="style.css">
    

    
    <script src="some.js"></script>
    

</head>"#
        .to_string();
    let mut head = HeadContext::default();
    head.title = "some_title".to_string();
    head.add_meta("author", "test");
    head.add_style("style.css");
    head.add_script("some.js");
    let out = head.render_once().unwrap();
    assert_eq!(out, rendered);
}

#[test]
fn renders_html_empty_body() {
    #[derive(Serialize, Debug, TemplateOnce, Clone, Default)]
    #[template(path = "test_body.stpl")]
    pub struct EmptyBody {}

    let html = HtmlContext::builder()
        .title("duże wow")
        .lang("pl")
        .body(EmptyBody {})
        .build()
        .unwrap();

    let out = html.render_once().unwrap();
    std::fs::write("head_out.html", &out).unwrap();
    let rendered = r#"<!doctype html>
<html lang="pl">

<head>

    <title> duże wow </title>
    <meta charset="utf-8">
    

    

    

</head>

<body>



</body>

</html>
"#;
    assert_eq!(out, rendered);
}
