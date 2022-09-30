use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(id="navbar") {
            p {
                "Navbar here"
            }
        }
    }
}
