//! JSON

use serde::Serialize;

use crate::{Error, Node, NodeData, Span};

#[derive(Debug, Serialize)]
pub struct JsonNode<'a, T>
where
    T: NodeData,
{
    pub data: &'a T,
    pub span: Option<&'a Span>,
    pub children: Vec<JsonNode<'a, T>>,
}

impl<T> Node<T>
where
    T: NodeData + Serialize,
{
    /// Returns the node as JSON
    pub fn to_json(&self, pretty: bool) -> Result<String, Error> {
        let json_node = self.as_json_node()?;
        if pretty {
            Ok(serde_json::to_string_pretty(self)?)
        } else {
            Ok(serde_json::to_string(&json_node)?)
        }
    }

    /// Converts an AST node to a JSON node
    pub fn as_json_node(&self) -> Result<JsonNode<'_, T>, Error> {
        let data = &self.data;
        let span = self.span.as_ref();
        let mut children = vec![];
        for child in &self.children {
            let child_json = child.as_json_node()?;
            children.push(child_json);
        }

        Ok(JsonNode {
            data,
            span,
            children,
        })
    }
}
