//! Generic AST for docz

#[cfg(test)]
mod tests;

/// AST node
pub trait AstNode
where
    Self: Sized,
{
    /// Returns a reference to the children
    fn children(&self) -> Option<&Vec<Self>>;

    /// Returns a mutable reference to the children
    fn children_mut(&mut self) -> Option<&mut Vec<Self>>;

    /// Visits a node recursively
    fn visit(&self, f: &mut impl FnMut(&Self)) {
        f(self);
        if let Some(children) = self.children() {
            for child in children {
                child.visit(f);
            }
        }
    }

    /// Visits a node recursively and mutably
    fn visit_mut(&mut self, f: &mut impl FnMut(&mut Self)) {
        f(self);
        if let Some(children) = self.children_mut() {
            for child in children {
                child.visit_mut(f);
            }
        }
    }
}
