use std::collections::HashSet;

use crate::component::project::project_list_entry::project_list_entry_generator;
use crate::templates::{header::Header, navbar::Navbar};
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, Indexed, Scope, View};
use sycamore::reactive::create_signal;

use crate::data::project::get_projects;

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, _state: IndexPageStateRx<'a>) -> View<G> {
    let projects = create_signal(cx, get_projects());
    view! { cx,
        Header()
        Navbar()
        div(id="about") {
            h2(class="section-header") {
                "About Me"
            }
            p {
                "I am a programmer and game developer currently attending Worcester Polytechnic Institute pursuing a double major in Computer Science and in Interactive Media & Game Development."
            }
        }
        div(id="project-list") {
            h2(class="section-header") {
                "Projects"
            }
            div(class="tags-area") {

            }
            div(class="list") {
                Indexed(
                    iterable = projects,
                    view = project_list_entry_generator(create_signal(cx, HashSet::new())),
                )
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: IndexPageState) -> View<SsrNode> {
    view! { cx,
        meta(charset = "UTF-8")
        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
        title { "David Mahany - Portfolio" }
        link(rel = "stylesheet", href = ".perseus/static/css/index.css")
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState { greeting: "Hello World 4.0!".to_string() })
}
