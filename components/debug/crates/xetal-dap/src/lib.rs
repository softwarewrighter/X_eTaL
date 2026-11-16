//! `xetal dap`: the Debug Adapter Protocol (Saga 43, step debug-dap;
//! docs/debugging.md) over the debugger's core, on standard input and
//! output, so VS Code, Emacs (dape, dap-mode) and Neovim (nvim-dap)
//! step, set breakpoints, read the stack, the variables and watches,
//! and travel back (`stepBack`, `reverseContinue`).

mod inspect;
mod launch;
mod motion;
mod server;
mod wire;

pub use server::Server;
pub use wire::{Framing, serve};
