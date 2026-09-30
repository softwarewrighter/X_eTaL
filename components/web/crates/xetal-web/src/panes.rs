//! The three panes: the ASCII source (a text area), the source drawn
//! decorated, and the types or, after a run, the output.

use web_sys::{Element, HtmlTextAreaElement};
use xetal_play::{Class, Run, check, decorate};
use yew::prelude::*;

use crate::app::Pane;

/// A pane's frame: the current one is marked (a bright border and a
/// triangle before its title).
fn frame(title: &str, pane: Pane, current: Pane, focus: Callback<Pane>, body: Html) -> Html {
    let here = pane == current;
    let class = classes!("pane", pane.css(), here.then_some("current"));
    let heading = if here {
        format!("\u{25b6} {title}")
    } else {
        title.to_string()
    };
    html! {
        <section class={class} onfocusin={focus.reform(move |_: FocusEvent| pane)}>
            <h2>{ heading }</h2>
            { body }
        </section>
    }
}

/// The ASCII pane; as it scrolls (by hand, or as the browser follows
/// the cursor), the drawn pane scrolls to the same line.
pub(crate) fn source(
    text: &str,
    current: Pane,
    focus: Callback<Pane>,
    edit: Callback<String>,
    drawn: NodeRef,
) -> Html {
    let input = Callback::from(move |e: InputEvent| {
        let area: HtmlTextAreaElement = e.target_unchecked_into();
        edit.emit(area.value());
    });
    let follow = Callback::from(move |e: Event| {
        let area: HtmlTextAreaElement = e.target_unchecked_into();
        if let Some(pre) = drawn.cast::<Element>() {
            pre.set_scroll_top(area.scroll_top());
        }
    });
    let body = html! {
        <textarea value={text.to_string()} oninput={input} onscroll={follow}
            spellcheck="false" autocomplete="off" aria-label="source, as typed"/>
    };
    frame("ASCII", Pane::Source, current, focus, body)
}

fn class_name(class: Class) -> String {
    format!("c-{class:?}").to_lowercase()
}

pub(crate) fn rendered(text: &str, current: Pane, focus: Callback<Pane>, drawn: NodeRef) -> Html {
    let spans = decorate(text).into_iter().map(|s| {
        html! { <span class={class_name(s.class)}>{ s.text }</span> }
    });
    let body = html! { <pre tabindex="0" ref={drawn}>{ for spans }</pre> };
    frame("Rendered", Pane::Rendered, current, focus, body)
}

/// The types, or after a run its output (scrolled to the end by the
/// app, so the newest output shows).
pub(crate) fn output(
    text: &str,
    result: &Option<Run>,
    current: Pane,
    focus: Callback<Pane>,
    printed: NodeRef,
) -> Html {
    let (title, body) = match result {
        Some(r) => (
            "Output",
            html! {
                <pre tabindex="0" ref={printed.clone()}>{ &r.out }<span class="c-error">{ &r.err }</span></pre>
            },
        ),
        None => (
            "Types",
            html! { <pre tabindex="0">{ check(text).join("\n") }</pre> },
        ),
    };
    frame(title, Pane::Output, current, focus, body)
}
