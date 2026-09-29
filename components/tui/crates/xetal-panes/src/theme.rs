//! Colors by token class (the same palette as `xetal render --color`).

use ratatui::style::{Color, Modifier, Style};
use xetal_view::Class;

pub fn style(class: Class) -> Style {
    let fg = |c: Color| Style::default().fg(c);
    match class {
        Class::Builtin => fg(Color::Blue),
        Class::UserFunc => fg(Color::Green),
        Class::LibFunc => fg(Color::Cyan),
        Class::LambdaArg => fg(Color::Magenta),
        Class::Number | Class::Exponent => fg(Color::Yellow),
        Class::String => fg(Color::LightYellow),
        Class::Symbol | Class::Quote => Style::default().add_modifier(Modifier::BOLD),
        Class::Comment => Style::default().add_modifier(Modifier::DIM),
        Class::Error => fg(Color::Red).add_modifier(Modifier::UNDERLINED),
        Class::Variable | Class::Punct | Class::Unit | Class::Space => Style::default(),
    }
}
