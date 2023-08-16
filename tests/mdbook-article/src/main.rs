use std::io;

use mdbook::{renderer::RenderContext, BookItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ArticleConfig {
    pub title: Option<String>,
}

fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();
    // eprintln!("{:#?}", ctx.config);
    let article_cfg: Option<ArticleConfig> =
        ctx.config.get_deserialized_opt("output.article").unwrap();
    eprintln!("{:#?}", article_cfg);

    for item in ctx.book.iter() {
        match item {
            BookItem::Chapter(ch) => {
                eprintln!("Chapter: {ch:#?}");
            }
            BookItem::Separator => {
                eprintln!("sep");
            }
            BookItem::PartTitle(title) => {
                eprintln!("title={title}");
            }
        }
    }
}
