//! The footer, as the other live demos show it: copyright, license,
//! the repository, the literate documents, and where and when this
//! build was made (from build.rs).

use yew::prelude::*;

pub(crate) const REPOSITORY: &str = "https://github.com/softwarewrighter/X_eTaL";

fn sep() -> Html {
    html! { <span class="sep">{ "\u{00b7}" }</span> }
}

pub fn footer() -> Html {
    html! {
        <footer>
            <span>{ "Copyright (c) 2026 Michael A Wright" }</span>{ sep() }
            <span>{ "MIT License" }</span>{ sep() }
            <a href={REPOSITORY} target="_blank">{ "Repository" }</a>{ sep() }
            <a href="literate/index.html" target="_blank">{ "Literate docs" }</a>{ sep() }
            <span>{ format!("Build Host {}", env!("BUILD_HOST")) }</span>{ sep() }
            <span>{ format!("Build Commit {}", env!("BUILD_SHA")) }</span>{ sep() }
            <span>{ format!("Build Time {}", env!("BUILD_TIMESTAMP")) }</span>
        </footer>
    }
}
