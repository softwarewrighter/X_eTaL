//! The footer, as the other live demos show it: copyright, license,
//! the repository, the literate documents, the poster, the Rosetta
//! stone, and where and when this build was made (from build.rs). A
//! page below the site's root (the stone's, at rosetta/) gives the
//! prefix that reaches the root.

use yew::prelude::*;

pub(crate) const REPOSITORY: &str = "https://github.com/softwarewrighter/X_eTaL";

fn sep() -> Html {
    html! { <span class="sep">{ "\u{00b7}" }</span> }
}

/// The footer of a page at the site's root (the live demo).
pub fn footer() -> Html {
    footer_at("")
}

/// The footer of a page `prefix` below the site's root (`"../"` for a
/// page one directory down), its links reaching the root through it.
pub fn footer_at(prefix: &str) -> Html {
    let at = |path: &str| format!("{prefix}{path}");
    html! {
        <footer>
            <span>{ "Copyright (c) 2026 Michael A Wright" }</span>{ sep() }
            <span>{ "MIT License" }</span>{ sep() }
            <a href={REPOSITORY} target="_blank">{ "Repository" }</a>{ sep() }
            <a href={at("index.html")}>{ "Live editor" }</a>{ sep() }
            <a href={at("rosetta/index.html")}>{ "Rosetta stone" }</a>{ sep() }
            <a href={at("literate/index.html")} target="_blank">{ "Literate docs" }</a>{ sep() }
            <a href={at("poster/index.html")} target="_blank">{ "Syntax poster" }</a>{ sep() }
            <span title="Built on this host, from this commit, at this time">{ format!(
                "Build ({} {} {})",
                env!("BUILD_HOST"),
                env!("BUILD_SHA"),
                env!("BUILD_TIMESTAMP")
            ) }</span>
        </footer>
    }
}
