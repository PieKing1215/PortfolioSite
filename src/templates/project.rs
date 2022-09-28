use perseus::{Html, RenderFnResult, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, Scope, View};

use crate::data::project::{get_projects, Project};

#[perseus::make_rx(ProjectPageStateRx)]
pub struct ProjectPageState {
    pub project: Project,
}

#[perseus::template_rx]
pub fn project_page<'a, G: Html>(cx: Scope<'a>, state: ProjectPageStateRx<'a>) -> View<G> {
    view! { cx,
        h1 { (state.project.get().name) }
        a(href = "", id = "home-link") { "Home!" }
        div(id="project-list") {
            div(class="project") {
                img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", state.project.get().icon)) {}
                div {
                    h2(class="title") { (state.project.get().name) }
                    p(class="desc") { (state.project.get().desc) }
                }
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("project")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .template(project_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: ProjectPageState) -> View<SsrNode> {
    view! { cx,
        meta(charset = "UTF-8")
        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
        title { "Index Page" }
        link(rel = "stylesheet", href = ".perseus/static/css/index.css")
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    path: String,
    _locale: String,
) -> RenderFnResultWithCause<ProjectPageState> {
    Ok(ProjectPageState {
        project: get_projects()
            .into_iter()
            .find(|p| format!("project/{}", p.id) == path)
            .expect(&path),
    })
}

#[perseus::build_paths]
pub async fn get_build_paths() -> RenderFnResult<Vec<String>> {
    Ok(get_projects().into_iter().map(|p| p.id).collect())
}
