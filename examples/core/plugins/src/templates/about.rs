use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

// This page will actually be replaced entirely by a plugin!
#[perseus::template]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        p { "About." }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page | Perseus Example – Plugins" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
