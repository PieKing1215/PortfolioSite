use perseus::{Html, RenderFnResult, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::*;

use crate::data::project::get_projects;

#[perseus::make_rx(ProjectPageStateRx)]
pub struct ProjectPageState {
    pub project_idx: usize,
}

#[perseus::template_rx]
pub fn project_page<'a, G: Html>(cx: Scope<'a>, state: ProjectPageStateRx<'a>) -> View<G> {
    let project = get_projects()[*state.project_idx.get()].clone();
    let project2 = project.clone();

    view! { cx,
        h1 { (project2.name) }
        a(href = "", id = "home-link") { "Home!" }
        div(id="project-list") {
            div(class="project") {
                img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", project.icon)) {}
                div {
                    h2(class="title") { (project.name) }
                    div { ((project.long_desc)(cx)) }
                    // p(class="desc") { (state.project.get().desc) }
                }
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("project")
        .build_paths_fn(get_build_paths::<G>)
        .build_state_fn(get_build_state::<G>)
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

#[cfg(target_arch = "wasm32")]
pub fn get_build_state<G: Html>() {}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_build_state<G: Html>(
    path: String,
    locale: String,
) -> perseus::RenderFnResultWithCause<String> {
    async fn get_build_state<G: Html>(
        path: String,
        _locale: String,
    ) -> RenderFnResultWithCause<ProjectPageState> {
        {
            Ok(ProjectPageState {
                project_idx: get_projects::<G>()
                    .into_iter()
                    .enumerate()
                    .find(|(_i, p)| format!("project/{}", p.id) == path)
                    .expect(&path)
                    .0,
            })
        }
    }
    let build_state = get_build_state::<G>(path, locale).await;
    build_state.map(|val| ::serde_json::to_string(&val).unwrap())
}

#[cfg(target_arch = "wasm32")]
pub fn get_build_paths<G: Html>() {}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_build_paths<G: Html>() -> RenderFnResult<Vec<String>> {
    {
        Ok(get_projects::<G>().into_iter().map(|p| p.id).collect())
    }
}
