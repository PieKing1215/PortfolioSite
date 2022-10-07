use std::collections::HashSet;

use crate::{
    data::project::Tag,
    templates::{header::Header, navbar::Navbar},
};
use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};

use sycamore::prelude::*;

use crate::data::project::get_projects;

#[perseus::make_rx(ProjectsPageStateRx)]
pub struct ProjectsPageState {
    pub greeting: String,
    pub filter_tags: HashSet<Tag>,
}

#[perseus::template_rx]
pub fn projects_page<'a, G: Html>(cx: Scope<'a>, state: ProjectsPageStateRx<'a>) -> View<G> {
    // let projects = create_signal(cx, get_projects().into_iter().filter(|p| state.filter_tags.get().is_empty() || p.tags.iter().any(|t| state.filter_tags.get().contains(t))).collect());

    let projects = state.filter_tags.map(cx, |ft| {
        get_projects()
            .into_iter()
            .filter(|p| ft.is_empty() || p.tags.iter().any(|t| ft.contains(t)))
            .collect()
    });

    let tags = create_signal(cx, vec![Tag::Modding]);
    let langs = create_signal(cx, vec![Tag::Rust, Tag::Java]);

    view! { cx,
        Header()
        Navbar()
        div(id="project-list") {
            h2(class="section-header") {
                "Projects"
            }
            div(class="tags-area") {
                "Filter by Tags:"
                div(class="tags-container") {
                    Indexed(
                        iterable = tags,
                        view = move |cx, item| view! { cx,
                            div(class = if state.filter_tags.get().contains(&item) { "tag active" } else { "tag" }, on:click = move |_| {
                                if state.filter_tags.get().contains(&item) {
                                    state.filter_tags.modify().remove(&item);
                                } else {
                                    state.filter_tags.modify().insert(item);
                                }
                            }) { (format!("{item:?}")) }
                        }
                    )
                    Indexed(
                        iterable = langs,
                        view = move |cx, item| view! { cx,
                            div(class = if state.filter_tags.get().contains(&item) { "tag active" } else { "tag" }, on:click = move |_| {
                                if state.filter_tags.get().contains(&item) {
                                    state.filter_tags.modify().remove(&item);
                                } else {
                                    state.filter_tags.modify().insert(item);
                                }
                            }) { (format!("{item:?}")) }
                        }
                    )
                }
            }
            div(class="list") {
                Indexed(
                    iterable = projects,
                    view = move |cx, item| view! { cx,
                        div(class="project") {
                            a(href = format!("project/{}", item.id))
                            img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", item.icon)) {}
                            div {
                                h2(class="title") { (item.name) }
                                div(class="desc") { ((item.short_desc)(cx)) }
                                div(class="tags") {
                                    Indexed(
                                        iterable = create_signal(cx, item.tags.into_iter().collect()),
                                        view = move |cx, item| view! { cx,
                                            div(class=if state.filter_tags.get().contains(&item) { "tag active" } else { "tag" }, on:click = move |_| {
                                                if state.filter_tags.get().contains(&item) {
                                                    state.filter_tags.modify().remove(&item);
                                                } else {
                                                    state.filter_tags.modify().insert(item);
                                                }
                                            }) {
                                                (format!("{item:?}"))
                                            }
                                        }
                                    )
                                }
                                // p(class="desc", dangerously_set_inner_html=&item.desc)
                            }
                        }
                    },
                )
            }
        }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("projects")
        .build_state_fn(get_build_state)
        .template(projects_page)
        .head(head)
}

#[perseus::head]
pub fn head(cx: Scope, _props: ProjectsPageState) -> View<SsrNode> {
    view! { cx,
        meta(charset = "UTF-8")
        meta(name = "viewport", content = "width=device-width, initial-scale=1.0")
        title { "David Mahany | Projects" }
        link(rel = "stylesheet", href = ".perseus/static/css/projects.css")
    }
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<ProjectsPageState> {
    Ok(ProjectsPageState {
        greeting: "Hello World 4.0!".to_string(),
        filter_tags: HashSet::from([]),
    })
}
