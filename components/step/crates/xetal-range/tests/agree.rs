//! Every kept operation gives what the same built-in gives on the
//! range's items (step 202: no semantic change), on small ranges.

use proptest::prelude::*;
use xetal_arith::Rng;
use xetal_base::Span;
use xetal_progression::Progression;
use xetal_range::{dense, dense_args};
use xetal_value::Value;

/// The built-in on the items, as `xetal-prim` computes it.
fn on_items(name: &str, args: &[Value<'static>]) -> String {
    let args = dense_args(args.to_vec(), Span::default()).expect("small");
    let mut out = Vec::new();
    let got = xetal_prim::call(name, &args, Span::default(), &mut out, &mut Rng::seeded(1));
    format!("{:?}", got.map(|v| dense(&v).expect("small").into_owned()))
}

/// The kept operation, when it does not decline.
fn kept(name: &str, args: &[Value<'static>]) -> Option<String> {
    let v = xetal_range::call(name, args)?;
    Some(format!(
        "{:?}",
        Ok::<_, ()>(dense(&v).expect("small").into_owned())
    ))
}

fn agree(name: &str, args: &[Value<'static>]) {
    if let Some(k) = kept(name, args) {
        assert_eq!(k, on_items(name, args), "{name} {args:?}");
    }
}

fn range(start: i64, step: i64, count: usize) -> Value<'static> {
    Value::Range(Progression::new(start, step, count).expect("fits"))
}

proptest! {
    #[test]
    fn kept_operations_agree(start in -5i64..6, step in -3i64..4, count in 0usize..8, n in -10i64..11) {
        let r = range(start, step, count);
        for name in ["t_ake", "d_rop", "s_elect", "m_ember?", "+", "-", "*"] {
            agree(name, &[Value::Int(n), r.clone()]);
        }
        for name in ["i_ndexOf", "+", "-", "*"] {
            agree(name, &[r.clone(), Value::Int(n)]);
        }
        for name in ["r_ev", "t_ally", "s_hape", "f_irst"] {
            agree(name, std::slice::from_ref(&r));
        }
        agree("s_elect", &[range(n.rem_euclid(4), 1, count.min(3)), r]);
    }

    #[test]
    fn ranges_are_their_items(n in 0i64..20) {
        agree("r_ange", &[Value::Int(n)]);
        agree("o_ffsets", &[Value::Int(n)]);
    }

    #[test]
    fn sums_are_exact(start in -50i64..50, step in -9i64..10, count in 0usize..40) {
        let p = Progression::new(start, step, count).expect("fits");
        let items: i64 = p.items().expect("small").iter().sum();
        prop_assert_eq!(p.sum(), Some(items));
    }
}

#[test]
fn a_range_past_max_items_keeps_its_end() {
    let r = xetal_range::call("r_ange", &[Value::Int(1_000_000_000)]).expect("kept");
    let last = xetal_range::call("t_ake", &[Value::Int(-2), r]).expect("kept");
    assert_eq!(
        dense(&last).expect("small").to_string(),
        "999999999 1000000000"
    );
}
