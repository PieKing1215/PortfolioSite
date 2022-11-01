use std::{cmp::Ordering, collections::HashSet, sync::Arc};

use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use super::mc_mods::invmove_download_count;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Clone)]
pub struct Project<G: Html> {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub date: Option<(String, Option<String>)>,
    pub short_desc: Arc<dyn Fn(Scope) -> View<G>>,
    pub long_desc: Arc<dyn Fn(Scope) -> View<G>>,
    pub tags: HashSet<Tag>, // could use linked_hash_set to preserve order
}

impl<G: Html> Project<G> {
    pub fn format_date(&self) -> String {
        match &self.date {
            Some((start, Some(end))) => format!("({start}\u{2013}{end})"),
            Some((start, None)) => format!("({start}\u{2013})"),
            None => String::new(),
        }
    }
}

impl<G: Html> PartialEq for Project<G> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.icon == other.icon
    }
}

impl<G: Html> std::fmt::Debug for Project<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Project")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("icon", &self.icon)
            .field("date", &self.date)
            .field("tags", &self.tags)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Tag {
    Web,
    Modding,
    GameDev,
    Unity,
    Minecraft,
    pxtone,

    Rust,
    Java,
    JavaScript,
    HTML,
    CSS,
    CSharp,
}

trait StringIntoViewFn<G: Html> {
    fn into_view_fn(self) -> Arc<dyn Fn(Scope) -> View<G>>;
}

impl<G: Html> StringIntoViewFn<G> for &'static str {
    fn into_view_fn(self) -> Arc<dyn Fn(Scope) -> View<G>> {
        Arc::new(move |cx| {
            view! { cx,
                p(dangerously_set_inner_html = self) {}
            }
        })
    }
}

pub fn get_projects<G: Html>() -> Vec<Project<G>> {
    let mut ps = vec![
        Project {
            id: "invmove".into(),
            name: "InvMove".into(),
            icon: Some("invmove".into()),
            date: Some(("2019".into(), None)),
            tags: HashSet::from([Tag::Modding, Tag::Minecraft, Tag::Java]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source Minecraft Forge/Fabric/Quilt mod that adds the ability to walk around while in inventories." }
                    p { ((invmove_download_count() / 10000 * 10000).to_formatted_string(&Locale::en)) "+ Downloads" }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source Minecraft Forge/Fabric/Quilt mod that adds the ability to walk around while in inventories." }
                    p { ((invmove_download_count() / 10000 * 10000).to_formatted_string(&Locale::en)) "+ Downloads" }
                }
            }),
        },
        Project {
            id: "dripsounds".into(),
            name: "Drip Sounds".into(),
            icon: Some("dripsounds".into()),
            date: Some(("2020".into(), None)),
            tags: HashSet::from([Tag::Modding, Tag::Minecraft, Tag::Java]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source Minecraft Forge/Fabric mod that adds sounds for drip particles landing." }
                    p { "780,000+ Downloads" }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source Minecraft Forge/Fabric mod that adds sounds for drip particles landing." }
                    p { "780,000+ Downloads" }
                }
            }),
        },
        Project {
            id: "ptcMod".into(),
            name: "ptcMod".into(),
            icon: Some("ptcMod".into()),
            date: Some(("2021".into(), None)),
            tags: HashSet::from([Tag::Modding, Tag::pxtone, Tag::Rust]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source mod for " a(href="https://pxtone.org/downloads/") { "pxtone Collage" } " that adds more features." }
                    p { "Uses low level memory tricks, DLL injection, and ASM injection to mod the program in-memory." }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source mod for " a(href="https://pxtone.org/downloads/") { "pxtone Collage" } " that adds more features." }
                    p { "Uses low level memory tricks, DLL injection, and ASM injection to mod the program in-memory." }
                }
            }),
        },
        Project {
            id: "rust_pxtone".into(),
            name: "rust-pxtone".into(),
            icon: None,
            date: Some(("2021".into(), None)),
            tags: HashSet::from([Tag::pxtone, Tag::Rust]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source low and high level bindings to " a(href="https://pxtone.org/") { "pxtone" } " for Rust" }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "Open source low and high level bindings to " a(href="https://pxtone.org/") { "pxtone" } " for Rust" }
                }
            }),
        },
        Project {
            id: "quantum_games".into(),
            name: "Quantum Games (IQP)".into(),
            icon: Some("quantum_games".into()),
            date: Some(("2021".into(), Some("2022".into()))),
            tags: HashSet::from([Tag::Web, Tag::JavaScript, Tag::HTML, Tag::CSS]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "A website hosting small educational games based on our advisor's prior research in quantum mechanics." }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "A website hosting small educational games based on our advisor's prior research in quantum mechanics." }
                }
            }),
        },
        Project {
            id: "marching_sim".into(),
            name: "Marching Band Simulator (MQP)".into(),
            icon: None,
            date: Some(("2022".into(), None)),
            tags: HashSet::from([Tag::GameDev, Tag::Unity, Tag::CSharp]),
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p { "An in-development casual game about running a marching band, with a focus on procedural generation." }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p { "An in-development casual game about running a marching band, with a focus on procedural generation." }
                }
            }),
        },
        Project {
            id: "test1".into(),
            name: "Test 1".into(),
            icon: Some("test".into()),
            date: None,
            short_desc: Arc::new(|cx| {
                view! { cx,
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim ..."
                    }
                }
            }),
            long_desc: Arc::new(|cx| {
                view! { cx,
                    p {
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    }
                }
            }),
            tags: HashSet::from([Tag::Modding, Tag::Rust, Tag::Java]),
        },
        Project {
            id: "test2".into(),
            name: "Test 2".into(),
            icon: Some("test2".into()),
            date: None,
            short_desc: "Description for test 2<br>here".into_view_fn(),
            long_desc: "Description for test 2<br>here".into_view_fn(),
            tags: HashSet::from([]),
        },
        Project {
            id: "test3".into(),
            name: "Test 3".into(),
            icon: None,
            date: None,
            short_desc: "Description for test 3<br>here".into_view_fn(),
            long_desc: "Description for test 3<br>here".into_view_fn(),
            tags: HashSet::from([]),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            icon: None,
            date: None,
            short_desc: "Description for test 4<br>here".into_view_fn(),
            long_desc: "Description for test 4<br>here".into_view_fn(),
            tags: HashSet::from([]),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            icon: None,
            date: None,
            short_desc: "Description for test 4<br>here".into_view_fn(),
            long_desc: "Description for test 4<br>here".into_view_fn(),
            tags: HashSet::from([]),
        },
        Project {
            id: "test4".into(),
            name: "Test 4".into(),
            icon: None,
            date: None,
            short_desc: "Description for test 4<br>here".into_view_fn(),
            long_desc: "Description for test 4<br>here".into_view_fn(),
            tags: HashSet::from([]),
        },
    ];
    ps.sort_by(|a, b| match a.date.as_ref() {
        Some((start, None)) => match b.date.as_ref() {
            Some((b_start, None)) => {
                if let (Ok(sa), Ok(sb)) = (start.parse::<u16>(), b_start.parse::<u16>()) {
                    sb.cmp(&sa)
                } else {
                    Ordering::Equal
                }
            },
            Some((b_start, Some(b_end))) => Ordering::Less,
            None => Ordering::Less,
        },
        Some((start, Some(end))) => match b.date.as_ref() {
            Some((b_start, None)) => Ordering::Greater,
            Some((b_start, Some(b_end))) => Ordering::Equal,
            None => Ordering::Less,
        },
        None => {
            if b.date.is_some() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        },
    });
    ps
}
