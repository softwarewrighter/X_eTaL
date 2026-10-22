//! The list drawn under the text, and writing a candidate into it.

use web_sys::HtmlTextAreaElement;
use xetal_complete::Completion;
use yew::prelude::*;

use crate::hook::Open;

/// The list: each candidate's label and detail, the chosen one marked;
/// a press on a row writes it (before the text area loses the focus).
pub(crate) fn draw(open: &Open, area: &NodeRef, edit: &Callback<String>) -> Html {
    let Some((c, chosen)) = (**open).as_ref() else {
        return html! {};
    };
    let rows = c.candidates.iter().enumerate().map(|(i, k)| {
        let (open, area, edit, c) = (open.clone(), area.clone(), edit.clone(), c.clone());
        let pick = Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            accept(&area, &c, i, &edit);
            open.set(None);
        });
        html! {
            <li class={classes!((i == *chosen).then_some("chosen"))} onmousedown={pick}>
                <span class="label">{ &k.label }</span><span class="detail">{ &k.detail }</span>
            </li>
        }
    });
    html! { <ul class="suggest" role="listbox" aria-label="completions">{ for rows }</ul> }
}

/// Write candidate `i` over the word in the text area, put the cursor
/// after it, and give the editor the text.
pub(crate) fn accept(area: &NodeRef, c: &Completion, i: usize, edit: &Callback<String>) {
    let Some(area) = area.cast::<HtmlTextAreaElement>() else {
        return;
    };
    let text = area.value();
    let (Some(head), Some(tail)) = (text.get(..c.start), text.get(c.end..)) else {
        return;
    };
    let insert = &c.candidates[i].insert;
    let after = format!("{head}{insert}{tail}");
    area.set_value(&after);
    let at = after[..c.start + insert.len()].encode_utf16().count() as u32;
    let _ = area.set_selection_range(at, at);
    edit.emit(after);
}

/// The byte offset in `text` of the cursor at `utf16` code units (what
/// a text area counts in); the end when past it.
pub(crate) fn byte_offset(text: &str, utf16: usize) -> usize {
    let mut units = 0;
    for (i, ch) in text.char_indices() {
        if units >= utf16 {
            return i;
        }
        units += ch.len_utf16();
    }
    text.len()
}
