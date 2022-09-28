use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, Indexed, Scope, View};

use crate::data::project::{get_projects, Project};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub projects: Vec<Project>,
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    view! { cx,
        h1 { (state.greeting.get()) }
        a(href = "about", id = "about-link") { "About!" }
        div(id="project-list") {
            Indexed(
                iterable = state.projects,
                view = |cx, item| view! { cx,
                    a(class="project", href = format!("project/{}", item.id)) {
                        img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", item.icon)) {}
                        div {
                            h2(class="title") { (item.name) }
                            p(class="desc") { (item.desc) }
                        }
                    }
                },
            )
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
        title { "Index Page | Perseus Example – Basic" }
        link(rel = "stylesheet", href = ".perseus/static/css/index.css")
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World 4.0!".to_string(),
        projects: get_projects(),
    })
}
