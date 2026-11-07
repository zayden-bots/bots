use htmd::element_handler::Handlers;
use htmd::options::{CodeBlockStyle, HeadingStyle, Options};
use htmd::{Element, HtmlToMarkdown};

use crate::wiki::WikiError;

pub fn html_to_markdown(html: &str) -> Result<String, WikiError> {
    let options = Options {
        heading_style: HeadingStyle::Atx,
        code_block_style: CodeBlockStyle::Fenced,
        ..Default::default()
    };

    HtmlToMarkdown::builder()
        .options(options)
        .skip_tags(vec!["script", "style"])
        .add_handler(
            vec!["block-tab"],
            |handlers: &dyn Handlers, element: Element<'_>| {
                let label = element
                    .attrs
                    .iter()
                    .find(|attr| attr.name.local.as_ref() == "label")
                    .map(|attr| attr.value.to_string())
                    .filter(|label| !label.trim().is_empty());

                let content = handlers.walk_children(element.node).content;
                let children = content.trim();

                let text = label.map_or_else(
                    || children.to_owned(),
                    |label| format!("\n\n**{label}**\n\n{children}\n\n"),
                );

                Some(text.into())
            },
        )
        .build()
        .convert(html)
        .map_err(WikiError::Html)
}
