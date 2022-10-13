use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

use crate::templates::{header::Header, navbar::Navbar};

#[perseus::template_rx]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Header()
        Navbar()
        p { "About." }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        meta(charset = "UTF-8")
        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
        title { "David Mahany | About" }
        link(rel = "stylesheet", href = ".perseus/static/css/about.css")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
