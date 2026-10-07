//! Strand items: a number stays as it is; a string in a strand is
//! enclosed, so a strand of strings is a nested vector (B14). Tuple
//! parts are lowered as they are (TU1).

use xetal_base::Diagnostic;
use xetal_syntax::{Expr as Surface, ExprKind};

use crate::lower::Lower;
use xetal_ir::{Expr, Kind};

impl Lower {
    /// The items of a strand (`strand`) or the parts of a tuple, lowered.
    pub(crate) fn parts(
        &mut self,
        items: &[Surface],
        strand: bool,
    ) -> Result<Vec<Expr>, Diagnostic> {
        items
            .iter()
            .map(|x| {
                if strand {
                    self.strand_item(x)
                } else {
                    self.expr(x)
                }
            })
            .collect()
    }

    pub(crate) fn strand_item(&mut self, e: &Surface) -> Result<Expr, Diagnostic> {
        let item = self.expr(e)?;
        if !matches!(e.kind, ExprKind::Str(_)) {
            return Ok(item);
        }
        let f = self.node(e.span, Kind::Prim("e_nclose".into()));
        Ok(self.node(e.span, Kind::App(Box::new(f), Box::new(item))))
    }
}
