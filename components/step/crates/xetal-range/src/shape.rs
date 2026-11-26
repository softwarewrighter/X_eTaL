//! Take, drop, reverse, count, shape and first of a progression.

use std::rc::Rc;

use xetal_array::Array;
use xetal_value::Value;

pub(crate) fn call<'a>(name: &str, args: &[Value<'a>]) -> Option<Value<'a>> {
    match (name, args) {
        ("t_ake", [Value::Int(n), Value::Range(p)]) => p.take(*n).map(Value::Range),
        ("d_rop", [Value::Int(n), Value::Range(p)]) => p.drop(*n).map(Value::Range),
        ("r_ev", [Value::Range(p)]) => p.reversed().map(Value::Range),
        ("t_ally", [Value::Range(p)]) => i64::try_from(p.count()).ok().map(Value::Int),
        ("s_hape", [Value::Range(p)]) => {
            let n = Value::Int(i64::try_from(p.count()).ok()?);
            Some(Value::Array(Rc::new(Array::vector(vec![n]))))
        }
        ("f_irst", [Value::Range(p)]) => p.at(0).map(Value::Int),
        _ => None,
    }
}
