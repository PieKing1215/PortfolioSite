use std::collections::HashSet;

use sycamore::prelude::*;

use crate::data::project::{Project, Tag};

pub fn project_list_entry_generator<'a, G: Html>(
    filter_tags: &'a Signal<HashSet<Tag>>,
) -> Box<dyn Fn(BoundedScope<'_, 'a>, Project<G>) -> View<G> + 'a> {
    Box::new(move |cx, item| {
        let date = item.format_date();
        let name_len = item.name.len();
        let name_size = if name_len >= 20 {
            "size3"
        } else if name_len >= 12 {
            "size2"
        } else {
            "size1"
        };

        let icon = if item.icon.is_some() {
            let icon_clone = item.icon.clone().unwrap();
            view! { cx,
                img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", icon_clone)) {}
            }
        } else {
            view! { cx, }
        };

        view! { cx,
            div(class="project") {
                a(href = format!("project/{}", item.id))
                (icon)
                div {
                    div(class="title") {
                        h2(class=format!("name {name_size}")) { (item.name) }
                        h3(class="date") { (date) }
                    }
                    div(class="desc") { ((item.short_desc)(cx)) }
                    div(class="tags") {
                        Indexed(
                            iterable = create_signal(cx, item.tags.into_iter().collect()),
                            view = move |cx, item| view! { cx,
                                div(class=if filter_tags.get().contains(&item) { "tag active" } else { "tag" }, on:click = move |_| {
                                    if filter_tags.get().contains(&item) {
                                        filter_tags.modify().remove(&item);
                                    } else {
                                        filter_tags.modify().insert(item);
                                    }
                                }) {
                                    (format!("{item:?}"))
                                }
                            }
                        )
                    }
                }
            }
        }
    })
}

/*
#[component]
pub fn ProjectListEntry<'a, 'b, G: Html>(
    cx: Scope<'a>,
    props: ProjectListEntryProps<'b, G>,
) -> View<G> {
    let ProjectListEntryProps { project: item, state } = props;

    let item = item.clone();
    let state = state.clone();

    let date = item.format_date();
    let name_len = item.name.len();
    let name_size = if name_len >= 20 { "size3" } else if name_len >= 12 { "size2" } else { "size1" };

    let icon = if item.icon.is_some() {
        let icon_clone = item.icon.clone().unwrap();
        view!{ cx,
            img(class="icon", src=format!(".perseus/static/assets/project_icon/{}.png", icon_clone)) {}
        }
    } else {
        view!{ cx, }
    };


    view! { cx,
        div(class="project") {
            a(href = format!("project/{}", item.id))
            (icon)
            div {
                div(class="title") {
                    h2(class=format!("name {name_size}")) { (item.name) }
                    h3(class="date") { (date) }
                }
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
            }
        }
    }
}
 */
