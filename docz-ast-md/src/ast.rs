//! AST

use docz_ast::AstNode;

use crate::MdNode;

impl AstNode for MdNode {
    fn children(&self) -> Option<&Vec<Self>> {
        todo!("MdNode children")
    }

    fn children_mut(&mut self) -> Option<&mut Vec<Self>> {
        todo!("MdNode children_mut")
    }
}
