use std::collections::HashSet;

use crate::{
    component::project::project_list_entry::project_list_entry_generator,
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
            .filter(|p| ft.iter().all(|t| p.tags.contains(t)))
            .collect()
    });

    let tags = create_signal(
        cx,
        vec![
            Tag::Web,
            Tag::Modding,
            Tag::GameDev,
            Tag::Unity,
            Tag::Minecraft,
            Tag::pxtone,
        ],
    );
    let langs = create_signal(
        cx,
        vec![
            Tag::Rust,
            Tag::Java,
            Tag::JavaScript,
            Tag::HTML,
            Tag::CSS,
            Tag::CSharp,
        ],
    );

    fn tag_row<'a, G: Html>(
        cx: BoundedScope<'_, 'a>,
        list: &'a Signal<Vec<Tag>>,
        filter_tags: &'a Signal<HashSet<Tag>>,
    ) -> View<G> {
        view! { cx,
            div(class="tags-container") {
                Indexed(
                    iterable = list,
                    view = move |cx, item| view! { cx,
                        div(class = if filter_tags.get().contains(&item) { "tag active" } else { "tag" }, on:click = move |_| {
                            if filter_tags.get().contains(&item) {
                                filter_tags.modify().remove(&item);
                            } else {
                                filter_tags.modify().insert(item);
                            }
                        }) { (format!("{item:?}")) }
                    }
                )
            }
        }
    }

    view! { cx,
        Header()
        Navbar()
        div(id="project-list") {
            h2(class="section-header") {
                "Projects"
            }
            div(class="tags-area") {
                h3 {
                    "Filter by Tags:"
                }
                (tag_row(cx, tags, state.filter_tags))
                (tag_row(cx, langs, state.filter_tags))
            }
            div(class="list") {
                Indexed(
                    iterable = projects,
                    view = project_list_entry_generator(state.filter_tags),
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
