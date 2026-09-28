//! Lambdas (L1-L7, E1) and trains (TR1-TR4).

use xetal_base::{Diagnostic, Span};
use xetal_syntax::{Fun, Lambda, Params, Target};

use crate::Lower;
use crate::ir::{Expr, Kind, Param};

impl Lower {
    /// Curried one-parameter lambdas; `_l` / `_r` are ordinary names
    /// that the innermost lambda rebinds (L2).
    pub(crate) fn lambda(&mut self, l: &Lambda, span: Span) -> Result<Expr, Diagnostic> {
        let params: Vec<(Param, bool)> = match &l.params {
            Params::Right => vec![(Param::Name("_r".into()), false)],
            Params::LeftRight => vec![
                (Param::Name("_l".into()), false),
                (Param::Name("_r".into()), false),
            ],
            Params::Niladic => vec![(Param::Unit, false)],
            Params::Named(ps) => ps
                .iter()
                .map(|p| {
                    let name = match &p.name {
                        Target::Var(v) => format!("{}{}", v.name, if v.mutable { "!" } else { "" }),
                        Target::Func(f) => f.spelled(),
                    };
                    (Param::Name(name), p.lazy)
                })
                .collect(),
        };
        self.scopes.push(Default::default());
        self.lambdas += 1;
        for (param, _) in &params {
            if let Param::Name(name) = param {
                self.bind(name);
            }
        }
        let body = self.body(&l.body, span);
        self.lambdas -= 1;
        self.scopes.pop();
        let mut body = body?;
        for (param, lazy) in params.into_iter().rev() {
            body = self.node(
                span,
                Kind::Lam {
                    param,
                    lazy,
                    body: Box::new(body),
                },
            );
        }
        Ok(body)
    }

    /// A train as a value or in monadic position is monadic (TR4).
    pub(crate) fn train_value(&mut self, fs: &[Fun], span: Span) -> Result<Expr, Diagnostic> {
        let x = self.fresh_name();
        let arg = self.node(span, Kind::Var(x.clone()));
        let body = self.train_apply(fs, None, arg)?;
        Ok(self.node(
            span,
            Kind::Lam {
                param: Param::Name(x),
                lazy: false,
                body: Box::new(body),
            },
        ))
    }

    /// `x T y` with a train written in place: dyadic (TR1, TR4). The
    /// arguments are bound once, right first (E4).
    pub(crate) fn dyadic(
        &mut self,
        f: &Fun,
        left: Expr,
        right: Expr,
        span: Span,
    ) -> Result<Expr, Diagnostic> {
        let xetal_syntax::FunKind::Train(fs) = &f.kind else {
            return self.apply2(f, left, right);
        };
        let (l, r) = (self.fresh_name(), self.fresh_name());
        let (lv, rv) = (
            self.node(left.span, Kind::Var(l.clone())),
            self.node(right.span, Kind::Var(r.clone())),
        );
        let applied = self.train_apply(fs, Some(lv), rv)?;
        let inner = self.node(
            span,
            Kind::Let {
                name: l,
                rec: false,
                value: Box::new(left),
                body: Box::new(applied),
            },
        );
        Ok(self.node(
            span,
            Kind::Let {
                name: r,
                rec: false,
                value: Box::new(right),
                body: Box::new(inner),
            },
        ))
    }

    /// Apply train elements to argument variables: `[F G] x = F (G x)`,
    /// `[F G H] x = (F x) G (H x)`, dyadically `x [F G H] y =
    /// (x F y) G (x H y)`. Nested trains expand in place.
    pub(crate) fn train_apply(
        &mut self,
        fs: &[Fun],
        left: Option<Expr>,
        right: Expr,
    ) -> Result<Expr, Diagnostic> {
        let tine = |me: &mut Self, f: &Fun| -> Result<Expr, Diagnostic> {
            match &left {
                Some(l) => me.apply2(f, l.clone(), right.clone()),
                None => match &f.kind {
                    xetal_syntax::FunKind::Train(inner) => {
                        me.train_apply(inner, None, right.clone())
                    }
                    _ => me.apply1(f, right.clone()),
                },
            }
        };
        match fs {
            [f, g] => {
                let inner = tine(self, g)?;
                self.apply1(f, inner)
            }
            [f, g, h] => {
                let (fx, hx) = (tine(self, f)?, tine(self, h)?);
                self.apply2(g, fx, hx)
            }
            _ => unreachable!("the parser groups trains into two or three elements"),
        }
    }
}
