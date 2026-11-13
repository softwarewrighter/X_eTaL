//! Workspace commands, as APL's )FNS and )VARS: a line starting with
//! `)` (which no line of code can) lists the session's functions or
//! variables with their types, or shows one definition drawn with its
//! doc comment and type. Names are told apart by their decoration
//! (rule 4): an underlined name is a function. Commands are not code:
//! the session does not keep them.

use xetal_base::Diagnostic;
use xetal_lex::{TokenKind, lex};

use crate::Reply;

const COMMANDS: &str = ")fns, )vars, )def NAME";

/// Whether `line` is a workspace command.
pub(crate) fn is_command(line: &str) -> bool {
    line.trim_start().starts_with(')')
}

/// The reply to command `line` over the session's `accepted` source
/// (libraries found from `origin`): its output, or the error it reports.
pub(crate) fn reply(line: &str, accepted: &str, origin: &str) -> Reply {
    match command(line, accepted, origin) {
        Ok(out) => Reply::Done {
            out,
            err: String::new(),
        },
        Err(e) => Reply::Done {
            out: String::new(),
            err: format!("{e}\n"),
        },
    }
}

fn command(line: &str, accepted: &str, origin: &str) -> Result<String, Diagnostic> {
    let words: Vec<&str> = line.split_whitespace().collect();
    let listed = |functions: bool| -> Result<String, Diagnostic> {
        let names = typed(accepted, origin)?;
        let shown = names.iter().filter(|(n, _)| is_function(n) == functions);
        Ok(shown.map(|(n, t)| format!("{n} : {t}\n")).collect())
    };
    match words.as_slice() {
        [")fns"] => listed(true),
        [")vars"] => listed(false),
        [")def", name] => {
            let names = typed(accepted, origin)?;
            let ty = names.iter().find(|(n, _)| n == name).map(|(_, t)| t);
            let (Some(ty), Some(text)) = (ty, definition(accepted, name)) else {
                let msg = format!("`{name}` is not defined in this session");
                return Err(Diagnostic::new("unknown-name", msg));
            };
            let drawn = xetal_render::decorate(&text).unwrap_or(text);
            Ok(format!("{drawn}\n{name} : {ty}\n"))
        }
        _ => Err(Diagnostic::new(
            "unknown-command",
            format!("`{}` is not a workspace command: {COMMANDS}", line.trim()),
        )),
    }
}

/// The session's own names and their types, in the order first defined
/// (a name defined again keeps its place and takes its newest type).
fn typed(accepted: &str, origin: &str) -> Result<Vec<(String, String)>, Diagnostic> {
    xetal_core::interactive(true);
    let loaded = xetal_program::load(origin, accepted);
    xetal_core::interactive(false);
    let mut program = loaded?.program;
    let lines = xetal_types::check_program(&mut program)?;
    let mut out: Vec<(String, String)> = Vec::new();
    for (name, ty) in lines.iter().filter_map(|l| l.split_once(" : ")) {
        match out.iter_mut().find(|(n, _)| n == name) {
            Some(seen) => seen.1 = ty.to_string(),
            None => out.push((name.to_string(), ty.to_string())),
        }
    }
    Ok(out)
}

/// Whether `name` is written as a function (an underlined name).
fn is_function(name: &str) -> bool {
    matches!(lex(name).as_deref(), Ok([t]) if matches!(t.kind, TokenKind::Func(_)))
}

/// The text of `name`'s newest definition in `src` (a statement
/// `name := ...` to its end, over lines while a bracket is open), with
/// the doc comment lines right above it when it starts its line.
fn definition(src: &str, name: &str) -> Option<String> {
    let tokens = lex(src).ok()?;
    let (mut depth, mut start, mut found) = (0i32, true, None);
    for (i, t) in tokens.iter().enumerate() {
        let next = tokens.get(i + 1).map(|n| &n.kind);
        if depth == 0
            && start
            && &src[t.span.start..t.span.end] == name
            && next == Some(&TokenKind::Assign)
        {
            let end = tokens[i..]
                .iter()
                .scan(0i32, |d, u| {
                    *d += bracket(&u.kind);
                    Some((*d, u))
                })
                .find(|(d, u)| *d == 0 && matches!(u.kind, TokenKind::Newline | TokenKind::Semi))
                .map_or(src.len(), |(_, u)| u.span.start);
            found = Some((t.span.start, end));
        }
        depth += bracket(&t.kind);
        start = matches!(t.kind, TokenKind::Newline | TokenKind::Semi);
    }
    let (from, to) = found?;
    let line_start = src[..from].rfind('\n').map_or(0, |n| n + 1);
    let mut first = from;
    if src[line_start..from].trim().is_empty() {
        first = line_start;
        for line in src[..line_start].lines().rev() {
            if !line.trim_start().starts_with("##") {
                break;
            }
            first -= line.len() + 1;
        }
    }
    Some(src[first..to].trim_end().to_string())
}

/// How a token changes the bracket depth.
fn bracket(kind: &TokenKind) -> i32 {
    match kind {
        TokenKind::LParen | TokenKind::LBrace | TokenKind::LBracket => 1,
        TokenKind::RParen | TokenKind::RBrace | TokenKind::RBracket => -1,
        _ => 0,
    }
}
