//! Typing.

use crate::syntax::*;
use crate::error::*;

impl Expr {
    /// Solve a type of the expression.
    pub fn solve(&self) -> Result<TypedExpr, TypingError> {
        Ok(TypedExpr::TELeaf(Type::TyCon("Foo".to_string()), self.clone()))
    }
}
