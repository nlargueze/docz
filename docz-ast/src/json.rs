//! JSON

use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use crate::{Error, Node, NodeKind, Span};

#[derive(Debug, Serialize)]
struct JsonNode {
    kind: String,
    attrs: HashMap<String, Value>,
    #[serde(skip)]
    #[allow(dead_code)]
    span: Option<Span>,
    value: Option<String>,
    children: Option<Vec<JsonNode>>,
}

impl Node {
    /// Returns the node as JSON
    pub fn to_json(&self, pretty: bool) -> Result<String, Error> {
        let json_node = self.as_json_node()?;
        if pretty {
            Ok(serde_json::to_string_pretty(&json_node)?)
        } else {
            Ok(serde_json::to_string(&json_node)?)
        }
    }

    /// Converts an AST node to a JSON node
    fn as_json_node(&self) -> Result<JsonNode, Error> {
        let mut attrs = HashMap::new();
        for (k, v) in &self.attrs {
            let value = serde_json::to_value(v)?;
            attrs.insert(k.to_string(), value);
        }

        let kind = match &self.kind {
            NodeKind::Document => "document".to_string(),
            NodeKind::Fragment => "fragment".to_string(),
            NodeKind::FrontMatter => "frontmatter".to_string(),
            NodeKind::Chapter => "chapter".to_string(),
            NodeKind::Section => "section".to_string(),
            NodeKind::Heading { level, id: _ } => format!("heading_{level}"),
            NodeKind::Paragraph => "paragraph".to_string(),
            NodeKind::Text => "text".to_string(),
            NodeKind::Comment => "comment".to_string(),
            NodeKind::ThematicBreak => "thematic_break".to_string(),
            NodeKind::LineBreak => "line_break".to_string(),
            NodeKind::SoftBreak => "soft_break".to_string(),
            NodeKind::Italic => "italic".to_string(),
            NodeKind::Bold => "bold".to_string(),
            NodeKind::BlockQuote => "blockquote".to_string(),
            NodeKind::List { ordered } => {
                let ordered = serde_json::to_value(ordered)?;
                attrs.insert("ordered".to_string(), ordered);
                "list".to_string()
            }
            NodeKind::ListItem { index: _ } => "list_item".to_string(),
            NodeKind::Code => "code".to_string(),
            NodeKind::Link { url, title } => {
                let url = serde_json::to_value(url)?;
                let title = serde_json::to_value(title)?;
                attrs.insert("url".to_string(), url);
                attrs.insert("title".to_string(), title);
                "link".to_string()
            }
            NodeKind::Image { url, title } => {
                let url = serde_json::to_value(url)?;
                let title = serde_json::to_value(title)?;
                attrs.insert("url".to_string(), url);
                attrs.insert("title".to_string(), title);
                "image".to_string()
            }
            NodeKind::Html => "html".to_string(),
            NodeKind::Table => "table".to_string(),
            NodeKind::TableRow { is_header: _ } => "table_row".to_string(),
            NodeKind::TableCell => "table_cell".to_string(),
            NodeKind::CodeBlock { info } => {
                let info = serde_json::to_value(info)?;
                attrs.insert("info".to_string(), info);
                "code_block".to_string()
            }
            NodeKind::FootnoteRef { id } => {
                let id = serde_json::to_value(id)?;
                attrs.insert("id".to_string(), id);
                "footnote_ref".to_string()
            }
            NodeKind::FootnoteDef { id } => {
                let id = serde_json::to_value(id)?;
                attrs.insert("id".to_string(), id);
                "footnote_def".to_string()
            }
            NodeKind::DefinitionList => "definition_list".to_string(),
            NodeKind::DefinitionItem => "definition_item".to_string(),
            NodeKind::DefinitionTerm => "definition_term".to_string(),
            NodeKind::DefinitionDetails => "definition_details".to_string(),
            NodeKind::StrikeThrough => "strike_through".to_string(),
            NodeKind::TaskItem { checked } => {
                let checked = serde_json::to_value(checked)?;
                attrs.insert("checked".to_string(), checked);
                "task_item".to_string()
            }
            NodeKind::Highlight => "highlight".to_string(),
            NodeKind::SubScript => "sub_script".to_string(),
            NodeKind::SuperScript => "super_script".to_string(),
            NodeKind::Other { name } => format!("other({name})"),
        };

        let children = if let Some(children) = &self.children {
            let mut json_children = Vec::new();
            for child in children {
                let json_child = child.as_json_node()?;
                json_children.push(json_child);
            }
            Some(json_children)
        } else {
            None
        };

        Ok(JsonNode {
            kind,
            attrs,
            span: self.span.clone(),
            children,
            value: self.value.clone(),
        })
    }
}
