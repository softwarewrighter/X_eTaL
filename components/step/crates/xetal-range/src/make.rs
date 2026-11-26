//! `r_ange n` (from 1, A5) and `o_ffsets n` (from 0, B5) as progressions.

use xetal_progression::Progression;
use xetal_value::Value;

pub(crate) fn call<'a>(name: &str, args: &[Value<'a>]) -> Option<Value<'a>> {
    let from = match name {
        "r_ange" => 1,
        "o_ffsets" => 0,
        _ => return None,
    };
    match args {
        [Value::Int(n)] => Progression::new(from, 1, usize::try_from(*n).ok()?).map(Value::Range),
        _ => None,
    }
}
