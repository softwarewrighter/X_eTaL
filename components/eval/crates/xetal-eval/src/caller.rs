//! The evaluator as the callback higher-order built-ins use: operands
//! run by the ordinary application rules (`components/hof`, B6).

use xetal_base::{Diagnostic, Span};
use xetal_value::{Caller, Slot, Value};

use crate::machine::Machine;

impl<'a> Caller<'a> for Machine<'a, '_> {
    fn call(&mut self, f: &Value<'a>, x: Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
        self.apply(f.clone(), Slot::Value(x), span)
    }
}
