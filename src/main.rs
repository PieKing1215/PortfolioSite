mod component;
pub mod data;
mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::projects::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::project::get_template)
        .error_pages(crate::error_pages::get_error_pages)
}
