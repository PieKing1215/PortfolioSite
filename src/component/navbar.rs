use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(id="navbar") {
            a(href="") {
                "Home"
            }
            a(href="projects") {
                "Projects"
            }
            a(href="about") {
                "About"
            }
        }
    }
}
